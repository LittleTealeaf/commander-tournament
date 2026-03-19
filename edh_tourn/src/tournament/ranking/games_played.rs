use core::cmp::Ordering;

use itertools::Itertools;

use crate::{
    analytics::winloss::MatchPerformance, error::TournamentError, player::RegisteredPlayer,
    tournament::Tournament,
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

impl Tournament {
    pub fn get_player_ranked_nemesis(
        &self,
        id: u32,
    ) -> Result<impl Iterator<Item = (RegisteredPlayer<'_>, MatchPerformance)>, TournamentError>
    {
        let iter = self.get_player_player_match_performance(id)?;
        let elo_base = self.get_player_or_default_stats(id).elo();
        let sorted = iter.sorted_by(|(player_a, perf_a), (player_b, perf_b)| {
            let total_games = perf_a.played().max(perf_b.played());
            let score_a = total_games + perf_a.wins() - perf_a.losses();
            let score_b = total_games + perf_b.wins() - perf_b.losses();

            score_a
                .cmp(&score_b)
                .then_with(|| closest_elo(elo_base, player_a, player_b))
        });

        Ok(sorted)
    }

    pub fn get_player_ranked_lost_with(
        &self,
        id: u32,
    ) -> Result<impl Iterator<Item = (RegisteredPlayer<'_>, MatchPerformance)>, TournamentError>
    {
        let iter = self.get_player_player_match_performance(id)?;
        let elo_base = self.get_player_or_default_stats(id).elo();
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

    pub fn get_player_ranked_least_played(
        &self,
        id: u32,
    ) -> Result<impl Iterator<Item = (RegisteredPlayer<'_>, MatchPerformance)>, TournamentError>
    {
        let iter = self.get_player_player_match_performance(id)?;
        let elo_base = self.get_player_or_default_stats(id).elo();
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
