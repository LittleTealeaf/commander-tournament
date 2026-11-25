use std::collections::HashMap;

use backwards_compat::backwards_compat;
use itertools::Itertools;

use crate::{
    config::{game::GameConfig, matchmaker::MatchmakerConfig},
    game::entry::GameEntry,
    player::{PlayerId, info::PlayerInfo},
    tournament::serialize::SerializedTournament,
};

#[derive(Debug)]
#[backwards_compat(
    tag="v",
    content = "p",
    versions(
        1: Self
    )
)]
struct SerializedPlayerInfo(Vec<(PlayerId, PlayerInfo)>);

impl FromIterator<(PlayerId, PlayerInfo)> for SerializedPlayerInfo {
    fn from_iter<T: IntoIterator<Item = (PlayerId, PlayerInfo)>>(iter: T) -> Self {
        Self(iter.into_iter().sorted_by_key(|(id, _)| *id).collect())
    }
}

impl SerializedPlayerInfo {
    pub(super) fn value(self) -> HashMap<PlayerId, PlayerInfo> {
        HashMap::from_iter(self.0)
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct V8Tournament {
    #[serde(rename = "pls", alias = "players")]
    players: SerializedPlayerInfo,
    #[serde(rename = "gms", alias = "games")]
    games: Vec<GameEntry>,
    #[serde(rename = "gc")]
    game_config: GameConfig,
    #[serde(rename = "mc")]
    matchmaker_config: MatchmakerConfig,
}

impl From<V8Tournament> for SerializedTournament {
    fn from(value: V8Tournament) -> Self {
        Self::new(
            value.players.0,
            value.games,
            value.game_config,
            value.matchmaker_config,
        )
    }
}
