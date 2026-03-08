use std::collections::HashMap;

use itertools::Itertools;

use crate::{
    Tournament,
    error::TournamentError,
    game::{match_player::MatchPlayer, record::GameRecord},
    player::{RegisteredPlayer, ranking::with_tie_breaker},
};

#[derive(Debug)]
pub struct LeastPlayedEntry<'a> {
    rank: usize,
    player: RegisteredPlayer<'a>,
    games_played_against: usize,
}

impl<'a> From<LeastPlayedEntry<'a>> for u32 {
    fn from(value: LeastPlayedEntry<'a>) -> Self {
        value.player.id()
    }
}

impl<'a> LeastPlayedEntry<'a> {
    #[must_use]
    pub const fn rank(&self) -> usize {
        self.rank
    }

    #[must_use]
    pub const fn player(&self) -> &RegisteredPlayer<'a> {
        &self.player
    }

    #[must_use]
    pub const fn games_played_against(&self) -> usize {
        self.games_played_against
    }
}

impl Tournament {
    pub fn ranked_least_played(
        &self,
        id: u32,
    ) -> Result<impl Iterator<Item = LeastPlayedEntry<'_>>, TournamentError> {
        self.ensure_id_registered(id)?;

        let mut players = self
            .get_registered_players()
            .map(|player| (player.id(), (player, 0)))
            .collect::<HashMap<_, _>>();

        let games = self.games().iter().filter(|game| game.has_player(id));
        let game_players = games.flat_map(GameRecord::players).map(MatchPlayer::id);
        let game_counts = game_players.counts();

        for (id, game_count) in game_counts {
            players.entry(id).and_modify(|(_, games)| {
                *games += game_count;
            });
        }

        let (player, _) = players
            .remove(&id)
            .ok_or(TournamentError::InvalidPlayerId(id))?;

        let player_elo = player.stats().elo();

        // (player, games, elo_diff)
        let players_iter = players
            .into_values()
            .map(|(player, games)| (player, games, (player.stats().elo() - player_elo).abs()));

        let players_sorted = players_iter.sorted_by(|(id1, c1, elo1), (id2, c2, elo2)| {
            with_tie_breaker(c1.cmp(c2), || {
                with_tie_breaker(elo1.total_cmp(elo2), || id1.id().cmp(&id2.id()))
            })
        });

        Ok(players_sorted
            .enumerate()
            .map(
                |(rank, (player, games_played_against, _))| LeastPlayedEntry {
                    rank: rank + 1,
                    player,
                    games_played_against,
                },
            ))
    }
}

