use crate::config::{game::GameConfig, matchmaker::MatchmakerConfig};

pub mod game;
pub mod matchmaker;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Default)]
pub struct TournamentConfig {
    #[serde(default)]
    pub(crate) game: GameConfig,
    #[serde(default)]
    pub(crate) matchmaker: MatchmakerConfig,
}

impl TournamentConfig {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            game: GameConfig::new(),
            matchmaker: MatchmakerConfig::new(),
        }
    }

    #[must_use]
    pub const fn game_config(&self) -> &GameConfig {
        &self.game
    }

    #[must_use]
    pub const fn matchmaker_config(&self) -> &MatchmakerConfig {
        &self.matchmaker
    }
}
