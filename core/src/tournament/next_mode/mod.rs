use std::collections::HashSet;

use self::streak::calculate_streak;
use crate::{player::PlayerId, tournament::Tournament};

pub(crate) mod streak;

#[derive(Debug, Copy, Clone, PartialEq, Eq, derive_more::Display, Default, strum::VariantArray)]
pub enum NextPlayerMode {
    #[display("Default")]
    #[default]
    Default,
    #[display("Longest Break")]
    LongestBreak,
    #[display("Least Played")]
    LeastPlayed,
    #[display("Win Streak")]
    WinStreak,
    #[display("Loss Streak")]
    LossStreak,
    #[display("Game Streak")]
    GameStreak,
}

impl NextPlayerMode {
    #[must_use]
    pub fn select_player(&self, tournament: &Tournament) -> Option<PlayerId> {
        let players = tournament
            .registered_players()
            .filter(|player| (!player.info().is_archived()) && (!player.info().is_precon()));

        match self {
            Self::Default => {
                let pool = players.collect::<Vec<_>>();
                // First: Check if anyone is still low ranked.
                {
                    let min_games = pool
                        .iter()
                        .map(|player| (player.stats().games(), player.id()))
                        .min();

                    if let Some((games, player)) = min_games
                        && games < tournament.config.game().calibration_games()
                    {
                        return Some(player);
                    }
                }
                // Then fall back to longest break
                {
                    let mut pool = pool.into_iter().map(|p| p.id()).collect::<HashSet<_>>();

                    if pool.len() <= 1 {
                        return pool.into_iter().next();
                    }

                    let games = tournament.games().iter().rev();

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
            Self::LongestBreak => {
                let mut pool = players.map(|player| player.id()).collect::<HashSet<_>>();

                if pool.len() <= 1 {
                    return pool.into_iter().next();
                }

                let games = tournament.games().iter().rev();

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
            Self::LeastPlayed => Some(
                players
                    .min_by_key(|player| (player.stats().games(), player.id()))?
                    .id(),
            ),

            Self::WinStreak => calculate_streak(players, tournament.games(), Some(true)),
            Self::GameStreak => calculate_streak(players, tournament.games(), None),
            Self::LossStreak => calculate_streak(players, tournament.games(), Some(false)),
        }
    }
}
