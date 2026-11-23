use std::collections::HashMap;

use crate::{
    config::{game::GameConfig, matchmaker::MatchmakerConfig},
    game::entry::GameEntry,
    player::{PlayerId, info::PlayerInfo},
    tournament::serialize::compat::v7::{V7Tournament, player_info_deserialize},
};

#[derive(Debug, Clone, PartialEq, Default, derive_more::Constructor, serde::Deserialize)]
struct TournamentConfig {
    game: GameConfig,
    matchmaker: MatchmakerConfig,
}

#[derive(Debug, serde::Deserialize, derive_more::Constructor)]
pub(in crate::tournament::serialize) struct V6Tournament {
    #[serde(alias = "cfg")]
    config: TournamentConfig,
    #[serde(deserialize_with = "player_info_deserialize", alias = "pls")]
    players: HashMap<PlayerId, PlayerInfo>,
    #[serde(alias = "gms")]
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
