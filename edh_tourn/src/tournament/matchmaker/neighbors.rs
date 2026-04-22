use core::cmp::Ordering;
use std::collections::HashMap;

use itertools::{Itertools, chain};

use crate::{
    analytics::{aggregate::AggregateStats, winloss::MatchPerformance},
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
        self,
        agg_stats: &AggregateStats,
        performances: &HashMap<PlayerId, MatchPerformance>,
    ) -> impl Iterator<Item = (PlayerId, usize)> {
        let config = self.0.matchmaker_config();
        chain!(
            to_weight_rank(
                self.wr_neighbor(agg_stats.wr().unwrap_or(0.0), performances.keys().copied()),
                config.wr_neighbor
            ),
            to_weight_rank(
                self.elo_neighbor(
                    agg_stats
                        .avg_elo()
                        .unwrap_or_else(|| self.0.default_stats().elo()),
                    performances.keys().copied()
                ),
                config.elo_neighbor
            ),
            to_weight_rank(
                self.expected_neighbor(agg_stats, performances.keys().copied(),),
                config.expected_neighbor
            )
        )
    }

    fn elo_neighbor<I>(self, target_elo: f64, players: I) -> impl Iterator<Item = PlayerId>
    where
        I: IntoIterator<Item = PlayerId>,
    {
        players
            .into_iter()
            .map(|id| (id, self.0.get_player_or_default_stats(id).elo()))
            .sorted_by(|(id_a, elo_a), (id_b, elo_b)| {
                ordered_by_proximity(target_elo, *elo_a, *elo_b).then_with(|| id_a.cmp(id_b))
            })
            .map(|(id, _)| id)
    }

    fn wr_neighbor<I>(self, target_wr: f64, players: I) -> impl Iterator<Item = PlayerId>
    where
        I: IntoIterator<Item = PlayerId>,
    {
        players
            .into_iter()
            .map(|id| {
                (
                    id,
                    self.0.get_player_or_default_stats(id).wr().unwrap_or(0.0),
                )
            })
            .sorted_by(|(id_a, wr_a), (id_b, wr_b)| {
                ordered_by_proximity(target_wr, *wr_a, *wr_b).then_with(|| id_a.cmp(id_b))
            })
            .map(|(id, _)| id)
    }

    fn expected_neighbor<I>(
        self,
        stats: &AggregateStats,
        players: I,
    ) -> impl Iterator<Item = PlayerId>
    where
        I: IntoIterator<Item = PlayerId>,
    {
        players
            .into_iter()
            .map(|id| {
                let player_stats = self.0.get_player_or_default_stats(id);
                let expected_diff =
                    self.calc_expected_diff(stats, player_stats.wr(), player_stats.elo());
                (id, expected_diff)
            })
            .sorted_by(|(ida, diffa), (idb, diffb)| {
                diffa.total_cmp(diffb).then_with(|| ida.cmp(idb))
            })
            .map(|(id, _)| id)
    }

    fn calc_expected_diff(self, stats: &AggregateStats, wr: Option<f64>, elo: f64) -> f64 {
        let config = self.0.game_config();

        let players = [
            BaseMatchable {
                elo: stats.avg_elo().unwrap_or(config.starting_elo),
                wr: stats.wr(),
            },
            BaseMatchable { elo, wr },
        ];

        let [_, (_, expected)] = calculate_expected_values(config, players);

        (expected - 0.5).abs()
    }
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
    use crate::tournament::Tournament;

    #[test]
    fn test_elo_neighbor_ranking() {
        let t = Tournament::generate_tournament(15, 30).unwrap();
        let mm = t.matchmaker();
        let target_elo = 1550.0;
        let players: Vec<PlayerId> = t.players().keys().copied().collect();

        let ranked: Vec<PlayerId> = mm.elo_neighbor(target_elo, players).collect();

        // Verify players are sorted by absolute distance to the target Elo
        let mut previous_diff = -1.0;
        for id in ranked {
            let elo = t.get_player_or_default_stats(id).elo();
            let diff = (target_elo - elo).abs();
            assert!(
                diff >= previous_diff,
                "Elo neighbor ordering failed. Prev: {previous_diff}, Curr: {diff}",
            );
            previous_diff = diff;
        }
    }

    #[test]
    fn test_wr_neighbor_ranking() {
        let t = Tournament::generate_tournament(15, 30).unwrap();
        let mm = t.matchmaker();
        let target_wr = 0.5;
        let players: Vec<PlayerId> = t.players().keys().copied().collect();

        let ranked: Vec<PlayerId> = mm.wr_neighbor(target_wr, players).collect();

        // Verify players are sorted by absolute distance to the target Win Rate
        let mut previous_diff = -1.0;
        for id in ranked {
            let wr = t.get_player_or_default_stats(id).wr().unwrap_or(0.0);
            let diff = (target_wr - wr).abs();
            assert!(
                diff >= previous_diff,
                "WR neighbor ordering failed. Prev: {previous_diff}, Curr: {diff}",
            );
            previous_diff = diff;
        }
    }

    #[test]
    fn test_expected_neighbor_ranking() {
        let t = Tournament::generate_tournament(10, 20).unwrap();
        let mm = t.matchmaker();

        let seed_id = *t.players().keys().next().unwrap();
        let agg_stats = AggregateStats::from(t.get_player_or_default_stats(seed_id));
        let players: Vec<PlayerId> = t.players().keys().copied().collect();

        let ranked: Vec<PlayerId> = mm.expected_neighbor(&agg_stats, players).collect();

        // Verify sorting by absolute difference mapped against standard base expected probability (0.5)
        let mut previous_diff = -1.0;
        for id in ranked {
            let stats = t.get_player_or_default_stats(id);
            let diff = mm.calc_expected_diff(&agg_stats, stats.wr(), stats.elo());
            assert!(
                diff >= previous_diff,
                "Expected neighbor ordering failed. Prev: {previous_diff}, Curr: {diff}",
            );
            previous_diff = diff;
        }
    }

    #[test]
    fn test_ordered_by_proximity_helper() {
        assert_eq!(ordered_by_proximity(10.0, 9.0, 12.0), Ordering::Less); // Diff 1 vs 2
        assert_eq!(ordered_by_proximity(10.0, 8.0, 11.0), Ordering::Greater); // Diff 2 vs 1
        assert_eq!(ordered_by_proximity(10.0, 9.0, 11.0), Ordering::Equal); // Diff 1 vs 1
    }
}
