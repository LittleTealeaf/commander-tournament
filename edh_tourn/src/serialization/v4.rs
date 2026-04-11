use std::collections::HashMap;

use serde::{Deserialize, Deserializer, Serialize};

use crate::error::TournamentError;
use crate::game::entry::GameEntry;
use crate::player::PlayerId;
use crate::serialization::utils::DeserializableMap;
use crate::tournament::Tournament;
use crate::{config::TournamentConfig, player::info::PlayerInfo};

fn player_info_deserialize<'de, D>(
    deserializer: D,
) -> Result<HashMap<PlayerId, PlayerInfo>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(
        DeserializableMap::<PlayerInfo>::deserialize_to_map(deserializer)?
            .into_iter()
            .map(|(id, info)| (PlayerId(id), info))
            .collect(),
    )
}

#[derive(Deserialize, Debug, Serialize)]
pub struct V4Tournament {
    #[serde(rename = "cfg", alias = "config")]
    pub(super) config: TournamentConfig,
    #[serde(
        deserialize_with = "player_info_deserialize",
        serialize_with = "super::utils::ordered_map",
        rename = "pls",
        alias = "players"
    )]
    pub(super) players: HashMap<PlayerId, PlayerInfo>,
    #[serde(rename = "gms", alias = "games")]
    pub(super) games: Vec<GameEntry>,
}

impl From<Tournament> for V4Tournament {
    fn from(value: Tournament) -> Self {
        Self {
            config: value.config,
            players: value.players,
            games: value.games.into_iter().map(GameEntry::from).collect(),
        }
    }
}

impl TryFrom<V4Tournament> for Tournament {
    type Error = TournamentError;
    fn try_from(value: V4Tournament) -> Result<Self, Self::Error> {
        let mut tournament = Self {
            config: value.config,
            players: value.players,
            ..Self::default()
        };
        tournament.reload()?;
        for game in value.games {
            tournament.register_entry(game)?;
        }

        tournament.snapshot = 0;

        Ok(tournament)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn deserialize() {
        let data = include_str!("../../../res/tests/compats/sample-v4.ron");
        let _: Tournament = ron::from_str(data).unwrap();
    }
}
