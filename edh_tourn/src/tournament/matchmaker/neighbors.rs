use core::cmp::Ordering;
use std::collections::HashMap;

use itertools::{Itertools, chain};

use crate::{
    analytics::{aggregate::AggregateStats, winloss::MatchPerformance},
    config::game::GameConfig,
    game::matchable::{Matchable, calculate_expected_values},
    player::PlayerId,
    tournament::matchmaker::{Matchmaker, to_weight_rank},
};

fn ordered_by_proximity(target: f64, left: f64, right: f64) -> Ordering {
    let diff_a = (target - left).abs();
    let diff_b = (target - right).abs();
    diff_a.total_cmp(&diff_b)
}

impl Matchmaker<'_> {
    pub(super) fn ranked_neighbors(
        &self,
        agg_stats: &AggregateStats,
        performances: &HashMap<PlayerId, MatchPerformance>,
    ) -> impl Iterator<Item = (PlayerId, usize)> {
        let config = self.config();
        let game_config = self.tourn.game_config();
        chain!(
            to_weight_rank(
                sort_by_proximity(
                    performances.keys().copied(),
                    agg_stats.wr().unwrap_or(0.0),
                    |id| { self.tourn.get_player_or_default_stats(*id).wr().unwrap_or(0.0) }
                ),
                config.wr_neighbor
            ),
            to_weight_rank(
                sort_by_proximity(
                    performances.keys().copied(),
                    agg_stats
                        .avg_elo()
                        .unwrap_or_else(|| self.tourn.default_stats().elo()),
                    |id| { self.tourn.get_player_or_default_stats(*id).elo() }
                ),
                config.elo_neighbor
            ),
            to_weight_rank(
                sort_by_proximity(performances.keys().copied(), 0.5, |id| {
                    let stats = self.tourn.get_player_or_default_stats(*id);
                    calc_expected(game_config, agg_stats, stats.wr(), stats.elo())
                }),
                config.expected_neighbor
            )
        )
    }
}

fn calc_expected(config: &GameConfig, stats: &AggregateStats, wr: Option<f64>, elo: f64) -> f64 {
    let players = [
        BaseMatchable {
            elo: stats.avg_elo().unwrap_or(config.starting_elo),
            wr: stats.wr(),
        },
        BaseMatchable { elo, wr },
    ];

    let [_, (_, expected)] = calculate_expected_values(config, players);
    expected
}

fn sort_by_proximity<I, F>(players: I, target: f64, value_by: F) -> impl Iterator<Item = PlayerId>
where
    I: IntoIterator<Item = PlayerId>,
    F: Fn(&PlayerId) -> f64,
{
    players
        .into_iter()
        .map(|id| (id, (target - value_by(&id)).abs()))
        .sorted_by(|(id_a, val_a), (id_b, val_b)| val_a.total_cmp(val_b).then_with(|| id_a.cmp(id_b)))
        .map(|(id, _)| id)
}

#[derive(Debug)]
struct BaseMatchable {
    elo: f64,
    wr: Option<f64>,
}

impl Matchable for BaseMatchable {
    fn elo(&self) -> f64 {
        self.elo
    }

    fn wr(&self) -> Option<f64> {
        self.wr
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ordered_by_proximity_helper() {
        assert_eq!(ordered_by_proximity(10.0, 9.0, 12.0), Ordering::Less); // Diff 1 vs 2
        assert_eq!(ordered_by_proximity(10.0, 8.0, 11.0), Ordering::Greater); // Diff 2 vs 1
        assert_eq!(ordered_by_proximity(10.0, 9.0, 11.0), Ordering::Equal); // Diff 1 vs 1
    }
}
