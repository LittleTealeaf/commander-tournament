use crate::{player::PlayerId, tournament::Tournament};

#[derive(Debug, Copy, Clone, PartialEq, Eq, derive_more::Display, Default, strum::VariantArray)]
pub enum NextPlayerMode {
    #[display("Default")]
    #[default]
    /// Default:
    /// - If there are players in calibration, selects the player with the least games
    /// - Otherwise grabs the player who has not played the longest
    Default,
    /// Least number of games played
    #[display("Least Games")]
    LeastGames,
    /// Player who has not played the longest
    #[display("Longest Break")]
    LongestBreak,
}

impl NextPlayerMode {
    #[must_use]
    pub fn next_player(self, tourn: &Tournament) -> Option<PlayerId> {
        let players = tourn
            .registered_players()
            .filter(|player| !player.info().is_archived() && (!player.info().is_precon()));

        match self {
            Self::Default => {
                let pool = players.collect::<Vec<_>>();
                // First check least games
                {
                    if let Some(player) = pool
                        .iter()
                        .min_by_key(|player| (player.stats().games(), player.id()))
                        && player.stats().games() < tourn.game_config().calibration_games
                    {
                        return Some(player.id());
                    }
                }

                {
                    let mut pool = pool.into_iter().map(|p| p.id()).collect::<im::HashSet<_>>();

                    if pool.len() <= 1 {
                        return pool.into_iter().next();
                    }

                    let games = tourn.games().iter().rev();

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
            }
            Self::LeastGames => players
                .min_by_key(|player| (player.stats().games(), player.id()))
                .map(|player| player.id()),
            Self::LongestBreak => {
                let mut pool = players.map(|p| p.id()).collect::<im::HashSet<_>>();

                if pool.len() <= 1 {
                    return pool.into_iter().next();
                }

                let games = tourn.games().iter().rev();

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
        }
    }
}
