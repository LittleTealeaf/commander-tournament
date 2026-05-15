#! Dev/Test helper functions
use core::hash::{Hash, Hasher};
use std::hash::DefaultHasher;

use itertools::Itertools;

use crate::{
    error::TournamentError,
    game::{entry::GameEntry, matchup::Matchup, record::GameRecord},
    player::PlayerId,
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
            tournament.record_entry(entry)?;
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
        ron::from_str(include_str!("../../res/tests/compats/sample-v2.ron")).unwrap()
    }

    pub fn register_debug_player(&mut self) -> Result<PlayerId, TournamentError> {
        let max = self.players.keys().max().map_or(0, |id| id.0 + 1);
        self.register_player(format!("debug-{max}"))
    }

    pub fn register_debug_players<const N: usize>(&mut self) -> Result<[PlayerId; N], TournamentError> {
        let first = self.register_debug_player()?;
        let mut values = [first; N];
        for value in values.iter_mut().skip(1) {
            *value = self.register_debug_player()?;
        }
        Ok(values)
    }
}

impl Matchup {
    #[must_use]
    pub fn debug_record(self) -> Option<GameRecord> {
        let players = self.players().clone().map(|player| format!("{player:?}"));
        let seed = hash_to_u64(players);
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let winner = self.players().choose(&mut rng)?.id();
        self.record(winner).ok()
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

    #[test]
    fn debug_players_are_all_different() {
        use std::collections::HashSet;

        let mut t = Tournament::new();
        let tests = [
            Vec::from(t.register_debug_players::<1>().unwrap()),
            Vec::from(t.register_debug_players::<2>().unwrap()),
            Vec::from(t.register_debug_players::<3>().unwrap()),
            Vec::from(t.register_debug_players::<4>().unwrap()),
            Vec::from(t.register_debug_players::<5>().unwrap()),
        ];

        for test in tests {
            let mut ids = HashSet::new();
            for id in test {
                assert!(!ids.contains(&id));
                ids.insert(id);
            }
        }
    }
}
