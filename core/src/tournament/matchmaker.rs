use core::{cmp::Ordering, ops::AddAssign};
use std::collections::{BinaryHeap, HashMap};

use itertools::Itertools;
use ordered_float::NotNan;

use crate::{
    analytics::{aggregate::AggregateStats, winloss::MatchPerformance},
    config::matchmaker::MatchmakerConfig,
    error::TournamentError,
    game::{POD_SIZE, matchup::Matchup},
    player::PlayerId,
    tournament::Tournament,
};

#[derive(Debug, Clone, getset::WithSetters)]
pub struct Matchmaker<'a> {
    tourn: &'a Tournament,
    #[set_with]
    players: im::HashSet<PlayerId>,
    #[set_with]
    config: MatchmakerConfig,
}

impl Tournament {
    #[must_use]
    pub fn matchmaker(&self) -> Matchmaker<'_> {
        Matchmaker {
            tourn: self,
            players: self
                .players
                .iter()
                .filter_map(|(id, info)| (!info.is_archived()).then_some(*id))
                .collect(),
            config: self.config.matchmaker().clone(),
        }
    }
}

impl Matchmaker<'_> {
    pub fn create_match(&self, player: PlayerId) -> Result<Matchup, TournamentError> {
        log::debug!("Creating Match for Player {player}");
        let pool = self.create_player_pool(player);
        let analytics = self.tourn.analytics();

        let mut aggregate_stats = AggregateStats::from(self.tourn.get_player_or_default_stats(player));
        log::debug!("Built Aggregate Stats: {aggregate_stats:?}");
        let mut performances = analytics.player_performance_all_others(player)?;
        performances.retain(|id, _| pool.contains(id));

        let mut players = Vec::with_capacity(POD_SIZE);
        players.push(player);

        for i in 1..POD_SIZE {
            let Some(id) = self.next_player(&aggregate_stats, &performances) else {
                return Err(TournamentError::NotEnoughPlayers);
            };
            log::debug!("Selected Player {id} for Slot {i}");

            players.push(id);
            performances.remove(&id);

            for (pl, per) in analytics.player_performance(id).into_iter().flatten() {
                performances.entry(pl).and_modify(|entry| entry.add_assign(per));
            }
            aggregate_stats += self.tourn.get_player_or_default_stats(id);
            log::debug!("Updated Stats: {aggregate_stats:?}");
        }

        let game_players = players
            .into_iter()
            .collect_array()
            .ok_or(TournamentError::NotEnoughPlayers)?;

        self.tourn.create_match(game_players)
    }

    fn next_player(
        &self,
        stats: &AggregateStats,
        performances: &HashMap<PlayerId, MatchPerformance>,
    ) -> Option<PlayerId> {
        let elo = stats
            .avg_elo()
            .unwrap_or_else(|| self.tourn.default_stats().elo());
        Some(
            *performances
                .iter()
                .min_by(|(l_id, l_perf), (r_id, r_perf)| {
                    {
                        // First: Least played against
                        l_perf.played().cmp(&r_perf.played())
                    }
                    .then_with(|| {
                        // Avoid if game count is within calibration
                        if let Ok(games) = u32::try_from(l_perf.played())
                            && games <= self.tourn.game_config().calibration_games()
                        {
                            return Ordering::Equal;
                        }

                        // Second: Most even match
                        even_match(l_perf).cmp(&even_match(r_perf))
                    })
                    .then_with(|| {
                        // Third: Closest Elo
                        let l_elo = self.tourn.get_player_or_default_stats(**l_id).elo();
                        let r_elo = self.tourn.get_player_or_default_stats(**r_id).elo();

                        let l_diff = (l_elo - elo).abs();
                        let r_diff = (r_elo - elo).abs();
                        l_diff.total_cmp(&r_diff)
                    })
                    .then_with(|| {
                        // Last: ID
                        l_id.cmp(r_id)
                    })
                })?
                .0,
        )
    }

    fn create_player_pool(&self, player: PlayerId) -> im::HashSet<PlayerId> {
        let target_elo = self.tourn.get_player_or_default_stats(player).elo();
        let elo_range = self.config.elo_range();

        let (mut pool, outliers): (Vec<_>, _) = self.players.iter().copied().partition(|id| {
            (self.tourn.get_player_or_default_stats(*id).elo() - target_elo).abs() < elo_range
        });

        pool.retain(|id| *id != player);

        if pool.len() < self.config.min_pool_size() {
            pool.reserve(self.config.min_pool_size() - pool.len());

            let mut heap = outliers
                .into_iter()
                .filter_map(|id| {
                    Some(HeapPlayer {
                        id,
                        diff: NotNan::new(
                            (self.tourn.get_player_or_default_stats(id).elo() - target_elo).abs(),
                        )
                        .ok()?,
                    })
                })
                .collect::<BinaryHeap<_>>();

            while pool.len() < self.config.min_pool_size()
                && let Some(entry) = heap.pop()
            {
                pool.push(entry.id);
            }
        }

        pool.into_iter().collect()
    }
}

const fn even_match(perf: &MatchPerformance) -> usize {
    /*
     * An even match means the player should lose to a given player 1 / N times
     * and win 1 / N times, where N is the number of players in the match
     *
     * GAMES * SIZE = SCALED_TOTAL
     * WINS * SIZE = SCALED_WIN
     * LOSSES * SIZE = SCALED_LOSS
     * EXPECTED_WINS = SCALED_TOTAL / SIZE
     * EXPECTED_LOSSES = SCALED_TOTAL / SIZE
     *
     * simplifies to:
     * (LOSSES * SIZE) expected to be GAMES
     * (WINS * SIZE) expected to be GAMES
     *
     */
    let wins = perf.wins() * POD_SIZE;
    let losses = perf.losses() * POD_SIZE;
    let diff_wins = wins.abs_diff(perf.played());
    let diff_losses = losses.abs_diff(perf.played());

    diff_wins + diff_losses
}

struct HeapPlayer {
    id: PlayerId,
    diff: NotNan<f64>,
}

impl PartialEq for HeapPlayer {
    fn eq(&self, other: &Self) -> bool {
        self.diff == other.diff && self.id == other.id
    }
}

impl Eq for HeapPlayer {}

impl PartialOrd for HeapPlayer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HeapPlayer {
    fn cmp(&self, other: &Self) -> Ordering {
        self.diff
            .cmp(&other.diff)
            .reverse()
            .then_with(|| self.id.cmp(&other.id))
    }
}
