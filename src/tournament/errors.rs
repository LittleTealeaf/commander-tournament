use thiserror::Error;

#[derive(Error, Debug)]
pub enum TournamentError {
    #[error("Player is not in the match: {0}")]
    WinnerNotInMatch(String),
    #[error("Player not registered: {0}")]
    PlayerNotRegistered(String),
    #[error("Player already registered: {0}")]
    PlayerAlreadyRegistered(String),
    #[error("Not enough players. Need at least 4 to create game")]
    #[allow(dead_code)]
    NotEnoughPlayers,
}
