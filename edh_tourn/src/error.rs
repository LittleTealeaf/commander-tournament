use thiserror::Error;

use crate::player::PlayerId;

#[derive(Error, Debug, Clone)]
pub enum TournamentError {
    #[error("Player is not in the match: {0}")]
    PlayerNotInMatch(PlayerId),
    #[error("Player name is not registered: {0}")]
    PlayerNameNotRegistered(String),
    #[error("Player ID is not valid: {0}")]
    InvalidPlayerId(PlayerId),
    #[error("Player name is already registered: {0}, id {1}")]
    PlayerAlreadyRegistered(String, PlayerId),
    #[error("Invalid Game: {0}")]
    GameNotFound(usize),
    #[error("Not enough players")]
    NotEnoughPlayers,
    #[error("Player name is invalid: '{0}'")]
    InvalidPlayerName(String),
    #[error("Record has no elo data")]
    RecordNoEloData,
}

pub type TournResult<T> = Result<T, TournamentError>;
