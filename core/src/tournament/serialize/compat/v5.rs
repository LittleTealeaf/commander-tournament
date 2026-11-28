use std::collections::HashMap;

use crate::{
    config::{game::GameConfig, matchmaker::MatchmakerConfig},
    game::entry::GameEntry,
    player::{PlayerId, info::PlayerInfo},
    tournament::serialize::compat::v7::{V7Tournament, player_info_deserialize},
};

#[derive(Debug, serde::Deserialize, derive_more::Constructor)]
pub(in crate::tournament::serialize) struct V5Tournament {
    #[serde(alias = "cfg")]
    config: V5TournamentConfig,
    #[serde(deserialize_with = "player_info_deserialize", alias = "pls")]
    players: HashMap<PlayerId, PlayerInfo>,
    #[serde(alias = "gms")]
    games: Vec<GameEntry>,
}

impl From<V5Tournament> for V7Tournament {
    fn from(value: V5Tournament) -> Self {
        Self::new(
            value.players,
            value.games,
            GameConfig::default(),
            MatchmakerConfig {
                elo_range: value.config.matchmaker.elo_range,
                min_pool_size: value.config.matchmaker.min_pool_size,
                ..Default::default()
            },
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

#[derive(Debug, serde::Deserialize)]
pub(super) struct V5GameConfig {
    starting_elo: f64,
    game_points: f64,
    game_elo_pow_scale: f64,
    game_wr_pow_scale: f64,
    game_elo_weight: f64,
    game_wr_weight: f64,
}

impl Default for V5GameConfig {
    fn default() -> Self {
        Self {
            starting_elo: 1500.0,
            game_points: 25.0,
            game_elo_pow_scale: 6.0,
            game_wr_pow_scale: 1.0,
            game_elo_weight: 65.0,
            game_wr_weight: 35.0,
        }
    }
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
