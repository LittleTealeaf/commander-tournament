use crate::config::{game::GameConfig, ranking::RankingConfig};

pub mod game;
pub mod ranking;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Default)]
pub struct TournamentConfig {
    pub(crate) game: GameConfig,
    pub(crate) ranking: RankingConfig,
}

impl TournamentConfig {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            game: GameConfig::new(),
            ranking: RankingConfig::new(),
        }
    }

    #[must_use]
    pub const fn game_config(&self) -> &GameConfig {
        &self.game
    }

    #[must_use]
    pub const fn ranking_config(&self) -> &RankingConfig {
        &self.ranking
    }
}
