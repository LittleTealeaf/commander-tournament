pub mod v1;
pub mod v2;

use core::hash::BuildHasher;
use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize, Serializer};

use crate::{
    Tournament,
    error::TournamentError,
    game::{entry::GameEntry, record::GameRecord},
    serialization::{v1::V1SerializedTournament, v2::V2SerializedTournament},
};

/// For use with serde's ``serialize_with`` attribute
pub fn ordered_map<S, K, V, HS>(value: &HashMap<K, V, HS>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    HS: BuildHasher,
    V: Serialize,
    K: Ord + Serialize,
{
    let ordered: BTreeMap<_, _> = value.iter().collect();
    ordered.serialize(serializer)
}

pub fn convert_games<S>(items: &[GameRecord], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let values = items
        .iter()
        .flat_map(|record| GameEntry::new(record.ids(), record.winner()))
        .collect::<Vec<_>>();
    values.serialize(serializer)
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum SerializedTournament {
    V2(V2SerializedTournament),
    V1(V1SerializedTournament),
}

impl TryFrom<SerializedTournament> for Tournament {
    type Error = TournamentError;

    fn try_from(value: SerializedTournament) -> Result<Self, Self::Error> {
        match value {
            SerializedTournament::V2(v2) => v2.try_into(), // Uses V2 logic
            SerializedTournament::V1(v1) => v1.try_into(), // Uses V1 migration logic
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Tournament;

    #[test]
    fn ron_serialize_loop() {
        for mut game in Tournament::test_tournaments() {
            for _ in 0..3 {
                let ser = ron::ser::to_string(&game).unwrap();
                game = ron::from_str(&ser).unwrap();
            }
        }
    }

    #[test]
    fn json_serialize_loop() {
        for mut game in Tournament::test_tournaments() {
            for _ in 0..3 {
                let ser = serde_json::to_string(&game).unwrap();
                game = serde_json::from_str(&ser).unwrap();
            }
        }
    }

    #[test]
    fn toml_serialize_loop() {
        for mut game in Tournament::test_tournaments() {
            for _ in 0..3 {
                let ser = toml::to_string(&game).unwrap();
                game = toml::from_str(&ser).unwrap();
            }
        }
    }

    #[test]
    fn deserialize_populates_player_table() {
        let mut tourn = Tournament::sample_game();
        let id = tourn.register_player(String::from("Test String")).unwrap();

        let serialized = ron::to_string(&tourn).unwrap();
        let de_tourn: Tournament = ron::from_str(&serialized).unwrap();

        assert_eq!(
            id,
            de_tourn
                .get_player_id(&String::from("Test String"))
                .unwrap()
        );
    }

    #[test]
    fn deserialize_configures_default_stats() {
        let mut tourn = Tournament::sample_game();
        let mut config = tourn.config.clone();
        config.starting_elo += 1500.0;
        tourn.set_config(config).unwrap();
        let starting_elo = tourn.default_stats().elo();

        let serialized = ron::to_string(&tourn).unwrap();
        let de_tourn: Tournament = ron::from_str(&serialized).unwrap();
        assert!((starting_elo - de_tourn.default_stats().elo()) <= 1e-9);
    }

    #[test]
    fn deserialize_resets_snapshot() {
        let mut t_source = Tournament::sample_game();
        t_source.snapshot = 2;

        let ser = ron::to_string(&t_source).unwrap();
        let t_deserialized: Tournament = ron::from_str(&ser).unwrap();
        assert_eq!(0, t_deserialized.snapshot);
    }
}
