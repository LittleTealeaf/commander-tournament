use core::hash::{Hash, Hasher};
use std::hash::DefaultHasher;

use itertools::{Itertools, chain};

use crate::{
    config::game::GameConfig, error::TournamentError, game::entry::GameEntry, player::PlayerId,
    tournament::Tournament,
};
use rand::{SeedableRng, seq::IndexedRandom};
use rand_chacha::ChaCha8Rng;

fn hash_to_u64<T>(value: T) -> u64
where
    T: Hash,
{
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);

    hasher.finish()
}

impl Tournament {
    #[must_use]
    pub const fn snapshot(&self) -> usize {
        self.snapshot
    }

    pub fn generate_tournament(player_count: usize, games: usize) -> Result<Self, TournamentError> {
        if games > 0 && player_count < 4 {
            return Err(TournamentError::NotEnoughPlayers);
        }

        let mut tournament = Self::default();

        let ids: Vec<PlayerId> = (0..player_count)
            .map(|_| tournament.register_debug_player())
            .collect::<Result<Vec<_>, _>>()?;

        let mut rng = ChaCha8Rng::seed_from_u64(hash_to_u64((player_count, games)));

        for _ in 0..games {
            let players = ids.sample(&mut rng, 4).copied().next_array().unwrap();
            let winner = *players.choose(&mut rng).unwrap();
            let entry = GameEntry::new(players, winner)?;
            tournament.register_entry(entry)?;
        }

        Ok(tournament)
    }

    pub fn random_game(&self) -> Option<GameEntry> {
        let hash = self.games().iter().map(GameEntry::from).collect_vec();
        let seed = hash_to_u64(hash);
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let ids = self.players().keys().copied().collect_vec();
        let players = ids.sample(&mut rng, 4).copied().next_array()?;
        let winner = *players.choose(&mut rng)?;
        GameEntry::new(players, winner).ok()
    }

    #[must_use]
    pub fn sample_game() -> Self {
        ron::from_str(include_str!("../../../res/tests/compats/sample-v2.ron")).unwrap()
    }

    pub fn sample_tsv_game() -> Result<Self, TournamentError> {
        Self::from_tsv_games(include_str!("../../../tests/sample-tsv.tsv"))
    }

    pub fn test_tournaments() -> impl Iterator<Item = Self> {
        chain!(
            [Self::sample_game(), Self::new()],
            Self::sample_tsv_game(),
            [0, 4, 8, 16, 32, 64]
                .into_iter()
                .flat_map(|a| {
                    [0, 4, 8, 16, 32, 64]
                        .into_iter()
                        .filter_map(move |b| Self::generate_tournament(a, b).ok())
                })
                .enumerate()
                .flat_map(|(i, tourn)| tourn.with_game_config(GameConfig::random(i)))
        )
    }

    pub fn register_debug_player(&mut self) -> Result<PlayerId, TournamentError> {
        let max = self.players.keys().max();
        if let Some(PlayerId(val)) = max {
            self.register_player(format!("debug-{}", val + 1))
        } else {
            self.register_player("debug-0".to_owned())
        }
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
