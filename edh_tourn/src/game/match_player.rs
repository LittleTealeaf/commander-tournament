use crate::player::{PlayerId, stats::PlayerStats};

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, getset::Getters, getset::CopyGetters,
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

impl MatchPlayer {
    pub(crate) const fn new(
        id: PlayerId,
        stats: PlayerStats,
        expected: f64,
        elo_win: f64,
        elo_loss: f64,
    ) -> Self {
        Self {
            id,
            stats,
            expected,
            elo_win,
            elo_loss,
        }
    }
}
