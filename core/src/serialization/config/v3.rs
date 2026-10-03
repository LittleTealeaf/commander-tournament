use crate::config::{TournamentConfig, game::GameConfig, matchmaker::MatchmakerConfig};

#[derive(Debug, serde::Serialize, serde::Deserialize, derive_more::Constructor, Default)]
pub struct V3TournamentConfig {
    game: GameConfig,
    matchmaker: MatchmakerConfig,
}

impl From<V3TournamentConfig> for TournamentConfig {
    fn from(value: V3TournamentConfig) -> Self {
        Self::new(value.game, value.matchmaker)
    }
}

impl From<TournamentConfig> for V3TournamentConfig {
    fn from(value: TournamentConfig) -> Self {
        Self {
            game: value.game().clone(),
            matchmaker: value.matchmaker().clone(),
        }
    }
}
