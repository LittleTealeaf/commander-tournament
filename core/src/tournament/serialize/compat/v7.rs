use std::collections::HashMap;

use serde::Deserializer;

use crate::{
    config::{game::GameConfig, matchmaker::MatchmakerConfig},
    game::entry::GameEntry,
    player::{PlayerId, info::PlayerInfo},
    tournament::serialize::{SerializedTournament, V3Tournament, V4Tournament, V5Tournament, V6Tournament},
    utils::DeserializableMap,
};

use backwards_compat::backwards_compat;

pub(super) fn player_info_deserialize<'de, D>(
    deserializer: D,
) -> Result<HashMap<PlayerId, PlayerInfo>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(DeserializableMap::<PlayerInfo>::deserialize_to_map(deserializer)?
        .into_iter()
        .map(|(id, info)| (PlayerId(id), info))
        .collect())
}

#[derive(Debug, derive_more::Constructor)]
#[backwards_compat(
    serialize=false,
    versions(
        _: Self,
        "3": V3Tournament => "5",
        "4": V4Tournament,
        "5": V5Tournament => Self,
        "6": V6Tournament,
        "7": Self
    )
)]
pub(in crate::tournament::serialize) struct V7Tournament {
    #[serde(deserialize_with = "player_info_deserialize", alias = "pls")]
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
