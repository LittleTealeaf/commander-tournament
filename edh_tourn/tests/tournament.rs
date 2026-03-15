use edh_tourn::tournament::Tournament;
use itertools::Itertools;

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
    for i in 0..100 {
        tourn.unregister_player(i).unwrap_err();
    }
}

#[test]
fn into_fresh_works_simple() -> anyhow::Result<()> {
    for game in Tournament::test_tournaments() {
        game.into_fresh()?;
    }
    Ok(())
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
fn into_fresh_resets_ids() -> anyhow::Result<()> {
    const REMOVE_COUNT: usize = 40;
    let mut game = Tournament::generate_tournament(100, 0)?;
    let mut ids = game.players().keys().copied().sorted().take(40);
    // Just a dummy test that the first one is 0
    assert_eq!(0, ids.next().unwrap());
    game.unregister_player(0)?;

    for id in ids {
        game.unregister_player(id)?;
    }

    assert_eq!(60, game.players().len());
    assert_eq!(99, *game.players().keys().max().unwrap());

    let new_game = game.into_fresh()?;

    assert_eq!(60, new_game.players().len());
    assert_eq!(59, *new_game.players().keys().max().unwrap());

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
