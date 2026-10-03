use crate::{
    config::game::GameConfig,
    serialization::config::{matchmaker::V1MatchmakerConfig, v3::V3TournamentConfig},
};

#[derive(Debug, serde::Deserialize, derive_more::Constructor)]
pub struct V2TournamentConfig {
    #[serde(default)]
    pub(crate) game: GameConfig,
    #[serde(default)]
    pub(crate) matchmaker: V2TournamentMatchmakerConfig,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, derive_more::Constructor)]
pub struct V2TournamentMatchmakerConfig {
    elo_range: f64,
    min_pool_size: usize,
}

impl Default for V2TournamentMatchmakerConfig {
    fn default() -> Self {
        Self {
            elo_range: 120.0,
            min_pool_size: 6,
        }
    }
}

impl From<V2TournamentConfig> for V3TournamentConfig {
    fn from(value: V2TournamentConfig) -> Self {
        Self::new(
            value.game,
            V1MatchmakerConfig::new(value.matchmaker.elo_range, value.matchmaker.min_pool_size).into(),
        )
    }
}
