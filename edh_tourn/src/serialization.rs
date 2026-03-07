use core::hash::BuildHasher;
use std::collections::{BTreeMap, HashMap};

use serde::{Serialize, Serializer};

use crate::{
    Tournament,
    config::TournamentConfig,
    error::TournamentError,
    game::{entry::GameEntry, record::GameRecord},
    player::info::PlayerInfo,
    player::stats::PlayerStats,
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

#[derive(serde::Deserialize)]
pub struct SerdeTournament {
    #[serde(alias = "c")]
    config: TournamentConfig,
    #[serde(alias = "p")]
    players: HashMap<u32, PlayerInfo>,
    #[serde(alias = "g")]
    games: Vec<GameEntry>,
}

impl TryFrom<SerdeTournament> for Tournament {
    type Error = TournamentError;
    fn try_from(value: SerdeTournament) -> Result<Self, TournamentError> {
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
