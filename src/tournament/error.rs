use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum TournamentError {
    #[error("Player is not in the match: {0}")]
    WinnerNotInMatch(String),
    #[error("Player name is not registered: {0}")]
    PlayerNameNotRegistered(String),
    #[error("Player ID is not valid: {0}")]
    InvalidPlayerId(usize),
    #[error("Player name is already registered: {0}")]
    PlayerAlreadyRegistered(String),
    #[error("Invalid Game: {0}")]
    GameNotFound(usize),
    #[error("Not enough players")]
    NotEnoughPlayers,
}
