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
        let compare_elo = stats
            .avg_elo()
            .unwrap_or_else(|| self.0.default_stats().elo());

        let elo = elo.powf(config.game_elo_pow_scale);
        let sum_elo = compare_elo.powf(config.game_elo_pow_scale) + elo;

        let (Some(wr), Some(compare_wr)) = (wr, stats.wr()) else {
            let perc = elo / sum_elo;
            return perc;
        };

        let elo_coef = config.game_elo_weight / sum_elo;

        let wr = wr.powf(config.game_wr_pow_scale);
        let sum_wr = compare_wr.powf(config.game_wr_pow_scale) + wr;
        let wr_coef = config.game_wr_weight / sum_wr;

        let expected = wr.mul_add(wr_coef, elo * elo_coef);

        (0.5 - expected).abs()
    }
}
