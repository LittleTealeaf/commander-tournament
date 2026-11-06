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
        if games > 0 && player_count < 2 {
            return Err(TournamentError::NotEnoughPlayers);
        }

        let mut tournament = Self::default();

        let ids: Vec<PlayerId> = (0..player_count)
            .map(|_| tournament.register_debug_player())
            .collect::<Result<Vec<_>, _>>()?;

        let mut rng = ChaCha8Rng::seed_from_u64(hash_to_u64((player_count, games)));

        for _ in 0..games {
            let match_size = player_count.min(4);
            let players = ids.sample(&mut rng, match_size).copied().collect::<Vec<_>>();
            let winner = *players.choose(&mut rng).unwrap();
            let entry = GameEntry::new(players, winner)?;
            tournament.record_entry(entry)?;
        }

        Ok(tournament)
    }

    pub fn debug_game<P: Into<Vec<PlayerId>>>(
        &mut self,
        players: P,
        winner: PlayerId,
    ) -> Result<(), TournamentError> {
        let matchup = self.create_match(players.into())?;
        let record = matchup.record(winner)?;
        self.record_game(record)?;
        Ok(())
    }

    pub fn random_game(&self) -> Option<GameEntry> {
        let hash = self.games().iter().map(GameEntry::from).collect_vec();
        let seed = hash_to_u64(hash);
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let ids = self.players().keys().copied().collect_vec();
        if ids.len() < 2 {
            return None;
        }
        let match_size = ids.len().min(4);
        let players = ids.sample(&mut rng, match_size).copied().collect::<Vec<_>>();
        let winner = *players.choose(&mut rng)?;
        GameEntry::new(players, winner).ok()
    }

    #[must_use]
    pub fn sample_game() -> Self {
        ron::from_str(include_str!("../../res/tests/compats/sample-v5.ron")).unwrap()
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
        let players = self
            .players()
            .iter()
            .map(|player| format!("{player:?}"))
            .collect::<Vec<_>>();
        let seed = hash_to_u64(players);
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let winner = self.players().choose(&mut rng)?.id();
        self.record(winner).ok()
    }
}

#[test]
fn snapshot_returns() {
    let t = Tournament::generate_tournament(50, 100).unwrap();
    assert_eq!(t.snapshot(), t.snapshot);
}

#[test]
fn generator_errors_when_few_players() {
    for i in 0..2 {
        Tournament::generate_tournament(i, 0).unwrap();
        Tournament::generate_tournament(i, 1).unwrap_err();
    }
    Tournament::generate_tournament(2, 1).unwrap();
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

#[test]
fn generate_tournament_two_and_three_players() {
    let t2 = Tournament::generate_tournament(2, 5).unwrap();
    assert_eq!(t2.players().len(), 2);
    assert_eq!(t2.games().len(), 5);
    for game in t2.games() {
        assert_eq!(game.players().len(), 2);
    }

    let t3 = Tournament::generate_tournament(3, 5).unwrap();
    assert_eq!(t3.players().len(), 3);
    assert_eq!(t3.games().len(), 5);
    for game in t3.games() {
        assert_eq!(game.players().len(), 3);
    }
}

#[test]
fn debug_game_variable_players() {
    let mut t = Tournament::new();
    let p1 = t.register_debug_player().unwrap();
    let p2 = t.register_debug_player().unwrap();
    let p3 = t.register_debug_player().unwrap();
    let p4 = t.register_debug_player().unwrap();

    // 2 players
    t.debug_game([p1, p2], p1).unwrap();
    assert_eq!(t.games().len(), 1);

    // 3 players
    t.debug_game([p1, p2, p3], p2).unwrap();
    assert_eq!(t.games().len(), 2);

    // 4 players
    t.debug_game([p1, p2, p3, p4], p3).unwrap();
    assert_eq!(t.games().len(), 3);

    let mut games = t.games().iter();
    assert_eq!(games.next().unwrap().players().len(), 2);
    assert_eq!(games.next().unwrap().players().len(), 3);
    assert_eq!(games.next().unwrap().players().len(), 4);
}

#[test]
fn random_game_variable_players() {
    let mut t = Tournament::new();
    // 0 players
    assert!(t.random_game().is_none());

    // 1 player
    let p1 = t.register_debug_player().unwrap();
    assert!(t.random_game().is_none());

    // 2 players
    let p2 = t.register_debug_player().unwrap();
    let entry2 = t.random_game().unwrap();
    assert_eq!(entry2.players.len(), 2);
    assert!(entry2.players.contains(&p1));
    assert!(entry2.players.contains(&p2));

    // 3 players
    let p3 = t.register_debug_player().unwrap();
    let entry3 = t.random_game().unwrap();
    assert_eq!(entry3.players.len(), 3);
    assert!(entry3.players.contains(&p3));
}
