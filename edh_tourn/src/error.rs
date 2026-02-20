use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum TournamentError {
    #[error("Player is not in the match: {0}")]
    WinnerNotInMatch(u32),
    #[error("Player name is not registered: {0}")]
    PlayerNameNotRegistered(String),
    #[error("Player ID is not valid: {0}")]
    InvalidPlayerId(u32),
    #[error("Player name is already registered: {0}, id {1}")]
    PlayerAlreadyRegistered(String, u32),
    #[error("Invalid Game: {0}")]
    GameNotFound(usize),
    #[error("Not enough players")]
    NotEnoughPlayers,
    #[error("Player name is invalid: '{0}'")]
    InvalidPlayerName(String),
}

pub type TournResult<T> = Result<T, TournamentError>;
