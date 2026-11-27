use std::collections::HashMap;

use itertools::Itertools;

use crate::{
    config::{game::GameConfig, matchmaker::MatchmakerConfig},
    game::entry::GameEntry,
    player::{PlayerId, info::PlayerInfo},
    tournament::serialize::SerializedTournament,
};

#[derive(Debug, serde::Deserialize)]
struct SerializedPlayerInfo {
    v: u32,
    #[serde(alias = "players")]
    p: Vec<(PlayerId, PlayerInfo)>,
}

impl FromIterator<(PlayerId, PlayerInfo)> for SerializedPlayerInfo {
    fn from_iter<T: IntoIterator<Item = (PlayerId, PlayerInfo)>>(iter: T) -> Self {
        Self {
            v: 0,
            p: iter.into_iter().sorted_by_key(|(id, _)| *id).collect(),
        }
    }
}

impl SerializedPlayerInfo {
    pub(super) fn value(self) -> HashMap<PlayerId, PlayerInfo> {
        HashMap::from_iter(self.p)
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct V8Tournament {
    #[serde(alias = "pls")]
    players: SerializedPlayerInfo,
    #[serde(alias = "gms")]
    games: Vec<GameEntry>,
    #[serde(alias = "gc")]
    game_config: GameConfig,
    #[serde(alias = "mc")]
    matchmaker_config: MatchmakerConfig,
}

impl From<V8Tournament> for SerializedTournament {
    fn from(value: V8Tournament) -> Self {
        Self::new(
            value.players.p,
            value.games,
            value.game_config,
            value.matchmaker_config,
        )
    }
}
