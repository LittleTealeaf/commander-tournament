use core::{cmp::Reverse, ops::AddAssign};
use std::collections::BinaryHeap;

use itertools::{Either, Itertools};
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
pub struct Matchmaker<'a, F>
where
    F: Fn(PlayerId, &PlayerInfo) -> bool,
{
    tourn: &'a Tournament,
    player_filter: F,
    config: Option<MatchmakerConfig>,
}

impl Tournament {
    #[must_use]
    pub fn matchmaker(&self) -> Matchmaker<'_, impl Fn(PlayerId, &PlayerInfo) -> bool> {
        Matchmaker {
            tourn: self,
            player_filter: |_, info| !info.is_archived(),
            config: None,
        }
    }
}

impl<'a, F> Matchmaker<'a, F>
where
    F: Fn(PlayerId, &PlayerInfo) -> bool,
{
    pub fn with_filter<F2>(self, filter: F2) -> Matchmaker<'a, impl Fn(PlayerId, &PlayerInfo) -> bool>
    where
        F2: Fn(PlayerId, &PlayerInfo) -> bool,
    {
        Matchmaker {
            tourn: self.tourn,
            player_filter: move |id, info| (self.player_filter)(id, info) && filter(id, info),
            config: self.config,
        }
    }

    pub fn with_config(self, config: MatchmakerConfig) -> Self {
        Self {
            config: Some(config),
            ..self
        }
    }

    fn config(&self) -> &MatchmakerConfig {
        self.config
            .as_ref()
            .map_or_else(|| self.tourn.matchmaker_config(), |config| config)
    }

    fn valid_players(&self) -> impl Iterator<Item = PlayerId> {
        self.tourn
            .players()
            .iter()
            .filter_map(|(id, info)| (self.player_filter)(*id, info).then_some(*id))
    }

    pub fn create_match(&self, player: PlayerId) -> Result<Matchup, TournamentError> {
        // Get player info
        let info = self.tourn.require_player_info(player)?;
        let stats = self.tourn.get_player_or_default_stats(player);

        // Get Configuration
        let config = self.config();
        let game_config = self.tourn.game_config();

        let pool = im::HashSet::<PlayerId>::from_iter(self.create_pool(player));
        let analytics = self.tourn.analytics().with_pool(|id| pool.contains(&id));
        let mut aggregate_stats = AggregateStats::from(stats);
        let mut performances = analytics.player_performance_all_others(player)?;

        let mut players = Vec::with_capacity(POD_SIZE);
        players.push(player);

        let mut count_unranked = usize::from(stats.games() < self.tourn.game_config().calibration_games());
        let mut count_precon = usize::from(info.is_precon());

        let max_unranked = if count_unranked > 0 && config.strict_calibrate() {
            0
        } else {
            config.max_unranked()
        };

        for _ in 1..POD_SIZE {
            let id = self.next_player(
                &aggregate_stats,
                &performances,
                count_unranked >= max_unranked,
                count_precon >= config.max_precon(),
            )?;
            let info = self.tourn.require_player_info(id)?;

            players.push(id);
            performances.remove(&id);

            if let Ok(perfs) = analytics.player_performance(id) {
                for (pl, per) in perfs {
                    performances.entry(pl).and_modify(|entry| entry.add_assign(per));
                }
            }

            let stats = self.tourn.get_player_or_default_stats(id);

            if stats.games() < game_config.calibration_games() {
                count_unranked += 1;
            }

            if info.is_precon() {
                count_precon += 1;
            }

            aggregate_stats += stats;
        }

        let game_players = players
            .into_iter()
            .collect_array()
            .ok_or(TournamentError::NotEnoughPlayers)?;

        self.tourn.create_match(game_players)
    }

    #[allow(clippy::cast_precision_loss, reason = "Exactness not needed")]
    fn next_player<I>(
        &self,
        aggregate: &AggregateStats,
        performances: I,
        deprioritize_unranked: bool,
        deprioritize_precon: bool,
    ) -> Result<PlayerId, TournamentError>
    where
        I: IntoIterator<Item = (&'a PlayerId, &'a MatchPerformance)>,
    {
        let avg_elo = aggregate
            .avg_elo()
            .unwrap_or_else(|| self.tourn.default_stats().elo());

        let calibrate_games = self.tourn.game_config().calibration_games();
        let config = self.config();

        Ok(*performances
            .into_iter()
            .min_by_key(|(id, perf)| {
                let stats = self.tourn.get_player_or_default_stats(**id);
                let info = self.tourn.get_player_info(id);

                let elo_diff = (stats.elo() - avg_elo).abs();
                let penalty = perf.played() as f64 * config.repeat_penalty();
                let score = elo_diff + penalty;

                (
                    deprioritize_unranked && (stats.games() < calibrate_games),
                    deprioritize_precon && info.map_or_default(PlayerInfo::is_precon),
                    NotNan::new(score).ok(),
                    **id,
                )
            })
            .ok_or(TournamentError::NotEnoughPlayers)?
            .0)
    }

    fn create_pool(&self, player: PlayerId) -> Vec<PlayerId> {
        let config = self.config();
        let target_elo = self.tourn.get_player_or_default_stats(player).elo();
        let range = config.elo_range();

        let (mut pool, mut outliers): (Vec<_>, BinaryHeap<_>) = self.valid_players().partition_map(|id| {
            let elo = self.tourn.get_player_or_default_stats(id).elo();
            let diff = (elo - target_elo).abs();
            if diff <= range {
                Either::Left(id)
            } else {
                Either::Right(Reverse((NotNan::new(diff).ok(), id)))
            }
        });

        pool.retain(|id| *id != player);

        while pool.len() < config.min_pool_size()
            && let Some(Reverse((_, id))) = outliers.pop()
        {
            pool.push(id);
        }

        pool
    }
}
