use edh_tourn::tournament::Tournament;

#[test]
fn new_has_no_players() {
    let new_tourn = Tournament::new();
    assert_eq!(0, new_tourn.players().len());
}

#[test]
fn unregister_removes_players_games() {
    let sample = Tournament::sample_game();
    for id in sample.players().keys() {
        let mut tourn = sample.clone();
        tourn.unregister_player(*id).unwrap();
        for game in tourn.games() {
            assert!(!game.has_player(*id));
            assert_ne!(game.winner(), *id);
        }
    }
}

#[test]
fn unregister_invalid_id_returns_err() {
    let mut tourn = Tournament::new();
    let id = tourn.register_debug_player().unwrap();
    tourn.unregister_player(id).unwrap();
    tourn.unregister_player(id).unwrap_err();
}

#[test]
fn into_fresh_same_players() -> anyhow::Result<()> {
    let game = Tournament::generate_tournament(35, 20)?;
    let new_game = game.into_fresh()?;
    let new_game_players = new_game.players().values().collect::<Vec<_>>();
    for player in game.players().values() {
        assert!(new_game_players.contains(&player));
    }
    assert_eq!(game.players().len(), new_game_players.len());

    Ok(())
}

#[test]
fn into_fresh_same_stats() -> anyhow::Result<()> {
    for game in Tournament::test_tournaments() {
        let new_game = game.into_fresh()?;
        for (id, info) in game.players() {
            let stats = game.get_player_stats(*id);
            let new_id = new_game.get_player_id(info.name()).unwrap();
            let new_stats = new_game.get_player_stats(new_id);
            assert_eq!(stats.is_some(), new_stats.is_some());
            let (Some(stats), Some(new_stats)) = (stats, new_stats) else {
                continue;
            };

            assert!(
                (stats.elo() - new_stats.elo()).abs() <= 1e-9,
                "Elo Difference, from {} to {}",
                stats.elo(),
                new_stats.elo()
            );
        }
    }

    Ok(())
}

#[test]
fn into_fresh_resets_ids() {
    // or your specific Result type
    // use edh_tourn::player::PlayerId; // Not strictly needed if we convert to u32
    use itertools::Itertools;

    let mut tourn = Tournament::generate_tournament(100, 0).unwrap();

    let ids_to_delete = tourn
        .players()
        .keys()
        .sorted()
        .take(40)
        .copied()
        .collect::<Vec<_>>();
    for id in ids_to_delete {
        tourn.unregister_player(id).unwrap();
    }

    assert_eq!(60, tourn.players().len());

    let new_game = tourn.into_fresh().unwrap();

    assert_eq!(60, new_game.players().len());

    assert_ne!(
        new_game.players().keys().max(),
        tourn.players().keys().max()
    );
    assert_ne!(
        new_game.players().keys().min(),
        tourn.players().keys().min()
    );
}

#[test]
fn merge_tournaments_merge_players() {
    let players = ["a", "b", "c", "d"];
    let mut tournament_a = Tournament::new();
    for p in &players {
        tournament_a.register_player(p.to_string()).unwrap();
    }
    let mut tournament_b = Tournament::new();
    for p in &players {
        tournament_b.register_player(p.to_string()).unwrap();
    }

    tournament_a.merge(&tournament_b).unwrap();

    assert_eq!(4, tournament_a.players().len());
}
