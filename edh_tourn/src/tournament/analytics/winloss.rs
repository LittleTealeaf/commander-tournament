use std::collections::HashMap;

use itertools::{Itertools, chain};

use crate::{
    analytics::winloss::MatchPerformance,
    error::TournamentError,
    player::{
        RegisteredPlayer,
        color::{ColorIdentity, MtgColor},
    },
    tournament::analytics::Analytics,
};

impl<'a> Analytics<'a> {
    fn player_get_all_players_match_performance(
        self,
        id: u32,
    ) -> Result<impl Iterator<Item = (RegisteredPlayer<'a>, MatchPerformance)> + 'a, TournamentError>
    {
        self.tourn.require_id_registered(id)?;

        Ok(self
            .tourn
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
            .filter_map(|(id, perf)| Some((self.tourn.get_registered_player(id)?, perf))))
    }

    pub fn player_vs_player_performance(
        self,
        id: u32,
    ) -> Result<impl Iterator<Item = (RegisteredPlayer<'a>, MatchPerformance)> + 'a, TournamentError>
    {
        let players = self.player_get_all_players_match_performance(id)?;
        let filtered = players.filter(move |(player, _)| player.id() != id);
        Ok(filtered)
    }

    pub fn player_vs_identity_performance(
        self,
        id: u32,
    ) -> Result<HashMap<ColorIdentity, MatchPerformance>, TournamentError> {
        Ok(self
            .player_get_all_players_match_performance(id)?
            .map(|(player, perf)| (player.info().color_identity(), perf))
            .into_grouping_map()
            .sum())
    }

    pub fn player_vs_color_performance(
        self,
        id: u32,
    ) -> Result<HashMap<MtgColor, MatchPerformance>, TournamentError> {
        Ok(self
            .player_get_all_players_match_performance(id)?
            .flat_map(|(player, perf)| {
                player
                    .info()
                    .color_identity()
                    .colors()
                    .map(move |color| (color, perf))
            })
            .into_grouping_map()
            .sum())
    }
}

impl Analytics<'_> {
    #[must_use]
    pub fn identity_vs_identity_performance(
        self,
        identity: ColorIdentity,
    ) -> HashMap<ColorIdentity, MatchPerformance> {
        let identity_map = self
            .tourn
            .get_registered_players()
            .map(|player| (player.id(), player.info().color_identity()))
            .collect::<HashMap<_, _>>();

        self.tourn
            .games()
            .iter()
            .flat_map(|game| {
                let winner = identity_map.get(&game.winner()).copied();
                let [loser_a, loser_b, loser_c] =
                    game.losers().map(|id| identity_map.get(&id).copied());

                [
                    (winner == Some(identity)).then_some([
                        (loser_a, MatchPerformance::WIN),
                        (loser_b, MatchPerformance::WIN),
                        (loser_c, MatchPerformance::WIN),
                    ]),
                    (loser_a == Some(identity)).then_some([
                        (winner, MatchPerformance::LOSS),
                        (loser_b, MatchPerformance::DRAW),
                        (loser_c, MatchPerformance::DRAW),
                    ]),
                    (loser_b == Some(identity)).then_some([
                        (winner, MatchPerformance::LOSS),
                        (loser_a, MatchPerformance::DRAW),
                        (loser_c, MatchPerformance::DRAW),
                    ]),
                    (loser_c == Some(identity)).then_some([
                        (winner, MatchPerformance::LOSS),
                        (loser_a, MatchPerformance::DRAW),
                        (loser_b, MatchPerformance::DRAW),
                    ]),
                ]
            })
            .flatten()
            .flatten()
            .filter_map(|(ident, perf)| Some((ident?, perf)))
            .into_grouping_map()
            .sum()
    }

    #[must_use]
    pub fn identity_vs_color_performance(
        self,
        identity: ColorIdentity,
    ) -> HashMap<MtgColor, MatchPerformance> {
        self.identity_vs_identity_performance(identity)
            .into_iter()
            .flat_map(|(identity, perf)| identity.colors().map(move |color| (color, perf)))
            .into_grouping_map()
            .sum()
    }
}

impl Analytics<'_> {
    #[must_use]
    pub fn color_vs_color_performance(
        self,
        color: MtgColor,
    ) -> HashMap<MtgColor, MatchPerformance> {
        let get_identity = |id: u32| -> ColorIdentity {
            let Some(info) = self.tourn.get_player_info(&id) else {
                return ColorIdentity::COLORLESS;
            };
            info.color_identity()
        };

        self.tourn
            .games()
            .iter()
            .flat_map(|game| {
                let winner = get_identity(game.winner());
                let [loser_a, loser_b, loser_c] = game.losers().map(&get_identity);

                chain!(
                    winner.has_color(color).then_some([
                        (loser_a, MatchPerformance::WIN),
                        (loser_b, MatchPerformance::WIN),
                        (loser_c, MatchPerformance::WIN),
                    ]),
                    loser_a.has_color(color).then_some([
                        (winner, MatchPerformance::LOSS),
                        (loser_b, MatchPerformance::DRAW),
                        (loser_c, MatchPerformance::DRAW),
                    ]),
                    loser_b.has_color(color).then_some([
                        (winner, MatchPerformance::LOSS),
                        (loser_a, MatchPerformance::DRAW),
                        (loser_c, MatchPerformance::DRAW),
                    ]),
                    loser_c.has_color(color).then_some([
                        (winner, MatchPerformance::LOSS),
                        (loser_a, MatchPerformance::DRAW),
                        (loser_b, MatchPerformance::DRAW),
                    ])
                )
                .flatten()
            })
            .flat_map(|(identity, perf)| identity.colors().map(move |color| (color, perf)))
            .into_grouping_map()
            .sum()
    }
}
