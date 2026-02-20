use crate::{Tournament, error::TournamentError, game::GameRecord};

impl Tournament {
    pub fn generate_tournament(
        player_count: usize,
        games: usize,
    ) -> Result<Tournament, TournamentError> {
        if games > 0 && player_count < 4 {
            return Err(TournamentError::NotEnoughPlayers);
        }
        let mut tournament = Tournament::default();
        let mut ids = Vec::new();
        for name in 0..player_count {
            let id = tournament.register_player(format!("{name}"))?;
            ids.push(id);
        }

        let len = ids.len();
        for i in 0..games {
            let players = [
                ids[i % len],
                ids[(i + 1) % len],
                ids[(i + 2) % len],
                ids[(i + 3) % len],
            ];
            let winner = players[i % 4];
            tournament.register_record(GameRecord::new(players, winner)?)?;
        }

        Ok(tournament)
    }

    pub fn sample_game() -> Tournament {
        ron::from_str(include_str!("../../tests/sample-game.ron")).unwrap()
    }
}

mod tests {
    use crate::Tournament;

    #[test]
    fn generator_errors_when_few_players() {
        for i in 0..3 {
            assert!(Tournament::generate_tournament(i, 0).is_ok());
            assert!(Tournament::generate_tournament(i, 1).is_err());
        }
    }

    #[test]
    fn generator_populates_correct_player_count() {
        for i in [0, 1, 15, 100] {
            let tournament = Tournament::generate_tournament(i, 0).unwrap();
            assert_eq!(i, tournament.players().len());
        }
    }

    #[test]
    fn generator_populates_correct_game_count() {
        for i in [0, 1, 5, 15, 100] {
            let tournament = Tournament::generate_tournament(10, i).unwrap();
            assert_eq!(i, tournament.games().len());
        }
    }

    #[test]
    fn sample_game_loads() {
        Tournament::sample_game();
    }
}
