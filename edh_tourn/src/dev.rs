use crate::{Tournament, error::TournamentError, game::GameRecord};

impl Tournament {
    pub fn generate_tournament(player_count: usize, games: usize) -> Result<Self, TournamentError> {
        if games > 0 && player_count < 4 {
            return Err(TournamentError::NotEnoughPlayers);
        }
        let mut tournament = Self::default();
        let mut ids = Vec::new();
        for name in 0..player_count {
            let id = tournament.register_player(format!("{name}"))?;
            ids.push(id);
        }

        let len = ids.len();
        for i in 0..games {
            let players = [
                *ids.get(i % len).ok_or(TournamentError::NotEnoughPlayers)?,
                *ids.get((i + 1) % len)
                    .ok_or(TournamentError::NotEnoughPlayers)?,
                *ids.get((i + 2) % len)
                    .ok_or(TournamentError::NotEnoughPlayers)?,
                *ids.get((i + 3) % len)
                    .ok_or(TournamentError::NotEnoughPlayers)?,
            ];
            let winner = *players
                .get(i % 4)
                .ok_or_else(|| TournamentError::WinnerNotInMatch(u32::try_from(i).unwrap() % 4))?;
            tournament.register_record(GameRecord::new(players, winner)?)?;
        }

        Ok(tournament)
    }

    #[must_use]
    pub fn sample_game() -> Self {
        ron::from_str(include_str!("../res/tests/sample-game.ron")).unwrap()
    }
}

mod tests {
    #[allow(unused)]
    use super::*;

    #[test]
    fn generator_errors_when_few_players() {
        for i in 0..3 {
            Tournament::generate_tournament(i, 0).unwrap();
            Tournament::generate_tournament(i, 1).unwrap_err();
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
        let _ = Tournament::sample_game();
    }
}
