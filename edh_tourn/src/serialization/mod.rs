pub mod utils;
pub mod v1;
pub mod v2;
pub mod v3;
pub mod v4;

use serde::{Deserialize, Serialize};

use crate::{
    error::TournamentError,
    serialization::{v1::V1Tournament, v2::V2Tournament, v3::V3Tournament, v4::V4Tournament},
    tournament::Tournament,
};

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum SerializedTournament {
    Tagged(SerdeTournament),
    V3(V3Tournament),
    V2(V2Tournament),
    V1(V1Tournament),
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(tag = "version")]
pub enum SerdeTournament {
    #[serde(rename = "4")]
    V4(V4Tournament),
    #[serde(rename = "3")]
    V3(V3Tournament),
}

impl From<Tournament> for SerdeTournament {
    fn from(value: Tournament) -> Self {
        Self::V4(value.into())
    }
}

impl TryFrom<SerializedTournament> for Tournament {
    type Error = TournamentError;

    fn try_from(value: SerializedTournament) -> Result<Self, Self::Error> {
        match value {
            SerializedTournament::Tagged(version) => version.try_into(),
            SerializedTournament::V3(v3) => {
                SerializedTournament::Tagged(SerdeTournament::V4(v3.into())).try_into()
            }
            SerializedTournament::V2(v2) => SerializedTournament::V3(v2.into()).try_into(),
            SerializedTournament::V1(v1) => SerializedTournament::V2(v1.into()).try_into(),
        }
    }
}

impl TryFrom<SerdeTournament> for Tournament {
    type Error = TournamentError;

    fn try_from(value: SerdeTournament) -> Result<Self, Self::Error> {
        match value {
            SerdeTournament::V3(v3) => SerdeTournament::V4(v3.into()).try_into(),
            SerdeTournament::V4(v4) => v4.try_into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tournament::Tournament;

    #[test]
    fn ron_serialize_loop() {
        let mut tournament = Tournament::generate_tournament(15, 100).unwrap();
        for _ in 0..10 {
            let ser = ron::ser::to_string(&tournament).unwrap();
            let deser: Tournament = ron::from_str(&ser).unwrap();
            assert_eq!(tournament.games().len(), deser.games().len());
            assert_eq!(tournament.players().len(), deser.players().len());

            tournament = deser;
            tournament.register_debug_player().unwrap();
            tournament
                .record_entry(tournament.random_game().unwrap())
                .unwrap();
        }
    }

    #[test]
    fn json_serialize_loop() {
        let mut tournament = Tournament::generate_tournament(15, 100).unwrap();
        for _ in 0..10 {
            let ser = serde_json::to_string(&tournament).unwrap();
            let deser: Tournament = serde_json::from_str(&ser).unwrap();
            assert_eq!(tournament.games().len(), deser.games().len());
            assert_eq!(tournament.players().len(), deser.players().len());

            tournament = deser;
            tournament.register_debug_player().unwrap();
            tournament
                .record_entry(tournament.random_game().unwrap())
                .unwrap();
        }
    }

    #[test]
    fn toml_serialize_loop() {
        let mut tournament = Tournament::generate_tournament(15, 100).unwrap();
        for _ in 0..10 {
            let ser = toml::to_string(&tournament).unwrap();
            let deser: Tournament = toml::from_str(&ser).unwrap();
            assert_eq!(tournament.games().len(), deser.games().len());
            assert_eq!(tournament.players().len(), deser.players().len());

            tournament = deser;
            tournament.register_debug_player().unwrap();
            tournament
                .record_entry(tournament.random_game().unwrap())
                .unwrap();
        }
    }

    #[test]
    fn deserialize_populates_player_table() {
        let mut tourn = Tournament::sample_game();
        let id = tourn.register_player(String::from("Test String")).unwrap();

        let serialized = ron::to_string(&tourn).unwrap();
        let de_tourn: Tournament = ron::from_str(&serialized).unwrap();

        assert_eq!(id, de_tourn.get_player_id(&String::from("Test String")).unwrap());
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
