use core::num::ParseIntError;
use std::collections::HashMap;

use serde::{Deserialize, Deserializer};

use crate::{
    Tournament,
    config::TournamentConfig,
    error::TournamentError,
    game::entry::GameEntry,
    player::{info::PlayerInfo, stats::PlayerStats},
};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum PlayersVariant {
    Integer(HashMap<u32, PlayerInfo>),
    String(HashMap<String, PlayerInfo>),
}

#[allow(clippy::implicit_hasher)]
impl TryFrom<PlayersVariant> for HashMap<u32, PlayerInfo> {
    type Error = ParseIntError;
    fn try_from(value: PlayersVariant) -> Result<Self, Self::Error> {
        match value {
            PlayersVariant::Integer(map) => Ok(map),
            PlayersVariant::String(map) => map
                .into_iter()
                .map(|(key, val)| key.parse().map(|id| (id, val)))
                .collect(),
        }
    }
}

fn deserialize_players<'de, D>(deserializer: D) -> Result<HashMap<u32, PlayerInfo>, D::Error>
where
    D: Deserializer<'de>,
{
    let variant = PlayersVariant::deserialize(deserializer)?;

    // Map the TryFrom error to a Serde de::Error
    variant.try_into().map_err(serde::de::Error::custom)
}

#[derive(serde::Deserialize, Debug)]
pub struct V2SerializedTournament {
    #[serde(alias = "c")]
    config: TournamentConfig,
    #[serde(alias = "p", deserialize_with = "deserialize_players")]
    players: HashMap<u32, PlayerInfo>,
    #[serde(alias = "g")]
    games: Vec<GameEntry>,
}

impl TryFrom<V2SerializedTournament> for Tournament {
    type Error = TournamentError;
    fn try_from(value: V2SerializedTournament) -> Result<Self, TournamentError> {
        let player_names = value
            .players
            .iter()
            .map(|(id, info)| (info.name().to_owned(), *id))
            .collect();

        let mut tournament = Self {
            default_stats: PlayerStats::new(value.config.starting_elo),
            config: value.config,
            stats: HashMap::new(),
            players: value.players,
            player_names,
            games: Vec::new(),
            snapshot: 0,
        };

        for game in value.games {
            tournament.register_entry(game)?;
        }

        tournament.snapshot = 0;

        Ok(tournament)
    }
}

#[cfg(test)]
mod tests {
    use crate::Tournament;

    #[test]
    fn serialize_v2() {
        let data = include_str!("../../../res/tests/compats/sample-v2.ron");
        let _: Tournament = ron::from_str(data).unwrap();
    }
}
