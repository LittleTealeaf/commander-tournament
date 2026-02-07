#[derive(thiserror::Error, Debug)]
#[allow(dead_code)]
pub enum AppError {
    #[error("Missing Player: {0}")]
    MissingPlayer(usize),
}
