use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct MatchmakerConfig {
    pub player_least_played: usize,
    pub player_lost_with: usize,
    pub player_nemesis: usize,
    pub elo_neighbor: usize,
    pub wr_neighbor: usize,
}

impl MatchmakerConfig {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            player_least_played: 4,
            player_nemesis: 3,
            player_lost_with: 2,
            elo_neighbor: 4,
            wr_neighbor: 3,
        }
    }
}

impl Default for MatchmakerConfig {
    fn default() -> Self {
        Self::new()
    }
}

