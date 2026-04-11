use core::cmp::Ordering;

use itertools::Itertools;

use crate::{
    analytics::winloss::MatchPerformance, player::PlayerId, tournament::matchmaker::Matchmaker,
};

impl Matchmaker<'_> {
    pub(super) fn sort_least_played<I>(
        self,
        target_elo: f64,
        performances: I,
    ) -> impl Iterator<Item = (PlayerId, MatchPerformance)>
    where
        I: IntoIterator<Item = (PlayerId, MatchPerformance)>,
    {
        self.sort_by(target_elo, performances, |a, b| a.draws().cmp(&b.draws()))
    }

    pub(super) fn sort_lost_with<I>(
        self,
        target_elo: f64,
        performances: I,
    ) -> impl Iterator<Item = (PlayerId, MatchPerformance)>
    where
        I: IntoIterator<Item = (PlayerId, MatchPerformance)>,
    {
        self.sort_by(target_elo, performances, |a, b| a.draws().cmp(&b.draws()))
    }

    pub(super) fn sort_nemesis<I>(
        self,
        target_elo: f64,
        performances: I,
    ) -> impl Iterator<Item = (PlayerId, MatchPerformance)>
    where
        I: IntoIterator<Item = (PlayerId, MatchPerformance)>,
    {
        self.sort_by(target_elo, performances, Ord::cmp)
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
                    .then_with(|| closest_to_value(target_elo, sa.elo(), sb.elo()))
                    .then_with(|| pa.cmp(pb))
            })
            .map(|(id, _, perf)| (id, perf))
    }
}

fn closest_to_value(target: f64, left: f64, right: f64) -> Ordering {
    let elo_diff_left = (target - left).abs();
    let elo_diff_right = (target - right).abs();
    elo_diff_left.total_cmp(&elo_diff_right)
}
