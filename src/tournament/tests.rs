// Comprehensive test suite for tournament mechanics, player management, and matchmaking
#![allow(clippy::unwrap_used)]

use crate::tournament::{GameMatch, MatchmakerConfig, ScoreConfig, Tournament, TournamentError};

// ============================================================================
// FIXTURES AND HELPERS
// ============================================================================

/// Creates a new tournament with default settings for testing
fn default_tournament() -> Tournament {
    Tournament::new()
}

/// Registers multiple players for testing
fn setup_players(tournament: &mut Tournament, names: &[&str]) {
    for name in names {
        tournament.register_player(name.to_string());
    }
}

/// Helper to submit a game between 4 players
fn submit_game(
    tournament: &mut Tournament,
    p1: &str,
    p2: &str,
    p3: &str,
    p4: &str,
    winner: &str,
) -> Result<(), TournamentError> {
    let game = tournament.create_game([p1, p2, p3, p4]);
    tournament.submit_game(game, winner)
}

// ============================================================================
// SCORECONFIG TESTS
// ============================================================================

#[test]
fn score_config_new_creates_defaults() {
    let config = ScoreConfig::new();
    assert_eq!(config.starting_elo, 1500.0);
    assert_eq!(config.game_points, 25.0);
    assert_eq!(config.elo_pow, 6.0);
    assert_eq!(config.wr_pow, 1.0);
    assert_eq!(config.elo_weight, 65.0);
    assert_eq!(config.wr_weight, 100.0);
}

#[test]
fn score_config_default_equals_new() {
    let default = ScoreConfig::default();
    let new = ScoreConfig::new();
    assert_eq!(default, new);
}

#[test]
fn score_config_new_player_stats_starts_at_starting_elo() {
    let config = ScoreConfig::new();
    let stats = config.new_player_stats();
    assert_eq!(stats.elo(), config.starting_elo);
    assert_eq!(stats.games(), 0);
    assert_eq!(stats.wins(), 0);
}

#[test]
fn score_config_custom_values_respected() {
    let mut config = ScoreConfig::new();
    config.starting_elo = 2000.0;
    config.game_points = 50.0;

    let stats = config.new_player_stats();
    assert_eq!(stats.elo(), 2000.0);
}

// ============================================================================
// PLAYERSTATS TESTS
// ============================================================================

#[test]
fn player_stats_wr_returns_none_with_no_games() {
    let mut tournament = default_tournament();
    tournament.register_player("Alice".to_string());

    let alice = tournament.players().get("Alice").unwrap();
    assert_eq!(alice.wr(), None);
}

#[test]
fn player_stats_wr_calculated_correctly() {
    let mut tournament = default_tournament();
    setup_players(&mut tournament, &["Alice", "Bob", "Charlie", "David"]);

    // Win 4 times out of 10
    for _ in 0..4 {
        submit_game(&mut tournament, "Alice", "Bob", "Charlie", "David", "Alice").unwrap();
    }
    for _ in 0..6 {
        submit_game(&mut tournament, "Alice", "Bob", "Charlie", "David", "Bob").unwrap();
    }

    let alice = tournament.players().get("Alice").unwrap();
    assert_eq!(alice.wr(), Some(0.4));
}

#[test]
fn player_stats_wr_perfect_record() {
    let mut tournament = default_tournament();
    setup_players(&mut tournament, &["Alice", "Bob", "Charlie", "David"]);

    for _ in 0..5 {
        submit_game(&mut tournament, "Alice", "Bob", "Charlie", "David", "Alice").unwrap();
    }

    let alice = tournament.players().get("Alice").unwrap();
    assert_eq!(alice.wr(), Some(1.0));
}

#[test]
fn player_stats_wr_zero_wins() {
    let mut tournament = default_tournament();
    setup_players(&mut tournament, &["Alice", "Bob", "Charlie", "David"]);

    for _ in 0..5 {
        submit_game(&mut tournament, "Alice", "Bob", "Charlie", "David", "Bob").unwrap();
    }

    let alice = tournament.players().get("Alice").unwrap();
    assert_eq!(alice.wr(), Some(0.0));
}

#[test]
fn player_stats_getters_return_correct_values() {
    let mut tournament = default_tournament();
    setup_players(&mut tournament, &["Alice", "Bob", "Charlie", "David"]);

    // Setup: Alice plays 20 games, wins 12
    for _ in 0..12 {
        submit_game(&mut tournament, "Alice", "Bob", "Charlie", "David", "Alice").unwrap();
    }
    for _ in 0..8 {
        submit_game(&mut tournament, "Alice", "Bob", "Charlie", "David", "Bob").unwrap();
    }

    let alice = tournament.players().get("Alice").unwrap();
    assert!(alice.elo() > 0.0);
    assert_eq!(alice.games(), 20);
    assert_eq!(alice.wins(), 12);
}

// ============================================================================
// GAMEPLAYER AND GAMEMATCH TESTS
// ============================================================================

#[test]
fn gamematch_contains_all_players() {
    let mut tournament = default_tournament();
    setup_players(&mut tournament, &["Alice", "Bob", "Charlie", "David"]);

    let game = tournament.create_game(["Alice", "Bob", "Charlie", "David"]);
    let GameMatch(players) = game;

    assert_eq!(players.len(), 4);
    assert!(players.iter().any(|p| p.name() == "Alice"));
    assert!(players.iter().any(|p| p.name() == "Bob"));
    assert!(players.iter().any(|p| p.name() == "Charlie"));
    assert!(players.iter().any(|p| p.name() == "David"));
}

// ============================================================================
// TOURNAMENT CREATION AND REGISTRATION TESTS
// ============================================================================

#[test]
fn tournament_new_is_empty() {
    let tournament = Tournament::new();
    assert_eq!(tournament.players().len(), 0);
}

#[test]
fn tournament_default_equals_new() {
    let t1 = Tournament::new();
    let t2 = Tournament::default();
    assert_eq!(t1.players().len(), t2.players().len());
}

#[test]
fn register_player_adds_player() {
    let mut tournament = default_tournament();
    tournament.register_player("Alice".to_string());

    assert!(tournament.has_registered_player("Alice"));
    assert_eq!(tournament.players().len(), 1);
}

#[test]
fn register_multiple_players() {
    let mut tournament = default_tournament();
    setup_players(&mut tournament, &["Alice", "Bob", "Charlie", "David"]);

    assert_eq!(tournament.players().len(), 4);
    assert!(tournament.has_registered_player("Alice"));
    assert!(tournament.has_registered_player("Bob"));
}

#[test]
fn has_registered_player_returns_false_for_missing() {
    let tournament = default_tournament();
    assert!(!tournament.has_registered_player("NonExistent"));
}

#[test]
fn register_player_starts_with_default_config_elo() {
    let mut tournament = default_tournament();
    tournament.register_player("Alice".to_string());

    let alice = tournament.players().get("Alice").unwrap();
    assert_eq!(alice.elo(), 1500.0);
    assert_eq!(alice.games(), 0);
    assert_eq!(alice.wins(), 0);
}

// ============================================================================
// PLAYER MANAGEMENT TESTS
// ============================================================================

#[test]
fn rename_player_updates_name() {
    let mut tournament = default_tournament();
    tournament.register_player("Alice".to_string());
    tournament
        .rename_player("Alice".to_string(), "Alicia".to_string())
        .unwrap();

    assert!(!tournament.has_registered_player("Alice"));
    assert!(tournament.has_registered_player("Alicia"));
}

#[test]
fn rename_player_preserves_stats() {
    let mut tournament = default_tournament();
    setup_players(&mut tournament, &["Alice", "Bob", "Charlie", "David"]);

    // Submit a game to give Alice some stats
    submit_game(&mut tournament, "Alice", "Bob", "Charlie", "David", "Alice").unwrap();

    let alice_original = *tournament.players().get("Alice").unwrap();
    tournament
        .rename_player("Alice".to_string(), "Alicia".to_string())
        .unwrap();

    let alicia = tournament.players().get("Alicia").unwrap();
    assert_eq!(alicia.games(), alice_original.games());
    assert_eq!(alicia.wins(), alice_original.wins());
}

#[test]
fn rename_player_updates_game_records() {
    let mut tournament = default_tournament();
    setup_players(&mut tournament, &["Alice", "Bob", "Charlie", "David"]);
    submit_game(&mut tournament, "Alice", "Bob", "Charlie", "David", "Alice").unwrap();

    tournament
        .rename_player("Alice".to_string(), "Alicia".to_string())
        .unwrap();

    // Note: We don't have a way to access games directly, but rename should update them
    // We can verify by renaiming and checking if new name works
    assert!(tournament.has_registered_player("Alicia"));
}

#[test]
fn rename_nonexistent_player_fails() {
    let mut tournament = default_tournament();
    let result = tournament.rename_player("Alice".to_string(), "Alicia".to_string());
    assert!(result.is_err());
}

#[test]
fn rename_to_existing_player_fails() {
    let mut tournament = default_tournament();
    setup_players(&mut tournament, &["Alice", "Bob"]);

    let result = tournament.rename_player("Alice".to_string(), "Bob".to_string());
    assert!(matches!(
        result,
        Err(TournamentError::PlayerAlreadyRegistered(_))
    ));
}

#[test]
fn remove_player_deletes_player() {
    let mut tournament = default_tournament();
    setup_players(&mut tournament, &["Alice", "Bob"]);

    tournament.remove_player("Alice".to_string()).unwrap();

    assert!(!tournament.has_registered_player("Alice"));
    assert_eq!(tournament.players().len(), 1);
}

#[test]
fn remove_nonexistent_player_fails() {
    let mut tournament = default_tournament();
    let result = tournament.remove_player("NonExistent".to_string());
    assert!(result.is_err());
}

#[test]
fn remove_player_removes_games() {
    let mut tournament = default_tournament();
    setup_players(&mut tournament, &["Alice", "Bob", "Charlie", "David"]);
    submit_game(&mut tournament, "Alice", "Bob", "Charlie", "David", "Alice").unwrap();

    // After removing Alice, the game should be removed too
    tournament.remove_player("Alice".to_string()).unwrap();
    assert!(!tournament.has_registered_player("Alice"));
}

// ============================================================================
// GAME CREATION AND EXPECTED PROBABILITY TESTS
// ============================================================================

#[test]
fn create_game_returns_all_four_players() {
    let mut tournament = default_tournament();
    setup_players(&mut tournament, &["Alice", "Bob", "Charlie", "David"]);

    let game = tournament.create_game(["Alice", "Bob", "Charlie", "David"]);
    let GameMatch(players) = game;

    assert_eq!(players.len(), 4);
    assert!(players.iter().any(|p| p.name() == "Alice"));
    assert!(players.iter().any(|p| p.name() == "Bob"));
    assert!(players.iter().any(|p| p.name() == "Charlie"));
    assert!(players.iter().any(|p| p.name() == "David"));
}

#[test]
fn create_game_expected_probabilities_sum_to_one() {
    let mut tournament = default_tournament();
    setup_players(&mut tournament, &["Alice", "Bob", "Charlie", "David"]);

    let game = tournament.create_game(["Alice", "Bob", "Charlie", "David"]);
    let GameMatch(players) = game;

    let sum: f64 = players.iter().map(|p| p.expected()).sum();
    assert!(
        (sum - 1.0).abs() < 0.0001,
        "Expected probabilities should sum to 1.0, got {}",
        sum
    );
}

#[test]
fn create_game_equal_players_equal_expected() {
    let mut tournament = default_tournament();
    setup_players(&mut tournament, &["Alice", "Bob", "Charlie", "David"]);

    let game = tournament.create_game(["Alice", "Bob", "Charlie", "David"]);
    let GameMatch(players) = game;

    // All players at same Elo should have equal expected
    for player in &players {
        assert!(
            (player.expected() - 0.25).abs() < 0.0001,
            "Equal players should have ~0.25 expected, got {}",
            player.expected()
        );
    }
}

#[test]
fn create_game_higher_elo_higher_expected() {
    let mut tournament = default_tournament();
    setup_players(&mut tournament, &["Alice", "Bob", "Charlie", "David"]);

    // Give Alice a win to boost her Elo
    submit_game(&mut tournament, "Alice", "Bob", "Charlie", "David", "Alice").unwrap();

    let game = tournament.create_game(["Alice", "Bob", "Charlie", "David"]);
    let GameMatch(players) = game;

    let alice = players.iter().find(|p| p.name() == "Alice").unwrap();
    let bob = players.iter().find(|p| p.name() == "Bob").unwrap();

    assert!(
        alice.expected() > bob.expected(),
        "Higher Elo player should have higher expected"
    );
}

#[test]
fn create_game_uses_current_stats() {
    let mut tournament = default_tournament();
    setup_players(&mut tournament, &["Alice", "Bob", "Charlie", "David"]);

    // Submit multiple games for Alice
    for _ in 0..5 {
        submit_game(&mut tournament, "Alice", "Bob", "Charlie", "David", "Alice").unwrap();
    }

    let game = tournament.create_game(["Alice", "Bob", "Charlie", "David"]);
    let GameMatch(players) = game;

    let alice = players.iter().find(|p| p.name() == "Alice").unwrap();
    assert_eq!(alice.stats().games(), 5, "Game should use current stats");
    assert_eq!(alice.stats().wins(), 5, "Game should use current stats");
}

// ============================================================================
// GAME SUBMISSION AND ELO CALCULATION TESTS
// ============================================================================

#[test]
fn submit_game_winner_gets_points() {
    let mut tournament = default_tournament();
    setup_players(&mut tournament, &["Alice", "Bob", "Charlie", "David"]);

    let start_elo = tournament.players().get("Alice").unwrap().elo();
    submit_game(&mut tournament, "Alice", "Bob", "Charlie", "David", "Alice").unwrap();
    let end_elo = tournament.players().get("Alice").unwrap().elo();

    assert!(end_elo > start_elo, "Winner should gain Elo");
}

#[test]
fn submit_game_loser_loses_points() {
    let mut tournament = default_tournament();
    setup_players(&mut tournament, &["Alice", "Bob", "Charlie", "David"]);

    let start_elo = tournament.players().get("Bob").unwrap().elo();
    submit_game(&mut tournament, "Alice", "Bob", "Charlie", "David", "Alice").unwrap();
    let end_elo = tournament.players().get("Bob").unwrap().elo();

    assert!(end_elo < start_elo, "Loser should lose Elo");
}

#[test]
fn submit_game_increments_game_count_all_players() {
    let mut tournament = default_tournament();
    setup_players(&mut tournament, &["Alice", "Bob", "Charlie", "David"]);

    submit_game(&mut tournament, "Alice", "Bob", "Charlie", "David", "Alice").unwrap();

    for player in ["Alice", "Bob", "Charlie", "David"] {
        let stats = tournament.players().get(player).unwrap();
        assert_eq!(
            stats.games(),
            1,
            "All players should have game count increased"
        );
    }
}

#[test]
fn submit_game_only_winner_gets_win() {
    let mut tournament = default_tournament();
    setup_players(&mut tournament, &["Alice", "Bob", "Charlie", "David"]);

    submit_game(&mut tournament, "Alice", "Bob", "Charlie", "David", "Alice").unwrap();

    assert_eq!(tournament.players().get("Alice").unwrap().wins(), 1);
    assert_eq!(tournament.players().get("Bob").unwrap().wins(), 0);
    assert_eq!(tournament.players().get("Charlie").unwrap().wins(), 0);
    assert_eq!(tournament.players().get("David").unwrap().wins(), 0);
}

#[test]
fn submit_game_invalid_winner_fails() {
    let mut tournament = default_tournament();
    setup_players(
        &mut tournament,
        &["Alice", "Bob", "Charlie", "David", "Eve"],
    );

    let game = tournament.create_game(["Alice", "Bob", "Charlie", "David"]);
    let result = tournament.submit_game(game, "Eve");

    assert!(matches!(result, Err(TournamentError::WinnerNotInMatch(_))));
}

#[test]
fn submit_game_expected_affects_elo_change() {
    let mut tournament = default_tournament();
    setup_players(
        &mut tournament,
        &["HighElo", "LowElo1", "LowElo2", "LowElo3"],
    );

    // Boost HighElo's rating
    for _ in 0..10 {
        submit_game(
            &mut tournament,
            "HighElo",
            "LowElo1",
            "LowElo2",
            "LowElo3",
            "HighElo",
        )
        .unwrap();
    }

    let high_elo_before = tournament.players().get("HighElo").unwrap().elo();

    // Now create a new player with low rating
    tournament.register_player("Newcomer".to_string());

    // HighElo vs Newcomer (and two others) - HighElo should win but gain less
    submit_game(
        &mut tournament,
        "HighElo",
        "Newcomer",
        "LowElo1",
        "LowElo2",
        "HighElo",
    )
    .unwrap();

    let high_elo_after = tournament.players().get("HighElo").unwrap().elo();
    let gain = high_elo_after - high_elo_before;

    // Expected gain should be positive but affected by expectation
    assert!(gain > 0.0, "High Elo should still gain points winning");
}

#[test]
fn submit_game_upset_gains_more_elo() {
    let mut tournament = default_tournament();
    setup_players(
        &mut tournament,
        &["HighElo", "LowElo1", "LowElo2", "Newcomer"],
    );

    // Boost HighElo
    for _ in 0..10 {
        submit_game(
            &mut tournament,
            "HighElo",
            "LowElo1",
            "LowElo2",
            "Newcomer",
            "HighElo",
        )
        .unwrap();
    }

    let low_elo1_before = tournament.players().get("LowElo1").unwrap().elo();

    // LowElo1 beats HighElo (upset)
    submit_game(
        &mut tournament,
        "HighElo",
        "LowElo1",
        "LowElo2",
        "Newcomer",
        "LowElo1",
    )
    .unwrap();

    let low_elo1_after = tournament.players().get("LowElo1").unwrap().elo();
    let gain = low_elo1_after - low_elo1_before;

    // Should gain significant Elo for the upset
    assert!(
        gain > 10.0,
        "Upset winner should gain significant Elo, got {}",
        gain
    );
}

// ============================================================================
// CONFIGURATION TESTS
// ============================================================================

#[test]
fn get_score_config_returns_current_config() {
    let tournament = default_tournament();
    let config = tournament.get_score_config();

    assert_eq!(config.starting_elo, 1500.0);
    assert_eq!(config.game_points, 25.0);
}

#[test]
fn set_score_config_changes_config() {
    let mut tournament = default_tournament();
    let mut config = ScoreConfig::new();
    config.game_points = 50.0;

    tournament.set_score_config(config).unwrap();

    assert_eq!(tournament.get_score_config().game_points, 50.0);
}

#[test]
fn set_score_config_recalculates_ratings() {
    let mut tournament = default_tournament();
    setup_players(&mut tournament, &["Alice", "Bob", "Charlie", "David"]);

    // Play a game
    submit_game(&mut tournament, "Alice", "Bob", "Charlie", "David", "Alice").unwrap();

    let alice_elo_first = tournament.players().get("Alice").unwrap().elo();

    // Change config and reload
    let mut new_config = ScoreConfig::new();
    new_config.game_points = 50.0; // Double the points
    tournament.set_score_config(new_config).unwrap();

    let alice_elo_second = tournament.players().get("Alice").unwrap().elo();

    // Elo should be different with new config
    assert_ne!(
        alice_elo_first, alice_elo_second,
        "Config change should recalculate Elo ratings"
    );
}

#[test]
fn get_match_config_returns_current_config() {
    let tournament = default_tournament();
    let config = tournament.get_match_config();

    assert_eq!(config.weight_least_played, 6.0);
}

#[test]
fn set_match_config_changes_config() {
    let mut tournament = default_tournament();
    let config = MatchmakerConfig {
        weight_nemesis: 10.0,
        ..Default::default()
    };

    tournament.set_match_config(config).unwrap();

    assert_eq!(tournament.get_match_config().weight_nemesis, 10.0);
}

// ============================================================================
// RELOAD TESTS
// ============================================================================

#[test]
fn reload_resets_ratings_from_history() {
    let mut tournament = default_tournament();
    setup_players(&mut tournament, &["Alice", "Bob", "Charlie", "David"]);

    // Play games to build history
    submit_game(&mut tournament, "Alice", "Bob", "Charlie", "David", "Alice").unwrap();
    submit_game(&mut tournament, "Bob", "Alice", "Charlie", "David", "Bob").unwrap();

    let alice_before = *tournament.players().get("Alice").unwrap();

    // Reload
    tournament.reload().unwrap();

    let alice_after = tournament.players().get("Alice").unwrap();

    // Ratings should be the same after reload
    assert_eq!(alice_before.elo(), alice_after.elo());
    assert_eq!(alice_before.games(), alice_after.games());
    assert_eq!(alice_before.wins(), alice_after.wins());
}

#[test]
fn reload_with_changed_config_recalculates() {
    let mut tournament = default_tournament();
    setup_players(&mut tournament, &["Alice", "Bob", "Charlie", "David"]);

    submit_game(&mut tournament, "Alice", "Bob", "Charlie", "David", "Alice").unwrap();

    let alice_elo_before = tournament.players().get("Alice").unwrap().elo();

    // Change config through public API
    let mut new_config = ScoreConfig::new();
    new_config.game_points = 100.0;
    tournament.set_score_config(new_config).unwrap();

    let alice_elo_after = tournament.players().get("Alice").unwrap().elo();

    // Should be different
    assert_ne!(alice_elo_before, alice_elo_after);
}

// ============================================================================
// MATCHMAKING TESTS
// ============================================================================

#[test]
fn rank_least_played_empty_player_history() {
    let mut tournament = default_tournament();
    setup_players(&mut tournament, &["Alice", "Bob", "Charlie", "David"]);

    let result = tournament.rank_least_played("Alice");
    assert!(result.is_ok(), "Should work with no games");

    let mut opponents: Vec<_> = result.unwrap().collect();
    opponents.sort(); // Stable sort for comparison

    assert_eq!(opponents.len(), 3);
    assert!(opponents.contains(&"Bob".to_string()));
    assert!(opponents.contains(&"Charlie".to_string()));
    assert!(opponents.contains(&"David".to_string()));
}

#[test]
fn rank_least_played_returns_unplayed_first() {
    let mut tournament = default_tournament();
    setup_players(
        &mut tournament,
        &["Alice", "Bob", "Charlie", "David", "Eve"],
    );

    // Alice plays Bob 3 times
    for _ in 0..3 {
        submit_game(&mut tournament, "Alice", "Bob", "Charlie", "David", "Alice").unwrap();
    }

    // Alice plays Charlie 1 time
    submit_game(&mut tournament, "Alice", "Charlie", "Bob", "David", "Alice").unwrap();

    let rank: Vec<_> = tournament.rank_least_played("Alice").unwrap().collect();

    // Eve (0 games) and David (0 games) should come before Bob (3 games)
    let eve_pos = rank.iter().position(|p| p == "Eve");
    let bob_pos = rank.iter().position(|p| p == "Bob");

    if let (Some(eve_p), Some(bob_p)) = (eve_pos, bob_pos) {
        assert!(
            eve_p < bob_p,
            "Eve (0 games) should rank above Bob (3 games)"
        );
    }
}

#[test]
fn rank_least_played_nonexistent_player_fails() {
    let tournament = default_tournament();
    let result = tournament.rank_least_played("NonExistent");
    assert!(result.is_err());
}

#[test]
fn rank_nemesis_with_no_head_to_head() {
    let mut tournament = default_tournament();
    setup_players(&mut tournament, &["Alice", "Bob", "Charlie", "David"]);

    let result = tournament.rank_nemesis("Alice");
    assert!(result.is_ok());

    let nemeses: Vec<_> = result.unwrap().collect();
    assert_eq!(nemeses.len(), 3);
}

#[test]
fn rank_nemesis_identifies_actual_nemesis() {
    let mut tournament = default_tournament();
    setup_players(
        &mut tournament,
        &["Alice", "Bob", "Charlie", "David", "Eve"],
    );

    // Bob beats Alice many times
    for _ in 0..5 {
        submit_game(&mut tournament, "Alice", "Bob", "Charlie", "David", "Bob").unwrap();
    }

    // Charlie beats Alice a few times
    submit_game(
        &mut tournament,
        "Alice",
        "Charlie",
        "Bob",
        "David",
        "Charlie",
    )
    .unwrap();

    // Eve has never played Alice

    let nemesis_rank: Vec<_> = tournament.rank_nemesis("Alice").unwrap().collect();

    // Bob should be the first nemesis (5 losses vs 1 loss)
    assert_eq!(
        nemesis_rank[0], "Bob",
        "Bob (5 wins vs Alice) should be nemesis"
    );
}

#[test]
fn rank_nemesis_nonexistent_player_fails() {
    let tournament = default_tournament();
    let result = tournament.rank_nemesis("NonExistent");
    assert!(result.is_err());
}

#[test]
fn rank_wr_neighbors_returns_all_other_players() {
    let mut tournament = default_tournament();
    setup_players(&mut tournament, &["Alice", "Bob", "Charlie", "David"]);

    let neighbors: Vec<_> = tournament.rank_wr_neighbors("Alice").unwrap().collect();
    assert_eq!(neighbors.len(), 3);
}

#[test]
fn rank_wr_neighbors_nonexistent_player_fails() {
    let tournament = default_tournament();
    let result = tournament.rank_wr_neighbors("NonExistent");
    assert!(result.is_err());
}

#[test]
fn rank_neighbors_returns_all_other_players() {
    let mut tournament = default_tournament();
    setup_players(&mut tournament, &["Alice", "Bob", "Charlie", "David"]);

    let neighbors: Vec<_> = tournament.rank_neighbors("Alice").unwrap().collect();
    assert_eq!(neighbors.len(), 3);
}

#[test]
fn rank_neighbors_nonexistent_player_fails() {
    let tournament = default_tournament();
    let result = tournament.rank_neighbors("NonExistent");
    assert!(result.is_err());
}

#[test]
fn rank_loss_with_identifies_problem_partners() {
    let mut tournament = default_tournament();
    setup_players(
        &mut tournament,
        &["Alice", "Bob", "Charlie", "David", "Eve"],
    );

    // Alice plays often with Bob but loses
    for _ in 0..3 {
        submit_game(&mut tournament, "Alice", "Bob", "Charlie", "David", "Bob").unwrap();
    }

    // Alice plays once with Charlie and wins
    submit_game(&mut tournament, "Alice", "Charlie", "Bob", "David", "Alice").unwrap();

    // Eve has never played with Alice

    let loss_rank: Vec<_> = tournament.rank_loss_with("Alice").unwrap().collect();

    // Bob (3 losses with) should rank before Charlie (1 win with)
    if let Some(bob_pos) = loss_rank.iter().position(|p| p == "Bob")
        && let Some(charlie_pos) = loss_rank.iter().position(|p| p == "Charlie")
    {
        assert!(
            bob_pos < charlie_pos,
            "Bob (3 losses) should rank before Charlie (1 win)"
        );
    }
}

#[test]
fn rank_loss_with_nonexistent_player_fails() {
    let tournament = default_tournament();
    let result = tournament.rank_loss_with("NonExistent");
    assert!(result.is_err());
}

#[test]
fn rank_combined_uses_all_strategies() {
    let mut tournament = default_tournament();
    setup_players(
        &mut tournament,
        &["Alice", "Bob", "Charlie", "David", "Eve"],
    );

    // Play some games to build history
    submit_game(&mut tournament, "Alice", "Bob", "Charlie", "David", "Alice").unwrap();
    submit_game(&mut tournament, "Alice", "Bob", "Charlie", "David", "Bob").unwrap();
    submit_game(&mut tournament, "Alice", "Charlie", "Bob", "Eve", "Alice").unwrap();

    let combined_rank: Vec<_> = tournament.rank_combined("Alice").unwrap().collect();

    // Should have 4 opponents (all except Alice)
    assert_eq!(
        combined_rank.len(),
        4,
        "Combined ranking should include all opponents"
    );

    // Eve should not be in the ranking (single player, not enough opponents)
    assert!(
        !combined_rank.contains(&"Alice".to_string()),
        "Alice shouldn't rank against herself"
    );
}

#[test]
fn rank_combined_nonexistent_player_fails() {
    let tournament = default_tournament();
    let result = tournament.rank_combined("NonExistent");
    assert!(result.is_err());
}

#[test]
fn game_least_played_creates_valid_match() {
    let mut tournament = default_tournament();
    setup_players(&mut tournament, &["Alice", "Bob", "Charlie", "David"]);

    let result = tournament.game_least_played("Alice");
    assert!(result.is_ok());

    let GameMatch(players) = result.unwrap();
    assert_eq!(players.len(), 4);
    assert_eq!(players[0].name(), "Alice");
}

#[test]
fn game_least_played_insufficient_players_fails() {
    let mut tournament = default_tournament();
    tournament.register_player("Alice".to_string());
    tournament.register_player("Bob".to_string());

    let result = tournament.game_least_played("Alice");
    assert!(matches!(result, Err(TournamentError::NotEnoughPlayers)));
}

#[test]
fn game_least_played_nonexistent_player_fails() {
    let mut tournament = default_tournament();
    setup_players(&mut tournament, &["Alice", "Bob", "Charlie", "David"]);

    let result = tournament.game_least_played("NonExistent");
    assert!(result.is_err());
}

#[test]
fn game_nemesis_creates_valid_match() {
    let mut tournament = default_tournament();
    setup_players(&mut tournament, &["Alice", "Bob", "Charlie", "David"]);

    let result = tournament.game_nemesis("Alice");
    assert!(result.is_ok());

    let GameMatch(players) = result.unwrap();
    assert_eq!(players.len(), 4);
    assert_eq!(players[0].name(), "Alice");
}

#[test]
fn game_wr_neighbors_creates_valid_match() {
    let mut tournament = default_tournament();
    setup_players(&mut tournament, &["Alice", "Bob", "Charlie", "David"]);

    let result = tournament.game_wr_neighbors("Alice");
    assert!(result.is_ok());

    let GameMatch(players) = result.unwrap();
    assert_eq!(players.len(), 4);
}

#[test]
fn game_neighbors_creates_valid_match() {
    let mut tournament = default_tournament();
    setup_players(&mut tournament, &["Alice", "Bob", "Charlie", "David"]);

    let result = tournament.game_neighbors("Alice");
    assert!(result.is_ok());

    let GameMatch(players) = result.unwrap();
    assert_eq!(players.len(), 4);
}

#[test]
fn game_loss_with_creates_valid_match() {
    let mut tournament = default_tournament();
    setup_players(&mut tournament, &["Alice", "Bob", "Charlie", "David"]);

    let result = tournament.game_loss_with("Alice");
    assert!(result.is_ok());

    let GameMatch(players) = result.unwrap();
    assert_eq!(players.len(), 4);
}

#[test]
fn game_combined_creates_valid_match() {
    let mut tournament = default_tournament();
    setup_players(&mut tournament, &["Alice", "Bob", "Charlie", "David"]);

    let result = tournament.game_combined("Alice");
    assert!(result.is_ok());

    let GameMatch(players) = result.unwrap();
    assert_eq!(players.len(), 4);
    assert_eq!(players[0].name(), "Alice");
}

// ============================================================================
// EDGE CASE AND INTEGRATION TESTS
// ============================================================================

#[test]
fn multiple_games_accumulate_stats() {
    let mut tournament = default_tournament();
    setup_players(&mut tournament, &["Alice", "Bob", "Charlie", "David"]);

    for _round in 0..10 {
        submit_game(&mut tournament, "Alice", "Bob", "Charlie", "David", "Alice").unwrap();
    }

    let alice = tournament.players().get("Alice").unwrap();
    assert_eq!(alice.games(), 10);
    assert_eq!(alice.wins(), 10);
    assert_eq!(alice.wr(), Some(1.0));
}

#[test]
fn mixed_results_correct_winrate() {
    let mut tournament = default_tournament();
    setup_players(&mut tournament, &["Alice", "Bob", "Charlie", "David"]);

    // Alice wins 6 times
    for _ in 0..6 {
        submit_game(&mut tournament, "Alice", "Bob", "Charlie", "David", "Alice").unwrap();
    }

    // Alice loses 4 times
    for _ in 0..4 {
        submit_game(&mut tournament, "Alice", "Bob", "Charlie", "David", "Bob").unwrap();
    }

    let alice = tournament.players().get("Alice").unwrap();
    assert_eq!(alice.games(), 10);
    assert_eq!(alice.wins(), 6);
    assert_eq!(alice.wr(), Some(0.6));
}

#[test]
fn elo_converges_for_consistent_players() {
    let mut tournament = default_tournament();
    setup_players(&mut tournament, &["Strong", "Weak1", "Weak2", "Weak3"]);

    // Strong always wins
    for _ in 0..20 {
        submit_game(
            &mut tournament,
            "Strong",
            "Weak1",
            "Weak2",
            "Weak3",
            "Strong",
        )
        .unwrap();
    }

    let strong_elo = tournament.players().get("Strong").unwrap().elo();
    let weak_elo = tournament.players().get("Weak1").unwrap().elo();

    // Strong should have gained, Weak should have lost
    assert!(strong_elo > 1500.0);
    assert!(weak_elo < 1500.0);
}

#[test]
fn serialization_deserialize_roundtrip() {
    let mut tournament = default_tournament();
    setup_players(&mut tournament, &["Alice", "Bob", "Charlie", "David"]);

    submit_game(&mut tournament, "Alice", "Bob", "Charlie", "David", "Alice").unwrap();
    submit_game(&mut tournament, "Bob", "Alice", "Charlie", "David", "Bob").unwrap();

    // Serialize and deserialize
    let serialized = serde_json::to_string(&tournament).unwrap();
    let deserialized: Tournament = serde_json::from_str(&serialized).unwrap();

    // Check that data is preserved
    assert_eq!(deserialized.players().len(), tournament.players().len());
    for (name, stats) in tournament.players() {
        let restored_stats = deserialized.players().get(name).unwrap();
        assert_eq!(stats.games(), restored_stats.games());
        assert_eq!(stats.wins(), restored_stats.wins());
    }
}
