use std::collections::HashMap;

use crate::{
    Tournament, config::TournamentConfig, error::TournamentError, game::GameRecord,
    info::PlayerInfo,
};

#[derive(serde::Deserialize)]
pub struct SerdeTournament {
    config: TournamentConfig,
    players: HashMap<u32, PlayerInfo>,
    games: Vec<GameRecord>,
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
            config: value.config,
            stats: HashMap::new(),
            players: value.players,
            player_names,
            games: Vec::new(),
        };

        tournament.config.version = 0;

        for game in value.games {
            tournament.register_record(game)?;
        }

        Ok(tournament)
    }
}

#[cfg(test)]
mod tests {
    use crate::Tournament;

    #[test]
    fn ron_serialize_loop() {
        for mut game in [
            Tournament::sample_game(),
            Tournament::generate_tournament(30, 50).unwrap(),
        ] {
            for _ in 0..3 {
                let ser = ron::ser::to_string(&game).unwrap();
                game = ron::from_str(&ser).unwrap();
            }
        }
    }

    #[test]
    fn json_serialize_loop() {
        for mut game in [
            Tournament::sample_game(),
            Tournament::generate_tournament(30, 50).unwrap(),
        ] {
            for _ in 0..3 {
                let ser = ron::ser::to_string(&game).unwrap();
                game = ron::from_str(&ser).unwrap();
            }
        }
    }

    #[test]
    fn toml_serialize_loop() {
        for mut game in [
            Tournament::sample_game(),
            Tournament::generate_tournament(30, 50).unwrap(),
        ] {
            for _ in 0..3 {
                let ser = toml::to_string(&game).unwrap();
                game = toml::from_str(&ser).unwrap();
            }
        }
    }

    #[test]
    fn deserialize_resets_config_version() {
        let mut t_source = Tournament::sample_game();
        t_source.config.version = 2;

        let ser = ron::to_string(&t_source).unwrap();
        let t_deserialized: Tournament = ron::from_str(&ser).unwrap();
        assert_eq!(0, t_deserialized.config.version);
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
}
