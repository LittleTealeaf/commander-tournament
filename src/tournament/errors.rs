use thiserror::Error;

#[derive(Error, Debug)]
pub enum TournamentError {
    #[error("Player is not in the match: {0}")]
    PlayerNotInMatch(String),
    #[error("Player not registered: {0}")]
    PlayerNotRegistered(String),
    #[error("Player already registered: {0}")]
    PlayerAlreadyRegistered(String)
}
