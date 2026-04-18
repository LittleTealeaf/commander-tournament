use crate::player::{PlayerId, stats::PlayerStats};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct MatchPlayer {
    id: PlayerId,
    stats: PlayerStats,
    expected: f64,
    elo_win: f64,
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

    #[must_use]
    pub const fn id(&self) -> PlayerId {
        self.id
    }

    #[must_use]
    pub const fn stats(&self) -> &PlayerStats {
        &self.stats
    }

    #[must_use]
    pub const fn expected(&self) -> &f64 {
        &self.expected
    }

    #[must_use]
    pub const fn elo_win(&self) -> &f64 {
        &self.elo_win
    }

    #[must_use]
    pub const fn elo_loss(&self) -> &f64 {
        &self.elo_loss
    }
}
