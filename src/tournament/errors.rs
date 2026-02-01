use thiserror::Error;

#[derive(Error, Debug)]
pub enum TournamentError {
    #[error("Invalid player: {0}")]
    InvalidPlayer(String),
}
