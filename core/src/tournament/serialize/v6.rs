use std::collections::HashMap;

use crate::{
    game::entry::GameEntry,
    player::{PlayerId, info::PlayerInfo},
    tournament::serialize::{config::TournamentConfig, player_info_deserialize, v7::V7Tournament},
};

#[derive(Debug, serde::Deserialize, derive_more::Constructor)]
pub struct V6Tournament {
    #[serde(rename = "cfg", alias = "config")]
    config: TournamentConfig,
    #[serde(
        deserialize_with = "player_info_deserialize",
        rename = "pls",
        alias = "players"
    )]
    players: HashMap<PlayerId, PlayerInfo>,
    #[serde(rename = "gms", alias = "games")]
    games: Vec<GameEntry>,
}

impl From<V6Tournament> for V7Tournament {
    fn from(value: V6Tournament) -> Self {
        Self::new(
            value.players,
            value.games,
            value.config.game,
            value.config.matchmaker,
        )
    }
}
