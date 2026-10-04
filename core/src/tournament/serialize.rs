use serde::Deserializer;
use std::collections::HashMap;

use crate::{
    config::TournamentConfig,
    error::TournamentError,
    game::entry::GameEntry,
    player::{PlayerId, info::PlayerInfo},
    tournament::Tournament,
    utils::DeserializableMap,
};

fn player_info_deserialize<'de, D>(deserializer: D) -> Result<HashMap<PlayerId, PlayerInfo>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(DeserializableMap::<PlayerInfo>::deserialize_to_map(deserializer)?
        .into_iter()
        .map(|(id, info)| (PlayerId(id), info))
        .collect())
}

#[derive(Debug, serde::Deserialize, serde::Serialize, derive_more::Constructor)]
pub struct SerializedTournament {
    #[serde(rename = "cfg", alias = "config")]
    config: TournamentConfig,
    #[serde(
        deserialize_with = "player_info_deserialize",
        serialize_with = "crate::utils::ordered_map",
        rename = "pls",
        alias = "players"
    )]
    players: HashMap<PlayerId, PlayerInfo>,
    #[serde(rename = "gms", alias = "games")]
    games: Vec<GameEntry>,
}

impl From<Tournament> for SerializedTournament {
    fn from(value: Tournament) -> Self {
        Self {
            config: value.config,
            players: value.players,
            games: value.games.into_iter().map(GameEntry::from).collect(),
        }
    }
}

impl TryFrom<SerializedTournament> for Tournament {
    type Error = TournamentError;
    fn try_from(value: SerializedTournament) -> Result<Self, Self::Error> {
        let mut tournament = Self {
            config: value.config,
            players: value.players,
            ..Self::default()
        };
        tournament.reload()?;
        for game in value.games {
            tournament.record_entry(game)?;
        }

        tournament.snapshot = 0;

        Ok(tournament)
    }
}

#[cfg(test)]
mod tests {
    use crate::tournament::Tournament;

    #[test]
    fn deserialize() {
        let data = include_str!("../../../res/tests/compats/sample-v6.ron");
        let _: Tournament = ron::from_str(data).unwrap();
    }
}
