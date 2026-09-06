use std::collections::HashMap;

use serde::Deserializer;

use crate::{
    config::TournamentConfig,
    error::TournamentError,
    game::entry::GameEntry,
    player::{PlayerId, info::PlayerInfo},
    serialization::utils::DeserializableMap,
    tournament::Tournament,
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

#[derive(serde::Deserialize, Debug, serde::Serialize)]
pub struct V5Tournament {
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

impl From<Tournament> for V5Tournament {
    fn from(value: Tournament) -> Self {
        Self {
            config: value.config,
            players: value.players,
            games: value.games.into_iter().map(GameEntry::from).collect(),
        }
    }
}

impl TryFrom<V5Tournament> for Tournament {
    type Error = TournamentError;
    fn try_from(value: V5Tournament) -> Result<Self, Self::Error> {
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
        let data = include_str!("../../../res/tests/compats/sample-v5.ron");
        let _: Tournament = ron::from_str(data).unwrap();
    }
}
