use crate::config::{game::GameConfig, matchmaker::MatchmakerConfig};

pub mod game;
pub mod matchmaker;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Default)]
pub struct TournamentConfig {
    #[serde(default)]
    game: GameConfig,
    #[serde(default)]
    matchmaker: MatchmakerConfig,
}

impl TournamentConfig {
    #[must_use]
    pub const fn with_configs(game: GameConfig, matchmaker: MatchmakerConfig) -> Self {
        Self { game, matchmaker }
    }

    #[must_use]
    pub const fn new() -> Self {
        Self::with_configs(GameConfig::new(), MatchmakerConfig::new())
    }

    #[must_use]
    pub const fn game_config(&self) -> &GameConfig {
        &self.game
    }

    #[must_use]
    pub const fn matchmaker_config(&self) -> &MatchmakerConfig {
        &self.matchmaker
    }

    pub const fn set_game_config(&mut self, config: GameConfig) {
        self.game = config;
    }

    pub const fn set_matchmaker_config(&mut self, config: MatchmakerConfig) {
        self.matchmaker = config;
    }
}
