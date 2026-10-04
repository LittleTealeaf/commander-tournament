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
        *config.initial_elo_mut() += 1500.0;
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
