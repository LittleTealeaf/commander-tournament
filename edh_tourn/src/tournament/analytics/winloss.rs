use std::collections::HashMap;

use itertools::Itertools;

use crate::{
    analytics::winloss::MatchPerformance,
    error::TournamentError,
    game::record::GameRecord,
    player::{
        PlayerId, RegisteredPlayer,
        color::{ColorIdentity, MtgColor},
    },
    tournament::analytics::Analytics,
};

impl<'a> Analytics<'a> {
    pub(crate) fn player_performance(
        self,
        id: PlayerId,
    ) -> Result<HashMap<PlayerId, MatchPerformance>, TournamentError> {
        self.0.require_id_registered(id)?;

        Ok(self
            .0
            .get_player_games(id)?
            .flat_map(|game| game_match_perfs(game, |i| i == id))
            .into_grouping_map()
            .sum())
    }

    pub(crate) fn player_performance_all(
        self,
        id: PlayerId,
    ) -> Result<HashMap<PlayerId, MatchPerformance>, TournamentError> {
        let mut map = self.player_performance(id)?;
        for id in self.0.players().keys() {
            map.entry(*id).or_default();
        }
        Ok(map)
    }

    pub(crate) fn player_performance_all_others(
        self,
        id: PlayerId,
    ) -> Result<HashMap<PlayerId, MatchPerformance>, TournamentError> {
        let mut map = self.player_performance_all(id)?;
        map.remove(&id);
        Ok(map)
    }

    pub(crate) fn player_vs_player_all_performances(
        self,
        id: PlayerId,
    ) -> Result<impl Iterator<Item = (RegisteredPlayer<'a>, MatchPerformance)> + 'a, TournamentError>
    {
        Ok(self
            .player_performance(id)?
            .into_iter()
            .filter_map(|(id, perf)| Some((self.0.get_registered_player(id)?, perf))))
    }

    pub fn player_vs_player_performance(
        self,
        id: PlayerId,
    ) -> Result<impl Iterator<Item = (RegisteredPlayer<'a>, MatchPerformance)> + 'a, TournamentError>
    {
        let players = self.player_vs_player_all_performances(id)?;
        let filtered = players.filter(move |(player, _)| player.id() != id);
        Ok(filtered)
    }

    pub fn player_vs_identity_performance(
        self,
        id: PlayerId,
    ) -> Result<HashMap<ColorIdentity, MatchPerformance>, TournamentError> {
        Ok(self
            .player_vs_player_all_performances(id)?
            .map(|(player, perf)| (player.info().color_identity(), perf))
            .into_grouping_map()
            .sum())
    }

    pub fn player_vs_color_performance(
        self,
        id: PlayerId,
    ) -> Result<HashMap<MtgColor, MatchPerformance>, TournamentError> {
        Ok(self
            .player_vs_player_all_performances(id)?
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
        self.0
            .games()
            .iter()
            .flat_map(|game| {
                game_match_perfs(game, |id| {
                    let Some(info) = self.0.get_player_info(&id) else {
                        return false;
                    };
                    info.color_identity() == identity
                })
            })
            .filter_map(|(id, perf)| Some((self.0.get_player_info(&id)?.color_identity(), perf)))
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
        self.0
            .games()
            .iter()
            .flat_map(|game| {
                game_match_perfs(game, |id| {
                    let Some(info) = self.0.get_player_info(&id) else {
                        return false;
                    };
                    info.color_identity().has_color(color)
                })
            })
            .filter_map(|(id, perf)| Some((self.0.get_player_info(&id)?.color_identity(), perf)))
            .flat_map(|(identity, perf)| identity.colors().map(move |color| (color, perf)))
            .into_grouping_map()
            .sum()
    }
}

fn game_match_perfs<F>(
    game: &GameRecord,
    check: F,
) -> impl Iterator<Item = (PlayerId, MatchPerformance)>
where
    F: Fn(PlayerId) -> bool,
{
    let winner = game.winner();
    let [loser_a, loser_b, loser_c] = game.losers();

    [
        check(winner).then_some([
            (loser_a, MatchPerformance::WIN),
            (loser_b, MatchPerformance::WIN),
            (loser_c, MatchPerformance::WIN),
        ]),
        check(loser_a).then_some([
            (winner, MatchPerformance::LOSS),
            (loser_b, MatchPerformance::DRAW),
            (loser_c, MatchPerformance::DRAW),
        ]),
        check(loser_b).then_some([
            (winner, MatchPerformance::LOSS),
            (loser_a, MatchPerformance::DRAW),
            (loser_c, MatchPerformance::DRAW),
        ]),
        check(loser_c).then_some([
            (winner, MatchPerformance::LOSS),
            (loser_a, MatchPerformance::DRAW),
            (loser_b, MatchPerformance::DRAW),
        ]),
    ]
    .into_iter()
    .flatten()
    .flatten()
}
