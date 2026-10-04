use std::collections::HashMap;

use crate::tournament::serialize::player_info_deserialize;
use crate::{
    config::{TournamentConfig, game::GameConfig, matchmaker::MatchmakerConfig},
    game::entry::GameEntry,
    player::{PlayerId, info::PlayerInfo},
    tournament::serialize::SerializedTournament,
};

#[derive(Debug, serde::Deserialize)]
pub struct V5Tournament {
    #[serde(rename = "cfg", alias = "config")]
    pub(super) config: V5TournamentConfig,
    #[serde(
        deserialize_with = "player_info_deserialize",
        serialize_with = "crate::utils::ordered_map",
        rename = "pls",
        alias = "players"
    )]
    pub(super) players: HashMap<PlayerId, PlayerInfo>,
    #[serde(rename = "gms", alias = "games")]
    pub(super) games: Vec<GameEntry>,
}

impl From<V5Tournament> for SerializedTournament {
    fn from(value: V5Tournament) -> Self {
        Self::new(
            TournamentConfig::new(
                GameConfig::default(),
                MatchmakerConfig::new(
                    value.config.matchmaker.elo_range,
                    value.config.matchmaker.min_pool_size,
                ),
            ),
            value.players,
            value.games,
        )
    }
}

#[derive(Debug, serde::Deserialize, derive_more::Constructor)]
pub(super) struct V5TournamentConfig {
    #[serde(default)]
    game: V5GameConfig,
    #[serde(default)]
    matchmaker: V5MatchmakerConfig,
}

#[derive(Debug, serde::Deserialize, Default)]
pub(super) struct V5GameConfig {
    starting_elo: f64,
    game_points: f64,
    game_elo_pow_scale: f64,
    game_wr_pow_scale: f64,
    game_elo_weight: f64,
    game_wr_weight: f64,
}

#[derive(Debug, serde::Deserialize)]
pub(super) struct V5MatchmakerConfig {
    elo_range: f64,
    min_pool_size: usize,
}

impl Default for V5MatchmakerConfig {
    fn default() -> Self {
        Self {
            elo_range: 50.0,
            min_pool_size: 9,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tournament::Tournament;

    #[test]
    fn deserialize() {
        let data = include_str!("../../../../res/tests/compats/sample-v5.ron");
        let _: Tournament = ron::from_str(data).unwrap();
    }
}
