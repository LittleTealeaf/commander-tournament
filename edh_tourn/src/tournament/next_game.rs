use std::collections::HashSet;

use crate::{
    error::TournamentError,
    game::{matchup::Matchup, next_mode::NextPlayerMode},
    player::PlayerId,
    tournament::Tournament,
};

impl Tournament {
    pub fn next_game(&self, mode: NextPlayerMode) -> Result<Matchup, TournamentError> {
        self.matchmaker().create_match(
            self.next_game_player(mode)
                .ok_or(TournamentError::NotEnoughPlayers)?,
        )
    }

    fn next_game_player(&self, mode: NextPlayerMode) -> Option<PlayerId> {
        let players = self
            .registered_players()
            .filter(|player| (!player.info().is_archived()) && (!player.info().is_precon()));
        match mode {
            NextPlayerMode::LongestBreak => {
                let mut pool = players.map(|player| player.id()).collect::<HashSet<_>>();

                if pool.len() <= 1 {
                    return pool.into_iter().next();
                }

                let games = self.games().iter().rev();

                for game in games {
                    for player in game.players() {
                        pool.remove(&player.id());
                        if pool.len() <= 1 {
                            return pool.into_iter().next();
                        }
                    }
                }

                pool.into_iter().min()
            }
            NextPlayerMode::LeastPlayed => Some(
                players
                    .min_by_key(|player| (player.stats().games(), player.id()))?
                    .id(),
            ),
        }
    }
}
