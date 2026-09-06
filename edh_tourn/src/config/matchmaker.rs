use serde::{Deserialize, Serialize};

use crate::game::POD_SIZE;

#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, getset::CopyGetters, getset::Setters, getset::WithSetters,
)]
pub struct MatchmakerConfig {
    #[getset(set = "pub", set_with = "pub", get_copy = "pub")]
    pub elo_range: f64,
    #[getset(set = "pub", set_with = "pub", get_copy = "pub")]
    pub min_pool_size: usize,
}

impl MatchmakerConfig {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            elo_range: 100.0,
            min_pool_size: (POD_SIZE - 1) * 3,
        }
    }
}

impl Default for MatchmakerConfig {
    fn default() -> Self {
        Self::new()
    }
}
