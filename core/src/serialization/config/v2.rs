use crate::config::{TournamentConfig, game::GameConfig, matchmaker::MatchmakerConfig};

#[derive(Debug, serde::Serialize, serde::Deserialize, derive_more::Constructor)]
pub struct V2TournamentConfig {
    #[serde(default)]
    pub(crate) game: GameConfig,
    #[serde(default)]
    pub(crate) matchmaker: MatchmakerConfig,
}

impl From<TournamentConfig> for V2TournamentConfig {
    fn from(value: TournamentConfig) -> Self {
        Self {
            game: value.game().clone(),
            matchmaker: value.matchmaker().clone(),
        }
    }
}

impl From<V2TournamentConfig> for TournamentConfig {
    fn from(value: V2TournamentConfig) -> Self {
        Self::new(value.game, value.matchmaker)
    }
}
