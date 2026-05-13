use core::f64;

use approx::{assert_abs_diff_eq, assert_relative_eq};
use edh_tourn::{player::PlayerId, tournament::Tournament};
use itertools::Itertools;

#[test]
fn update_match_same_snapshot() {
    let mut t = Tournament::new();
    let players = t.register_debug_players().unwrap();
    let matchup = t.create_match(players).unwrap();
    let updated = t.update_match(matchup.clone()).unwrap();
    assert!(matchup.eq(&updated));
}

#[test]
fn update_match_newer_snapshot() {
    let mut t = Tournament::new();
    let players = t.register_debug_players().unwrap();
    let matchup = t.create_match(players).unwrap();
    t.reload().unwrap();
    let updated = t.update_match(matchup.clone()).unwrap();
    assert!(!matchup.eq(&updated));
}

#[test]
fn update_record_same_snapshot() {
    let mut t = Tournament::new();
    let players = t.register_debug_players().unwrap();
    let matchup = t.create_match(players).unwrap();
    let record = matchup.debug_record().unwrap();
    let updated = t.update_record(record.clone()).unwrap();
    assert!(record.eq(&updated));
}

#[test]
fn update_record_newer_snapshot() {
    let mut t = Tournament::new();
    let players = t.register_debug_players().unwrap();
    let matchup = t.create_match(players).unwrap();
    let record = matchup.debug_record().unwrap();
    t.reload().unwrap();
    let updated = t.update_record(record.clone()).unwrap();
    assert!(!record.eq(&updated));
}

#[test]
fn mirror_matchup_equal_expected() {
    let mut tourn = Tournament::new();
    let id = tourn.register_player("A".to_owned()).unwrap();
    let mu = tourn.create_match([id, id, id, id]).unwrap();
    for p in mu.players() {
        assert_relative_eq!(0.25, *p.expected());
    }
}

#[test]
fn create_match_unregistered_player() {
    let mut tourn = Tournament::new();
    let players: [_; 4] = tourn.register_debug_players().unwrap();
    for id in &players {
        let mut t = tourn.clone();
        t.unregister_player(*id).unwrap();
        t.create_match(players).unwrap_err();
    }
}

#[test]
fn get_player_games_unregistered() {
    let mut tour = Tournament::generate_tournament(10, 100).unwrap();
    let id = *tour.players().keys().next().unwrap();
    tour.unregister_player(id).unwrap();
    assert!(tour.get_player_games(id).is_err());
}

#[test]
fn delete_index_out_of_bounds() {
    let mut tour = Tournament::generate_tournament(10, 100).unwrap();
    tour.delete_game(101).unwrap_err();
}

#[test]
fn matchup_sum_elo_always_zero() {
    let tourn = Tournament::generate_tournament(20, 100).unwrap();
    for (a, b, c, d) in tourn.players().keys().copied().tuple_windows() {
        let matchup = tourn.create_match((a, b, c, d).into()).unwrap();
        let mut sum_elo = matchup
            .players()
            .iter()
            .map(|player| -1.0 * *player.elo_loss())
            .sum::<f64>();

        for player in matchup.players() {
            // First remove the players' loss
            sum_elo += player.elo_loss();
            // Then, add the win
            sum_elo += player.elo_win();

            assert_abs_diff_eq!(sum_elo, 0.0, epsilon = 1.0e-10);

            sum_elo -= player.elo_loss();
            sum_elo -= player.elo_win();

            // Test that the record elo sum is zero
            let record = matchup.clone().record(player.id()).unwrap();
            let mut sum_elo_change = 0.0;
            for p in record.players() {
                sum_elo_change += record.get_player_elo_change(p.id()).unwrap();
            }
            assert_abs_diff_eq!(sum_elo_change, 0.0, epsilon = 1.0e-10);
        }
    }
}

#[test]
fn winner_gains_elo() {
    for i in 0..4 {
        let mut tourn = Tournament::generate_tournament(4, 0).unwrap();
        let ids: [PlayerId; 4] = tourn.players().keys().take(4).copied().collect_array().unwrap();
        let matchup = tourn.create_match(ids).unwrap();
        let starting_elo = matchup.players().get(i).unwrap().stats().elo();
        let winner = ids.get(i).unwrap();
        let record = matchup.record(*winner).unwrap();
        tourn.record_game(record).unwrap();
        let elo = tourn.get_player_or_default_stats(*winner).elo();
        assert!(
            elo.total_cmp(&starting_elo).is_gt(),
            "Elo {elo} should be greater than starting elo {starting_elo}"
        );
    }
}

#[test]
#[allow(clippy::needless_range_loop)]
#[allow(clippy::indexing_slicing)]
fn loser_loses_elo() -> anyhow::Result<()> {
    for winner_i in 0..4 {
        let tourn = Tournament::generate_tournament(4, 0)?;
        let ids: [PlayerId; 4] = tourn.players().keys().take(4).copied().collect_array().unwrap();
        let winner_id = &ids[winner_i];
        let matchup = tourn.create_match(ids)?;
        for loser_i in 0..4 {
            let mut tourn = tourn.clone();
            let matchup = matchup.clone();
            if winner_i == loser_i {
                continue;
            }
            let loser_id = &ids[loser_i];
            let starting_elo = matchup.players().get(loser_i).unwrap().stats().elo();
            tourn.record_game(matchup.record(*winner_id)?)?;
            let elo = tourn.get_player_or_default_stats(*loser_id).elo();
            assert!(elo.total_cmp(&starting_elo).is_le());
        }
    }

    Ok(())
}

#[test]
#[allow(clippy::needless_range_loop)]
#[allow(clippy::indexing_slicing)]
fn winner_only_counted_once() -> anyhow::Result<()> {
    let mut tourn = Tournament::new();
    let id = tourn.register_player(String::from("sample"))?;
    let matchup = tourn.create_match([id, id, id, id])?;
    let starting_elo = matchup.players()[0].stats().elo();
    tourn.record_game(matchup.record(id)?)?;
    let elo = tourn.get_player_or_default_stats(id).elo();
    assert!(
        (starting_elo - elo).abs() <= 1e-10,
        "Elos do not match: {starting_elo} to {elo}"
    );

    Ok(())
}

#[test]
fn record_winner_must_be_player() {
    let tournament = Tournament::generate_tournament(5, 0).unwrap();
    let mut ids = tournament.players().keys().copied();
    let player_a = ids.next().unwrap();
    let player_b = ids.next().unwrap();
    let player_c = ids.next().unwrap();
    let player_d = ids.next().unwrap();
    let player_e = ids.next().unwrap();

    let mu = tournament
        .create_match([player_a, player_b, player_c, player_d])
        .unwrap();
    mu.clone().record(player_a).unwrap();
    mu.clone().record(player_b).unwrap();
    mu.clone().record(player_c).unwrap();
    mu.clone().record(player_d).unwrap();
    mu.record(player_e).unwrap_err();
}
