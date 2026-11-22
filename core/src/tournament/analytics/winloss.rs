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

impl<'a, F> Analytics<'a, F>
where
    F: Fn(PlayerId) -> bool,
{
    pub(crate) fn player_performance(
        &self,
        id: PlayerId,
    ) -> Result<HashMap<PlayerId, MatchPerformance>, TournamentError> {
        self.tourn.require_id_registered(id)?;

        let games = self.tourn.get_player_games(id)?;
        let performances = games.flat_map(|game| match_performances(game, |i| i == id));
        let filtered_performances = performances.filter(|(id, _)| (self.pool)(*id));
        Ok(filtered_performances.into_grouping_map().sum())
    }

    pub(crate) fn player_performance_all(
        &self,
        id: PlayerId,
    ) -> Result<HashMap<PlayerId, MatchPerformance>, TournamentError> {
        let mut map = self.player_performance(id)?;
        for id in self.validated_pool() {
            map.entry(id).or_default();
        }
        Ok(map)
    }

    pub(crate) fn player_performance_all_others(
        &self,
        id: PlayerId,
    ) -> Result<HashMap<PlayerId, MatchPerformance>, TournamentError> {
        let mut map = self.player_performance_all(id)?;
        map.remove(&id);
        Ok(map)
    }

    pub(crate) fn player_vs_player_all_performances(
        &self,
        id: PlayerId,
    ) -> Result<impl Iterator<Item = (RegisteredPlayer<'a>, MatchPerformance)> + 'a, TournamentError> {
        Ok(self
            .player_performance(id)?
            .into_iter()
            .filter_map(|(id, perf)| Some((self.tourn.get_registered_player(id)?, perf))))
    }

    pub fn player_vs_player_performance(
        &self,
        id: PlayerId,
    ) -> Result<impl Iterator<Item = (RegisteredPlayer<'a>, MatchPerformance)> + 'a, TournamentError> {
        let players = self.player_vs_player_all_performances(id)?;
        let filtered = players.filter(move |(player, _)| player.id() != id);
        Ok(filtered)
    }

    pub fn player_vs_identity_performance(
        &self,
        id: PlayerId,
    ) -> Result<HashMap<ColorIdentity, MatchPerformance>, TournamentError> {
        Ok(self
            .player_vs_player_all_performances(id)?
            .map(|(player, perf)| (player.info().color_identity(), perf))
            .into_grouping_map()
            .sum())
    }

    pub fn player_vs_color_performance(
        &self,
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

impl<F> Analytics<'_, F>
where
    F: Fn(PlayerId) -> bool,
{
    #[must_use]
    pub fn identity_vs_identity_performance(
        &self,
        identity: ColorIdentity,
    ) -> HashMap<ColorIdentity, MatchPerformance> {
        let players = self
            .tourn
            .players()
            .iter()
            .filter(|(id, info)| info.color_identity() == identity && (self.pool)(**id))
            .map(|(id, _)| *id)
            .collect::<im::HashSet<PlayerId>>();

        self.tourn
            .games()
            .iter()
            .flat_map(|game| match_performances(game, |id| players.contains(&id)))
            .filter_map(|(id, perf)| {
                let info = self.tourn.get_player_info(&id)?;
                (self.pool)(id).then_some((info.color_identity(), perf))
            })
            .into_grouping_map()
            .sum()
    }

    #[must_use]
    pub fn identity_vs_color_performance(
        &self,
        identity: ColorIdentity,
    ) -> HashMap<MtgColor, MatchPerformance> {
        self.identity_vs_identity_performance(identity)
            .into_iter()
            .flat_map(|(identity, perf)| identity.colors().map(move |color| (color, perf)))
            .into_grouping_map()
            .sum()
    }
}

impl<F> Analytics<'_, F>
where
    F: Fn(PlayerId) -> bool,
{
    #[must_use]
    pub fn color_vs_color_performance(&self, color: MtgColor) -> HashMap<MtgColor, MatchPerformance> {
        let players = self
            .tourn
            .players()
            .iter()
            .filter(|(id, info)| (self.pool)(**id) && info.color_identity().has_color(color))
            .map(|(id, _)| *id)
            .collect::<im::HashSet<PlayerId>>();

        self.tourn
            .games()
            .iter()
            .flat_map(|game| match_performances(game, |id| players.contains(&id)))
            .filter_map(|(id, perf)| {
                let info = self.tourn.get_player_info(&id)?;
                (self.pool)(id).then_some((info.color_identity(), perf))
            })
            .flat_map(|(identity, perf)| identity.colors().map(move |color| (color, perf)))
            .into_grouping_map()
            .sum()
    }
}

fn match_performances<F>(
    game: &GameRecord,
    predicate: F,
) -> impl Iterator<Item = (PlayerId, MatchPerformance)>
where
    F: Fn(PlayerId) -> bool,
{
    let players = game.ids();
    let winner = game.winner();

    let index_winner = players
        .iter()
        .find_position(|i| *i == &winner)
        .map_or_default(|(i, _)| i);

    players
        .into_iter()
        .enumerate()
        .filter(move |(_, id)| predicate(*id))
        .flat_map(move |(i, _)| {
            game.ids()
                .into_iter()
                .enumerate()
                .filter(move |(j, _)| &i != j)
                .map(move |(j, id)| {
                    if i == index_winner {
                        (id, MatchPerformance::WIN)
                    } else if j == index_winner {
                        (id, MatchPerformance::LOSS)
                    } else {
                        (id, MatchPerformance::DRAW)
                    }
                })
        })
}
