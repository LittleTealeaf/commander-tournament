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

        let is_unranked = stats.games() < self.tourn.game_config().calibration_games();

        let mut count_unranked = usize::from(is_unranked);
        let mut count_precon = usize::from(info.is_precon());

        let max_unranked = usize::from(!(is_unranked && config.strict_calibrate())) * config.max_unranked();

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

            if info.is_precon() {
                count_precon += 1;
            } else if stats.games() < game_config.calibration_games() {
                count_unranked += 1;
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

        Ok(performances
            .into_iter()
            .filter_map(|(id, perf)| {
                let stats = self.tourn.get_player_or_default_stats(*id);
                let info = self.tourn.get_player_info(id)?;

                let elo_diff = (stats.elo() - avg_elo).abs();
                let penalty = perf.played() as f64 * config.repeat_penalty();
                let score = elo_diff + penalty;

                let is_unranked = stats.games() < calibrate_games;

                Some((
                    deprioritize_unranked && is_unranked,
                    deprioritize_precon && info.is_precon(),
                    NotNan::new(score).ok()?,
                    *id,
                ))
            })
            .min()
            .ok_or(TournamentError::NotEnoughPlayers)?
            .3)
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

    fn valid_players(&self) -> impl Iterator<Item = PlayerId> {
        self.tourn
            .players()
            .iter()
            .filter_map(|(id, info)| (self.player_filter)(*id, info).then_some(*id))
    }
}

#[cfg(test)]
mod tests {
    use crate::config::game::GameConfig;

    use super::*;

    #[test]
    fn config_overrides() {
        let mut tourn = Tournament::new();
        let config = MatchmakerConfig::default().with_max_precon(3);
        tourn.set_matchmaker_config(config);

        let matchmaker = tourn.matchmaker();
        assert_eq!(3, matchmaker.config().max_precon());

        let config = MatchmakerConfig::default().with_max_precon(10);
        let matchmaker = matchmaker.with_config(config);
        assert_eq!(10, matchmaker.config().max_precon());
    }

    #[test]
    fn valid_players_uses_filter() {
        let mut tourn = Tournament::new();
        let [id_a, id_b, id_c, id_d] = tourn.register_debug_players().unwrap();
        tourn
            .update_player_info(&id_a, |info| info.with_is_precon(true))
            .unwrap();
        tourn
            .update_player_info(&id_c, |info| info.with_is_archived(true))
            .unwrap();

        let matchmaker = tourn.matchmaker();

        assert!(
            !matchmaker.valid_players().contains(&id_c),
            "Included archived player"
        );

        let matchmaker = matchmaker.with_filter(|id, info| !info.is_precon() && id != id_b);
        let valid_players = matchmaker.valid_players().collect::<Vec<_>>();

        assert!(!valid_players.contains(&id_a), "Info filter failed");
        assert!(!valid_players.contains(&id_b), "Id filter failed");
        assert!(!valid_players.contains(&id_c), "Included Archived");
        assert!(valid_players.contains(&id_d), "Failed to include valid player");
    }

    #[test]
    fn pool_minimum_from_nearby() {
        let mut tourn = Tournament::new()
            .with_matchmaker_config(
                MatchmakerConfig::default()
                    .with_elo_range(1.0)
                    .with_min_pool_size(3),
            )
            .with_game_config(
                GameConfig::default()
                    .with_calibration_games(0)
                    .with_base_k(1.0)
                    .with_initial_elo(1500.0),
            )
            .unwrap();

        let [id_a, id_b, id_c, id_d, id_e] = tourn.register_debug_players().unwrap();
        tourn.debug_set_target_elo(id_a, 1500.0).unwrap();
        tourn.debug_set_target_elo(id_b, 1490.0).unwrap();
        tourn.debug_set_target_elo(id_c, 1520.0).unwrap();
        tourn.debug_set_target_elo(id_d, 1530.0).unwrap();
        tourn.debug_set_target_elo(id_e, 1540.0).unwrap();

        let matchmaker = tourn.matchmaker().create_pool(id_a);
        assert!(matchmaker.contains(&id_b));
        assert!(matchmaker.contains(&id_c));
        assert!(matchmaker.contains(&id_d));
        assert!(!matchmaker.contains(&id_e));
    }
}
