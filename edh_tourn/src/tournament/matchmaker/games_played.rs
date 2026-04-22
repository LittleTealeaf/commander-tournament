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
    left.draws().cmp(&right.draws()).reverse()
}

#[cfg(test)]
mod tests {
    use crate::tournament::Tournament;

    use super::*;

    #[test]
    fn test_order_least_played() {
        let p_few = MatchPerformance::new(2, 1, 1);
        let p_many = MatchPerformance::new(10, 5, 5);
        let p_equal = MatchPerformance::new(2, 0, 2);

        assert_eq!(order_least_played(&p_few, &p_many), Ordering::Less);
        assert_eq!(order_least_played(&p_many, &p_few), Ordering::Greater);
        assert_eq!(order_least_played(&p_few, &p_equal), Ordering::Equal);
    }

    #[test]
    fn test_order_lost_with() {
        let p_high_draws = MatchPerformance::new(10, 2, 2);
        let p_low_draws = MatchPerformance::new(10, 8, 1);

        assert_eq!(order_lost_with(&p_high_draws, &p_low_draws), Ordering::Less);
        assert_eq!(
            order_lost_with(&p_low_draws, &p_high_draws),
            Ordering::Greater
        );
    }

    #[test]
    fn test_sort_by_tie_breaker_uses_elo_proximity() {
        let t = Tournament::generate_tournament(10, 20).unwrap();
        let mm = t.matchmaker();

        let target_elo = 1600.0;

        // Pass identical default performances to force the tie breaker logic to kick in
        let performances: Vec<(PlayerId, MatchPerformance)> = t
            .players()
            .keys()
            .map(|&id| (id, MatchPerformance::default()))
            .collect();

        let ranked: Vec<PlayerId> = mm
            .sort_by(target_elo, performances, |_, _| Ordering::Equal)
            .map(|(id, _)| id)
            .collect();

        // Ensure the results are sorted ascendingly by distance to `target_elo`
        let mut previous_diff = -1.0;
        for id in ranked {
            let elo = t.get_player_or_default_stats(id).elo();
            let diff = (target_elo - elo).abs();
            assert!(
                diff >= previous_diff,
                "List not sorted by elo proximity. Prev diff: {previous_diff}, Curr diff: {diff}",
            );
            previous_diff = diff;
        }
    }
}
