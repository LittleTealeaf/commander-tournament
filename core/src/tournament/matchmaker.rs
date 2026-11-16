use core::{cmp::Reverse, ops::AddAssign};
use std::collections::{BinaryHeap, HashMap};

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
    #[allow(clippy::cast_precision_loss, reason = "Doesn't need to be accurate")]
    pub fn create_match(&self, player: PlayerId) -> Result<Matchup, TournamentError> {
        log::debug!("Creating Match for Player {player}");
        let stats = self.tourn.get_player_or_default_stats(player);

        let repeat_penalty = {
            if stats.games() < self.tourn.game_config().calibration_games() && self.config.strict_calibrate()
            {
                0.0
            } else {
                const DIVISOR: f64 = (POD_SIZE - 1) as f64;
                self.config.elo_range() / DIVISOR
            }
        };

        let pool = self
            .create_player_pool(player)
            .into_iter()
            .collect::<im::HashSet<PlayerId>>();
        let analytics = self.tourn.analytics().with_pool(|id| pool.contains(&id));
        let mut aggregate_stats = AggregateStats::from(stats);
        let mut performances = analytics.player_performance_all_others(player)?;

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
                repeat_penalty,
            ) else {
                return Err(TournamentError::NotEnoughPlayers);
            };
            log::debug!("Selected Player {id} for Slot {i}");

            players.push(id);
            performances.remove(&id);

            if let Ok(perfs) = analytics.player_performance(id) {
                for (pl, per) in perfs {
                    performances.entry(pl).and_modify(|entry| entry.add_assign(per));
                }
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
        aggregate_stats: &AggregateStats,
        performances: &HashMap<PlayerId, MatchPerformance>,
        deprioritize_unranked: bool,
        deprioritize_precon: bool,
        repeat_penalty: f64,
    ) -> Option<PlayerId> {
        let avg_elo = aggregate_stats
            .avg_elo()
            .unwrap_or_else(|| self.tourn.default_stats().elo());
        let calibrate_games = self.tourn.game_config().calibration_games();

        Some(
            *performances
                .iter()
                .min_by_key(|(id, perf)| {
                    let stats = self.tourn.get_player_or_default_stats(**id);
                    let info = self.tourn.get_player_info(id);

                    let elo_diff = (stats.elo() - avg_elo).abs();
                    let penalty = perf.played() as f64 * repeat_penalty;
                    let score = elo_diff + penalty;

                    (
                        deprioritize_unranked && (stats.games() < calibrate_games),
                        deprioritize_precon && info.map_or_default(PlayerInfo::is_precon),
                        NotNan::new(score).ok(),
                        **id,
                    )
                })?
                .0,
        )
    }

    fn create_player_pool(&self, player: PlayerId) -> Vec<PlayerId> {
        let target_elo = self.tourn.get_player_or_default_stats(player).elo();
        let elo_range = self.config.elo_range();

        let (mut pool, mut outliers): (Vec<_>, BinaryHeap<_>) =
            self.valid_players(player).partition_map(|id| {
                let elo = self.tourn.get_player_or_default_stats(id).elo();
                let diff = (elo - target_elo).abs();
                if diff <= elo_range {
                    Either::Left(id)
                } else {
                    Either::Right(Reverse((NotNan::new(diff).ok(), id)))
                }
            });

        pool.retain(|id| *id != player);

        while pool.len() < self.config.min_pool_size()
            && let Some(Reverse((_, id))) = outliers.pop()
        {
            pool.push(id);
        }

        pool
    }

    fn valid_players(&self, id: PlayerId) -> impl Iterator<Item = PlayerId> + '_ {
        self.players
            .iter()
            .filter(move |pid| {
                *pid != &id
                    && !self
                        .tourn
                        .get_player_info(pid)
                        .map_or_default(PlayerInfo::is_archived)
            })
            .copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn archived_not_valid() {
        let mut tourn = Tournament::new();
        let [id_a, id_b] = tourn.register_debug_players().unwrap();
        let id_archived = tourn
            .register_debug_player_info(|info| info.with_is_archived(true))
            .unwrap();

        let matchmaker = tourn.matchmaker();
        let players = matchmaker.valid_players(id_a).collect::<Vec<_>>();
        assert!(players.contains(&id_b));
        assert!(!players.contains(&id_archived));
    }

    #[test]
    fn player_pool_within_range() {
        let mut tourn = Tournament::new();
        tourn
            .config
            .set_game(tourn.game_config().clone().with_calibration_games(0));
        tourn.set_matchmaker_config(tourn.matchmaker_config().clone().with_elo_range(75.0));
        tourn.reload().unwrap();

        let id_target = tourn.register_debug_player().unwrap();

        let base = tourn.config.game().initial_elo();

        let mut in_range = Vec::new();
        let mut out_of_range = Vec::new();

        for diff in [0.0, 50.0, -50.0, 100.0, -100.0] {
            let ids: [_; 10] = tourn.register_debug_players().unwrap();
            let target_elo = diff + base;
            for i in ids {
                tourn.debug_set_target_elo(i, target_elo).unwrap();
            }
            if diff.abs() <= 75.0 {
                in_range.extend(ids);
            } else {
                out_of_range.extend(ids);
            }
        }

        let matchmaker = tourn.matchmaker();
        let pool = matchmaker.create_player_pool(id_target);

        assert_eq!(pool.len(), in_range.len());
        assert!(!pool.contains(&id_target));
        for id in in_range {
            assert!(pool.contains(&id));
        }
        for id in out_of_range {
            assert!(!pool.contains(&id));
        }
    }
}
