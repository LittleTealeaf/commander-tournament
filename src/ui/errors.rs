#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Missing Player: {0}")]
    MissingPlayer(usize),
}
