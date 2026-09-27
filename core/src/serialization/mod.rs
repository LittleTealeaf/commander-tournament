pub mod utils;
pub mod v3;
pub mod v4;
pub mod v5;

use backwards_compat::backwards_compat;

use crate::{
    serialization::{v3::V3Tournament, v4::V4Tournament, v5::V5Tournament},
    tournament::Tournament,
};

backwards_compat! {
    #[tag = "version", version = 5]
    compat Tournament {
        3: V3Tournament,
        4: V4Tournament,
        #[fallible] 5: V5Tournament,
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
        *config.starting_elo_mut() += 1500.0;
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
