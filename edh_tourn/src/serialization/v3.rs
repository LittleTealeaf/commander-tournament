use std::collections::HashMap;

use serde::Deserialize;

use crate::Tournament;
use crate::error::TournamentError;
use crate::game::entry::GameEntry;
use crate::serialization::utils::DeserializableMap;
use crate::{config::TournamentConfig, player::info::PlayerInfo};

#[derive(Deserialize, Debug)]
pub struct V3Tournament {
    pub(super) config: TournamentConfig,
    #[serde(deserialize_with = "DeserializableMap::deserialize_to_map")]
    pub(super) players: HashMap<u32, PlayerInfo>,
    pub(super) games: Vec<GameEntry>,
}

impl TryFrom<V3Tournament> for Tournament {
    type Error = TournamentError;
    fn try_from(value: V3Tournament) -> Result<Self, Self::Error> {
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
    fn serialize_v3() {
        let data = include_str!("../../../res/tests/compats/sample-v3.ron");
        let _: Tournament = ron::from_str(data).unwrap();
    }
}
