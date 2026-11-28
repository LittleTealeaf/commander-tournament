use crate::player::{PlayerId, stats::PlayerStats};

#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    getset::Getters,
    getset::CopyGetters,
    derive_more::Constructor,
)]
pub struct MatchPlayer {
    #[get_copy = "pub"]
    id: PlayerId,
    #[get = "pub"]
    stats: PlayerStats,
    #[get = "pub"]
    expected: f64,
    #[get = "pub"]
    elo_win: f64,
    #[get = "pub"]
    elo_loss: f64,
}
