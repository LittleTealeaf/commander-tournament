use std::collections::HashMap;

use itertools::Itertools;

use crate::{
    analytics::winloss::MatchPerformance,
    error::TournamentError,
    player::{
        RegisteredPlayer,
        color::{ColorIdentity, MtgColor},
    },
    tournament::Tournament,
};

impl Tournament {
    fn player_get_all_players_match_performance(
        &self,
        id: u32,
    ) -> Result<impl Iterator<Item = (RegisteredPlayer<'_>, MatchPerformance)>, TournamentError>
    {
        self.require_id_registered(id)?;

        Ok(self
            .get_player_games(id)?
            .flat_map(|game| {
                let winner = game.winner();
                let [loser_a, loser_b, loser_c] = game.losers();

                [
                    (winner == id).then_some([
                        (loser_a, MatchPerformance::WIN),
                        (loser_b, MatchPerformance::WIN),
                        (loser_c, MatchPerformance::WIN),
                    ]),
                    (loser_a == id).then_some([
                        (winner, MatchPerformance::LOSS),
                        (loser_b, MatchPerformance::DRAW),
                        (loser_c, MatchPerformance::DRAW),
                    ]),
                    (loser_b == id).then_some([
                        (winner, MatchPerformance::LOSS),
                        (loser_a, MatchPerformance::DRAW),
                        (loser_c, MatchPerformance::DRAW),
                    ]),
                    (loser_c == id).then_some([
                        (winner, MatchPerformance::LOSS),
                        (loser_a, MatchPerformance::DRAW),
                        (loser_b, MatchPerformance::DRAW),
                    ]),
                ]
            })
            .flatten()
            .flatten()
            .into_grouping_map()
            .sum()
            .into_iter()
            .filter_map(|(id, perf)| Some((self.get_registered_player(id)?, perf))))
    }

    pub fn get_player_player_match_performance(
        &self,
        id: u32,
    ) -> Result<impl Iterator<Item = (RegisteredPlayer<'_>, MatchPerformance)>, TournamentError>
    {
        let players = self.player_get_all_players_match_performance(id)?;
        let filtered = players.filter(move |(player, _)| player.id() != id);
        Ok(filtered)
    }

    pub fn get_player_identity_match_performance(
        &self,
        id: u32,
    ) -> Result<HashMap<ColorIdentity, MatchPerformance>, TournamentError> {
        Ok(self
            .player_get_all_players_match_performance(id)?
            .map(|(player, perf)| (*player.info().color_identity(), perf))
            .into_grouping_map()
            .sum())
    }

    pub fn get_player_color_match_performance(
        &self,
        id: u32,
    ) -> Result<HashMap<MtgColor, MatchPerformance>, TournamentError> {
        Ok(self
            .player_get_all_players_match_performance(id)?
            .flat_map(|(player, perf)| {
                player
                    .info()
                    .color_identity()
                    .into_colors()
                    .map(move |color| (color, perf))
            })
            .into_grouping_map()
            .sum())
    }
}
