use std::{collections::HashMap};
use core::cmp::Ordering;

use itertools::{Itertools, chain};

use crate::{
    analytics::{aggregate::AggregateStats, winloss::MatchPerformance},
    game::POD_SIZE,
    player::PlayerId,
    tournament::matchmaker::Matchmaker,
};

impl Matchmaker<'_> {
    /// Grabs the next player based on the following algorithm:
    ///
    /// First, orders (evenly) between the following criteria
    /// - Closest to the aggregated average elo
    /// - Least Played
    /// - Evenly Matched (Equal wins / losses / draws)
    ///
    /// Tie-Breakers at all points are handed by highest elo, and then by player id
    pub(super) fn next_player(
        &self,
        stats: &AggregateStats,
        performances: &HashMap<PlayerId, MatchPerformance>,
    ) -> Option<PlayerId> {
        let scores = chain!(
            self.rank_least_played(performances),
            self.rank_even_match(performances),
            stats
                .avg_elo()
                .into_iter()
                .flat_map(|elo| { self.rank_elo_neighbors(elo, performances.keys().copied()) })
        );
        let ranking = scores.into_grouping_map().sum();
        let next = ranking
            .into_iter()
            .min_by(|(left_id, left_rank), (right_id, right_rank)| {
                left_rank
                    .cmp(right_rank)
                    .then_with(|| self.tie_breaker(*left_id, *right_id))
            })?
            .0;
        Some(next)
    }

    fn rank_least_played(
        &self,
        performances: &HashMap<PlayerId, MatchPerformance>,
    ) -> impl Iterator<Item = (PlayerId, usize)> {
        performances
            .iter()
            .sorted_by(|(left_id, left_perf), (right_id, right_perf)| {
                left_perf
                    .played()
                    .cmp(&right_perf.played())
                    .then_with(|| self.tie_breaker(**left_id, **right_id))
            })
            .enumerate()
            .map(|(rank, (id, _))| (*id, rank))
    }

    fn rank_even_match(
        &self,
        performances: &HashMap<PlayerId, MatchPerformance>,
    ) -> impl Iterator<Item = (PlayerId, usize)> {
        performances
            .iter()
            .map(|(id, perf)| {
                /*
                 * GAMES * SIZE = SCALED_TOTAL
                 * WINS * SIZE = SCALED_WIN
                 * LOSSES * SIZE = SCALED_LOSS
                 * EXPECTED_WINS = SCALED_TOTAL / SIZE
                 * EXPECTED_LOSSES = SCALED_TOTAL / SIZE
                 *
                 * simplifies to:
                 * (LOSSES * SIZE) expected to be GAMES
                 * (WINS * SIZE) expected to be GAMES
                 */
                let wins = perf.played() * POD_SIZE;
                let losses = perf.played() * POD_SIZE;

                let diff_wins = wins.abs_diff(perf.played());
                let diff_losses = losses.abs_diff(perf.played());

                (*id, diff_wins + diff_losses)
            })
            .sorted_by(|(left_id, left_score), (right_id, right_score)| {
                left_score
                    .cmp(right_score)
                    .then_with(|| self.tie_breaker(*left_id, *right_id))
            })
            .enumerate()
            .map(|(rank, (id, _))| (id, rank))
    }

    fn rank_elo_neighbors<I>(&self, target_elo: f64, players: I) -> impl Iterator<Item = (PlayerId, usize)>
    where
        I: IntoIterator<Item = PlayerId>,
    {
        players
            .into_iter()
            .map(|id| {
                let stats = self.tourn.get_player_or_default_stats(id);
                let elo = stats.elo();
                let diff = (elo - target_elo).abs();
                (id, elo, diff)
            })
            .sorted_by(|(id_a, elo_a, diff_a), (id_b, elo_b, diff_b)| {
                diff_a
                    .total_cmp(diff_b)
                    .then_with(|| elo_a.total_cmp(elo_b).then_with(|| id_a.cmp(id_b)))
            })
            .enumerate()
            .map(|(rank, (id, _, _))| (id, rank))
    }

    fn tie_breaker(&self, left: PlayerId, right: PlayerId) -> Ordering {
        let left_elo = self.tourn.get_player_or_default_stats(left).elo();
        let right_elo = self.tourn.get_player_or_default_stats(right).elo();
        left_elo.total_cmp(&right_elo).reverse().then(left.cmp(&right))
    }
}
