use crate::config::{game::GameConfig, matchmaker::MatchmakerConfig, ranking::RankingConfig};

pub mod game;
pub mod matchmaker;
pub mod ranking;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Default)]
pub struct TournamentConfig {
    #[serde(default)]
    pub(crate) game: GameConfig,
    #[serde(default)]
    pub(crate) ranking: RankingConfig,
    #[serde(default)]
    pub(crate) matchmaker: MatchmakerConfig,
}

impl TournamentConfig {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            game: GameConfig::new(),
            ranking: RankingConfig::new(),
            matchmaker: MatchmakerConfig::new(),
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
