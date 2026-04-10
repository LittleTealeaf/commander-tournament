use core::cmp::Ordering;
use std::collections::HashMap;

use itertools::{Itertools, chain};

use crate::{
    analytics::{aggregate::AggregateStats, winloss::MatchPerformance},
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
}
