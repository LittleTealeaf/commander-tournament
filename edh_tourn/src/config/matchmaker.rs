use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MatchmakerConfig {
    pub player_least_played: usize,
    pub player_lost_with: usize,
    pub player_nemesis: usize,
    pub elo_neighbor: usize,
    pub wr_neighbor: usize,
    pub expected_neighbor: usize,
    pub exclude_precons: bool,
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
            expected_neighbor: 3,
            exclude_precons: false,
        }
    }
}

impl Default for MatchmakerConfig {
    fn default() -> Self {
        Self::new()
    }
}
