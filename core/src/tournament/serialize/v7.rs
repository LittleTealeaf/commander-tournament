use std::collections::HashMap;

use crate::{
    config::{game::GameConfig, matchmaker::MatchmakerConfig},
    game::entry::GameEntry,
    player::{PlayerId, info::PlayerInfo},
    tournament::serialize::{SerializedTournament, player_info_deserialize},
};

#[derive(Debug, serde::Deserialize, derive_more::Constructor)]
pub struct V7Tournament {
    #[serde(
        deserialize_with = "player_info_deserialize",
        serialize_with = "crate::utils::ordered_map",
        alias = "pls"
    )]
    players: HashMap<PlayerId, PlayerInfo>,
    #[serde(alias = "gms")]
    games: Vec<GameEntry>,
    #[serde(alias = "gc")]
    game_config: GameConfig,
    #[serde(alias = "mc")]
    matchmaker_config: MatchmakerConfig,
}

impl From<V7Tournament> for SerializedTournament {
    fn from(value: V7Tournament) -> Self {
        Self::new(
            FromIterator::from_iter(value.players),
            value.games,
            value.game_config,
            value.matchmaker_config,
        )
    }
}
