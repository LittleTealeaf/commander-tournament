use crate::config::{game::GameConfig, matchmaker::MatchmakerConfig, ranking::RankingConfig};

pub mod game;
pub mod ranking;
pub mod matchmaker;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Default)]
pub struct TournamentConfig {
    pub(crate) game: GameConfig,
    pub(crate) ranking: RankingConfig,
    pub(crate) matchmaker: MatchmakerConfig
}

impl TournamentConfig {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            game: GameConfig::new(),
            ranking: RankingConfig::new(),
            matchmaker: MatchmakerConfig::new()
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

    #[must_use]
    pub const fn matchmaker_config(&self) -> &MatchmakerConfig {
        &self.matchmaker
    }
}
