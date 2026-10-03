use backwards_compat::backwards_compat;

use crate::config::matchmaker::MatchmakerConfig;

backwards_compat! {
    #[tag="v", version=1]
    compat MatchmakerConfig {
        1: V1MatchmakerConfig
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, derive_more::Constructor)]
pub struct V1MatchmakerConfig {
    elo_range: f64,
    min_pool_size: usize,
}

impl Default for V1MatchmakerConfig {
    fn default() -> Self {
        Self {
            elo_range: 120.0,
            min_pool_size: 6,
        }
    }
}

impl From<MatchmakerConfig> for V1MatchmakerConfig {
    fn from(value: MatchmakerConfig) -> Self {
        Self {
            elo_range: value.elo_range(),
            min_pool_size: value.min_pool_size(),
        }
    }
}

impl From<V1MatchmakerConfig> for MatchmakerConfig {
    fn from(value: V1MatchmakerConfig) -> Self {
        Self::new(value.elo_range, value.min_pool_size)
    }
}
