#! Dev/Test helper functions
use core::hash::{Hash, Hasher};
use core::iter;
use std::{collections::BinaryHeap, hash::DefaultHasher};

use itertools::Itertools;
use ordered_float::NotNan;

use crate::{
    error::TournamentError,
    game::{POD_SIZE, entry::GameEntry, matchup::Matchup, record::GameRecord},
    player::{PlayerId, info::PlayerInfo},
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

    pub fn debug_game(&mut self, players: [PlayerId; 4], winner: PlayerId) -> Result<(), TournamentError> {
        let matchup = self.create_match(players)?;
        let record = matchup.record(winner)?;
        self.record_game(record)?;
        Ok(())
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
        ron::from_str(include_str!("../../res/tests/compats/sample-v5.ron")).unwrap()
    }

    pub fn register_debug_player(&mut self) -> Result<PlayerId, TournamentError> {
        let max = self.players().keys().max().map_or(0, |id| id.0 + 1);
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

    pub fn register_debug_player_info<F>(&mut self, with_info: F) -> Result<PlayerId, TournamentError>
    where
        F: Fn(PlayerInfo) -> PlayerInfo,
    {
        let id = self.register_debug_player()?;
        self.update_player_info(&id, with_info)?;
        Ok(id)
    }

    pub fn register_debug_players_info<const N: usize, F>(
        &mut self,
        with_info: F,
    ) -> Result<[PlayerId; N], TournamentError>
    where
        F: Fn(PlayerInfo) -> PlayerInfo,
    {
        let ids = self.register_debug_players()?;

        for id in &ids {
            self.update_player_info(id, &with_info)?;
        }

        Ok(ids)
    }
}

/// Setting Elo (Fun Debug Stuff)
impl Tournament {
    /// Uses and creates archived players to play against the target in order to get the target
    /// as close as possible to the target elo
    pub fn debug_set_target_elo(
        &mut self,
        target: PlayerId,
        target_elo: f64,
    ) -> Result<f64, TournamentError> {
        self.require_id_registered(target)?;

        let scale = self.game_config().logistic_scale();
        let drift_window = scale * 0.5;
        let max_steps = 500;
        let mut steps = 0;

        loop {
            let curr_elo = self.get_player_or_default_stats(target).elo();
            let curr_diff = target_elo - curr_elo;
            if curr_diff.abs() < self.game_config().base_k() || steps >= max_steps {
                break;
            }

            let pool = select_sparring_pool(self, curr_elo, target_elo)?;

            // Mark sparring pool as unarchived. They will not be used in subsequent versions.
            for id in pool {
                self.update_player_info(&id, |info| info.with_is_archived(false))?;
            }

            // if let Some(id) = pool.last() {
            for id in &pool {
                let stats = self.get_player_or_default_stats(*id);
                if stats.elo() < curr_elo - drift_window {
                    let desired_elo = drift_window.mul_add(-0.5, curr_elo);
                    self.debug_set_target_elo(*id, desired_elo)?;
                } else if stats.elo() > curr_elo + drift_window {
                    let desired_elo = drift_window.mul_add(0.5, curr_elo);
                    self.debug_set_target_elo(*id, desired_elo)?;
                }
            }

            let matchup = self.create_match([target, pool[0], pool[1], pool[2]])?;
            let target_match = &matchup.players()[0];
            let elo_win = *target_match.elo_win();
            let elo_loss = *target_match.elo_loss();

            if curr_diff > 0.0 {
                if (target_elo - (curr_elo + elo_win)).abs() >= curr_diff.abs() {
                    break;
                }
                let record = matchup.record(target)?;
                self.record_game(record)?;
            } else {
                if (target_elo - (curr_elo - elo_loss)).abs() >= curr_diff.abs() {
                    break;
                }
                #[allow(clippy::indexing_slicing, reason = "Known size")]
                let winner = pool[steps % (POD_SIZE - 1)];
                let record = matchup.record(winner)?;
                self.record_game(record)?;
            }

            for id in pool {
                self.update_player_info(&id, |info| info.with_is_archived(true))?;
            }

            steps += 1;
        }
        let elo = self.get_player_or_default_stats(target).elo();
        Ok(elo)
    }
}

fn select_sparring_pool(
    tourn: &mut Tournament,
    current_elo: f64,
    target_elo: f64,
) -> Result<[PlayerId; POD_SIZE - 1], TournamentError> {
    let mut candidates = tourn
        .players()
        .iter()
        .filter(|(_, info)| info.is_archived())
        .filter_map(|(id, _)| {
            let stats = tourn.get_player_or_default_stats(*id);
            Some(if target_elo > current_elo {
                (NotNan::new(stats.elo()).ok()?, *id)
            } else {
                (NotNan::new(-stats.elo()).ok()?, *id)
            })
        })
        .collect::<BinaryHeap<_>>();

    let baseline = {
        if target_elo > current_elo {
            tourn.game_config().initial_elo()
        } else {
            -tourn.game_config().initial_elo()
        }
    };

    if candidates.peek().is_none_or(|(val, _)| {
        let scale = tourn.game_config().logistic_scale();
        let drift_window = scale * 0.25;
        val.into_inner() + drift_window < baseline
    }) {
        return tourn.register_debug_players_info(|info| info.with_is_archived(true));
    }

    iter::from_fn(|| Some(candidates.pop()?.1))
        .chain(iter::from_fn(|| {
            tourn
                .register_debug_player_info(|info| info.with_is_archived(true))
                .ok()
        }))
        .take(POD_SIZE - 1)
        .collect_array()
        .ok_or(TournamentError::NotEnoughPlayers)
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

#[test]
fn set_elo_below() {
    let mut tourn = Tournament::new();
    let target = tourn.register_debug_player().unwrap();
    let target_elo = tourn
        .game_config()
        .logistic_scale()
        .mul_add(-0.75, tourn.game_config().initial_elo());
    tourn.debug_set_target_elo(target, target_elo).unwrap();

    let elo = tourn.get_player_or_default_stats(target).elo();

    assert!(
        (elo - target_elo).abs() < tourn.game_config().logistic_scale() * 0.1,
        "Expected to be near {target_elo}, found {elo}"
    );
}

#[test]
fn set_elo_low() {
    let mut tourn = Tournament::new();
    let target = tourn.register_debug_player().unwrap();
    let target_elo = tourn
        .game_config()
        .logistic_scale()
        .mul_add(-1.5, tourn.game_config().initial_elo());
    tourn.debug_set_target_elo(target, target_elo).unwrap();

    let elo = tourn.get_player_or_default_stats(target).elo();

    assert!(
        (elo - target_elo).abs() < tourn.game_config().logistic_scale() * 0.1,
        "Expected to be near {target_elo}, found {elo}"
    );
}

#[test]
fn set_elo_above() {
    let mut tourn = Tournament::new();
    let target = tourn.register_debug_player().unwrap();
    let target_elo = tourn
        .game_config()
        .logistic_scale()
        .mul_add(0.75, tourn.game_config().initial_elo());
    tourn.debug_set_target_elo(target, target_elo).unwrap();

    let elo = tourn.get_player_or_default_stats(target).elo();

    assert!(
        (elo - target_elo).abs() < tourn.game_config().logistic_scale() * 0.1,
        "Expected to be near {target_elo}, found {elo}"
    );
}

#[test]
fn set_elo_high() {
    let mut tourn = Tournament::new();
    let target = tourn.register_debug_player().unwrap();
    let target_elo = tourn
        .game_config()
        .logistic_scale()
        .mul_add(1.5, tourn.game_config().initial_elo());
    tourn.debug_set_target_elo(target, target_elo).unwrap();

    let elo = tourn.get_player_or_default_stats(target).elo();

    assert!(
        (elo - target_elo).abs() < tourn.game_config().logistic_scale() * 0.1,
        "Expected to be near {target_elo}, found {elo}"
    );
}

#[test]
fn set_many_elos() {
    let mut tourn = Tournament::new();
    let [lowest, low, high, highest] = tourn.register_debug_players().unwrap();
    tourn.debug_set_target_elo(lowest, 1300.0).unwrap();
    tourn.debug_set_target_elo(high, 1600.0).unwrap();
    tourn.debug_set_target_elo(low, 1400.0).unwrap();
    tourn.debug_set_target_elo(highest, 1700.0).unwrap();
}

#[test]
fn set_iterative_elos() {
    let mut tourn = Tournament::new();
    let target = tourn.register_debug_player().unwrap();
    let elos = [1450.0, 1550.0, 1300.0, 1650.0, 1531.0, 1200.0, 1300.0, 1500.0];
    for elo in elos {
        tourn.debug_set_target_elo(target, elo).unwrap();
    }
}
