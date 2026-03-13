pub mod utils;
pub mod v1;
pub mod v2;
pub mod v3;

use serde::Deserialize;

use crate::{
    Tournament,
    error::TournamentError,
    serialization::{v1::V1Tournament, v2::V2Tournament, v3::V3Tournament},
};
#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum SerializedTournament {
    V3(V3Tournament),
    V2(V2Tournament),
    V1(V1Tournament),
}

impl TryFrom<SerializedTournament> for Tournament {
    type Error = TournamentError;

    fn try_from(value: SerializedTournament) -> Result<Self, Self::Error> {
        match value {
            SerializedTournament::V3(v3) => v3.try_into(),
            SerializedTournament::V2(v2) => SerializedTournament::V3(v2.into()).try_into(),
            SerializedTournament::V1(v1) => SerializedTournament::V2(v1.into()).try_into(),
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
        let mut config = tourn.game_config().clone();
        config.starting_elo += 1500.0;
        tourn.set_game_config(config).unwrap();
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
