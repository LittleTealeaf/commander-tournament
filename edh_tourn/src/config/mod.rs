use crate::config::{game::GameConfig, matchmaker::MatchmakerConfig};

pub mod game;
pub mod matchmaker;

#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Default,
    getset::Getters,
    getset::Setters,
    getset::WithSetters,
    derive_more::Constructor,
)]
#[getset(set = "pub", get = "pub", set_with = "pub")]
pub struct TournamentConfig {
    #[serde(default)]
    game: GameConfig,
    #[serde(default)]
    matchmaker: MatchmakerConfig,
}
