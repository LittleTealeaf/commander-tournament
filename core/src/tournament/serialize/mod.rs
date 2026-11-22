pub mod config;
pub mod players;
pub mod v3;
pub mod v4;
pub mod v5;
pub mod v6;
pub mod v7;

use std::collections::HashMap;

use serde::Deserializer;
pub use v3::V3Tournament;
pub use v4::V4Tournament;
pub use v5::V5Tournament;

use crate::{
    config::{game::GameConfig, matchmaker::MatchmakerConfig},
    error::TournamentError,
    game::entry::GameEntry,
    player::{PlayerId, info::PlayerInfo},
    tournament::{Tournament, serialize::players::SerializedPlayerInfo},
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
    #[serde(rename = "pls", alias = "players")]
    players: SerializedPlayerInfo,
    #[serde(rename = "gms", alias = "games")]
    games: Vec<GameEntry>,
    #[serde(rename = "gc")]
    game_config: GameConfig,
    #[serde(rename = "mc")]
    matchmaker_config: MatchmakerConfig,
}

impl From<Tournament> for SerializedTournament {
    fn from(value: Tournament) -> Self {
        Self {
            game_config: value.game_config,
            matchmaker_config: value.matchmaker_config,
            players: FromIterator::from_iter(value.players),
            games: value.games.into_iter().map(GameEntry::from).collect(),
        }
    }
}

impl TryFrom<SerializedTournament> for Tournament {
    type Error = TournamentError;
    fn try_from(value: SerializedTournament) -> Result<Self, Self::Error> {
        let mut tournament = Self {
            game_config: value.game_config,
            matchmaker_config: value.matchmaker_config,
            players: value.players.value(),
            ..Self::default()
        };
        tournament.reload()?;
        for game in value.games {
            tournament.record_entry(game)?;
        }

        Ok(tournament)
    }
}

#[cfg(test)]
mod tests {
    use crate::tournament::Tournament;

    use rstest::rstest;

    #[rstest]
    fn deserialize(
        #[base_dir = "tests/fixtures/saves"]
        #[files("*.ron")]
        #[mode=str]
        data: &str,
    ) {
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
}
