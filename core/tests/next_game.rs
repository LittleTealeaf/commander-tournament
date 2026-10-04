use commander_tournament_core::{
    error::TournamentError,
    tournament::{NextPlayerMode, Tournament},
};

#[test]
fn next_game_not_enough_players() {
    let tourn = Tournament::new();
    let result = tourn.next_game(NextPlayerMode::Default);
    assert!(matches!(result, Err(TournamentError::NotEnoughPlayers)));
}

#[test]
fn next_game_with_modes() {
    let mut tourn = Tournament::new();
    for name in ["Alice", "Bob", "Charlie", "Diana"] {
        tourn.register_player(name.to_owned()).unwrap();
    }

    for mode in [
        NextPlayerMode::Default,
        NextPlayerMode::LongestBreak,
        NextPlayerMode::LeastPlayed,
        NextPlayerMode::WinStreak,
        NextPlayerMode::LossStreak,
        NextPlayerMode::GameStreak,
    ] {
        let matchup = tourn.next_game(mode).unwrap();
        assert_eq!(matchup.players().len(), 4);
    }
}
