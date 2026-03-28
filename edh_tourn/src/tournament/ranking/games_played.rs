use core::cmp::Ordering;
use std::collections::HashSet;

use itertools::{Itertools, chain};

use crate::{
    analytics::winloss::MatchPerformance, error::TournamentError, player::RegisteredPlayer,
    tournament::ranking::Ranking,
};

fn closest_elo(
    elo: f64,
    player_a: &RegisteredPlayer<'_>,
    player_b: &RegisteredPlayer<'_>,
) -> Ordering {
    let elo_diff_a = (elo - player_a.stats().elo()).abs();
    let elo_diff_b = (elo - player_b.stats().elo()).abs();

    let cmp = elo_diff_a.total_cmp(&elo_diff_b);

    cmp.then_with(|| player_a.id().cmp(&player_b.id()))
}

impl<'a> Ranking<'a> {
    fn player_match_performance(
        self,
        id: u32,
    ) -> Result<impl Iterator<Item = (RegisteredPlayer<'a>, MatchPerformance)>, TournamentError>
    {
        let iter = self.0.analytics().player_vs_player_match_performance(id)?;
        let mut missing = self.0.players().keys().copied().collect::<HashSet<_>>();
        missing.remove(&id);
        let players = iter
            .map(|(player, perf)| {
                missing.remove(&player.id());
                (player, perf)
            })
            .collect::<Vec<_>>();

        Ok(chain!(
            players,
            missing
                .into_iter()
                .filter_map(|id| self.0.get_registered_player(id))
                .map(|player| (player, MatchPerformance::default()))
        ))
    }

    pub fn nemesis(
        self,
        id: u32,
    ) -> Result<impl Iterator<Item = (RegisteredPlayer<'a>, MatchPerformance)> + 'a, TournamentError>
    {
        let iter = self.player_match_performance(id)?;
        let elo_base = self.0.get_player_or_default_stats(id).elo();
        let sorted = iter.sorted_by(|(player_a, perf_a), (player_b, perf_b)| {
            perf_a
                .cmp(perf_b)
                .then_with(|| closest_elo(elo_base, player_a, player_b))
        });

        Ok(sorted)
    }

    pub fn lost_with(
        self,
        id: u32,
    ) -> Result<impl Iterator<Item = (RegisteredPlayer<'a>, MatchPerformance)> + 'a, TournamentError>
    {
        let iter = self.player_match_performance(id)?;
        let elo_base = self.0.get_player_or_default_stats(id).elo();
        let sorted = iter.sorted_by(|(player_a, perf_a), (player_b, perf_b)| {
            let score_a = perf_a.draws();
            let score_b = perf_b.draws();

            score_a
                .cmp(&score_b)
                .reverse()
                .then_with(|| closest_elo(elo_base, player_a, player_b))
        });
        Ok(sorted)
    }

    pub fn least_played(
        self,
        id: u32,
    ) -> Result<impl Iterator<Item = (RegisteredPlayer<'a>, MatchPerformance)> + 'a, TournamentError>
    {
        let iter = self.player_match_performance(id)?;
        let elo_base = self.0.get_player_or_default_stats(id).elo();
        let sorted = iter.sorted_by(|(player_a, perf_a), (player_b, perf_b)| {
            let score_a = perf_a.played();
            let score_b = perf_b.played();

            score_a
                .cmp(&score_b)
                .then_with(|| closest_elo(elo_base, player_a, player_b))
        });
        Ok(sorted)
    }
}
