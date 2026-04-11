use core::cmp::Ordering;
use std::collections::HashMap;

use itertools::{Itertools, chain};

use crate::{
    analytics::{aggregate::AggregateStats, winloss::MatchPerformance},
    player::PlayerId,
    tournament::matchmaker::{Matchmaker, to_weight_rank},
    utils::IntoCopiedIter,
};

impl Matchmaker<'_> {
    pub(super) fn ranked_games_played(
        self,
        agg_stats: &AggregateStats,
        performances: &HashMap<PlayerId, MatchPerformance>,
    ) -> impl Iterator<Item = (PlayerId, usize)> {
        let avg_elo = agg_stats
            .avg_elo()
            .unwrap_or_else(|| self.0.default_stats().elo());
        let config = self.0.matchmaker_config();

        chain!(
            to_weight_rank(
                self.sort_by(avg_elo, performances.iter_copied(), order_lost_with)
                    .map(|(id, _)| id),
                config.player_lost_with
            ),
            to_weight_rank(
                self.sort_by(avg_elo, performances.iter_copied(), Ord::cmp)
                    .map(|(id, _)| id),
                config.player_nemesis
            ),
            to_weight_rank(
                self.sort_by(avg_elo, performances.iter_copied(), order_least_played)
                    .map(|(id, _)| id),
                config.player_least_played
            )
        )
    }

    fn sort_by<F, I>(
        self,
        target_elo: f64,
        performances: I,
        sort_by: F,
    ) -> impl Iterator<Item = (PlayerId, MatchPerformance)>
    where
        I: IntoIterator<Item = (PlayerId, MatchPerformance)>,
        F: Fn(&MatchPerformance, &MatchPerformance) -> Ordering,
    {
        performances
            .into_iter()
            .map(|(id, perf)| (id, self.0.get_player_or_default_stats(id), perf))
            .sorted_by(|(pa, sa, ma), (pb, sb, mb)| {
                sort_by(ma, mb)
                    .then_with(|| {
                        let left = sa.elo();
                        let right = sb.elo();
                        let elo_diff_left = (target_elo - left).abs();
                        let elo_diff_right = (target_elo - right).abs();
                        elo_diff_left.total_cmp(&elo_diff_right)
                    })
                    .then_with(|| pa.cmp(pb))
            })
            .map(|(id, _, perf)| (id, perf))
    }
}

fn order_least_played(left: &MatchPerformance, right: &MatchPerformance) -> Ordering {
    left.played().cmp(&right.played())
}

fn order_lost_with(left: &MatchPerformance, right: &MatchPerformance) -> Ordering {
    left.draws().cmp(&right.draws())
}
