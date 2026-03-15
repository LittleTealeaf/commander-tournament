use approx::assert_relative_eq;
use edh_tourn::tournament::Tournament;

#[test]
fn default_stats_use_starting_elo() {
    let tournament = Tournament::default();
    let starting_elo = tournament.game_config().starting_elo;
    let stats = tournament.default_stats();
    assert!(starting_elo.total_cmp(&stats.elo()).is_eq());
}

#[test]
fn all_players_start_with_default_elo() {
    let tournament = Tournament::generate_tournament(100, 0).unwrap();
    let starting_elo = tournament.game_config().starting_elo;
    for player in tournament.get_registered_players() {
        assert_relative_eq!(starting_elo, player.stats().elo());
    }
}

#[test]
fn get_player_or_default_returns_default() {
    let mut tourn = Tournament::new();
    let id = tourn.register_player("Sample Player".to_owned()).unwrap();
    let stats = tourn.get_player_or_default_stats(id);
    assert_eq!(tourn.default_stats(), stats);
}

#[test]
fn get_player_or_default_returns_player() {
    let mut tourn = Tournament::new();
    let player_1 = tourn.register_player("1".to_owned()).unwrap();
    let player_2 = tourn.register_player("2".to_owned()).unwrap();
    let player_3 = tourn.register_player("3".to_owned()).unwrap();
    let player_4 = tourn.register_player("4".to_owned()).unwrap();

    tourn
        .register_record(
            tourn
                .create_match([player_1, player_2, player_3, player_4])
                .unwrap()
                .record(player_1)
                .unwrap(),
        )
        .unwrap();

    let stats = tourn.get_player_or_default_stats(player_1);
    assert_ne!(tourn.default_stats(), stats);
}
