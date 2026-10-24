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

use super::super::player::info::PlayerInfo;

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

        let stats = self.tourn.get_player_or_default_stats(player);
        let mut aggregate_stats = AggregateStats::from(stats);
        log::debug!("Built Aggregate Stats: {aggregate_stats:?}");
        let mut performances = analytics.player_performance_all_others(player)?;
        performances.retain(|id, _| pool.contains(id));

        let mut players = Vec::with_capacity(POD_SIZE);
        players.push(player);

        let mut count_unranked = usize::from(stats.games() < self.tourn.game_config().calibration_games());
        let mut count_precon = usize::from(
            self.tourn
                .get_player_info(&player)
                .map_or_default(PlayerInfo::is_precon),
        );

        for i in 1..POD_SIZE {
            let Some(id) = self.next_player(
                &aggregate_stats,
                &performances,
                count_unranked >= self.config.max_unranked(),
                count_precon >= self.config.max_precon(),
            ) else {
                return Err(TournamentError::NotEnoughPlayers);
            };
            log::debug!("Selected Player {id} for Slot {i}");

            players.push(id);
            performances.remove(&id);

            for (pl, per) in analytics.player_performance(id).into_iter().flatten() {
                performances.entry(pl).and_modify(|entry| entry.add_assign(per));
            }

            let stats = self.tourn.get_player_or_default_stats(id);
            if stats.games() < self.tourn.game_config().calibration_games() {
                count_unranked += 1;
            }
            if self
                .tourn
                .get_player_info(&id)
                .map_or_default(PlayerInfo::is_precon)
            {
                count_precon += 1;
            }
            aggregate_stats += stats;
            log::debug!("Updated Stats: {aggregate_stats:?}");
        }

        let game_players = players
            .into_iter()
            .collect_array()
            .ok_or(TournamentError::NotEnoughPlayers)?;

        self.tourn.create_match(game_players)
    }

    #[allow(clippy::cast_precision_loss, reason = "Doesn't need to be accurate")]
    fn next_player(
        &self,
        agg_stats: &AggregateStats,
        performances: &HashMap<PlayerId, MatchPerformance>,
        deprioritize_unranked: bool,
        deprioritize_precon: bool,
    ) -> Option<PlayerId> {
        let avg_elo = agg_stats
            .avg_elo()
            .unwrap_or_else(|| self.tourn.default_stats().elo());

        let repeat_penalty = {
            const DIVISOR: f64 = (POD_SIZE - 1) as f64;
            self.config.elo_range() / DIVISOR
        };

        Some(
            performances
                .iter()
                .map(|(&id, perf)| {
                    let stats = self.tourn.get_player_or_default_stats(id);

                    let precon = deprioritize_precon
                        && self
                            .tourn
                            .get_player_info(&id)
                            .map_or_default(PlayerInfo::is_precon);

                    let elo_diff = (avg_elo - stats.elo()).abs();
                    let penalty = perf.played() as f64 * repeat_penalty;
                    let score = elo_diff + penalty;

                    let uncalibrated =
                        deprioritize_unranked && stats.games() < self.tourn.game_config().calibration_games();

                    (id, uncalibrated, precon, score)
                })
                .min_by(
                    |(id_a, unranked_a, precon_a, score_a), (id_b, unranked_b, precon_b, score_b)| {
                        unranked_a
                            .cmp(unranked_b)
                            .then_with(|| precon_a.cmp(precon_b))
                            .then_with(|| score_a.total_cmp(score_b))
                            .then_with(|| id_a.cmp(id_b))
                    },
                )?
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
