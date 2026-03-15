use approx::{assert_relative_eq, assert_relative_ne};
use edh_tourn::{game::match_player::MatchPlayer, tournament::Tournament};
use itertools::Itertools;

#[test]
fn update_match_updates_matches() {
    let mut t = Tournament::new();
    let id = t.register_debug_player().unwrap();
    let matchup = t.create_match([id, id, id, id]).unwrap();
    let elo_before = matchup.players().first().unwrap().stats().elo();

    let mut config = t.game_config().clone();
    config.starting_elo += 1500.0;
    let new_elo = config.starting_elo;
    t.set_game_config(config).unwrap();

    assert_relative_ne!(elo_before, t.get_player_or_default_stats(id).elo());
    assert_relative_eq!(elo_before, matchup.players().first().unwrap().stats().elo());

    let updated = t.update_match(matchup).unwrap();
    assert_relative_eq!(new_elo, updated.players().first().unwrap().stats().elo());
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
fn expected_adds_up_to_1() {
    #[allow(clippy::needless_pass_by_value)]
    fn assert_sums_up_to_one<const T: usize>(players: [MatchPlayer; T]) {
        assert_relative_eq!(1.0, players.iter().map(|p| { p.expected() }).sum::<f64>());
    }
    let t = Tournament::generate_tournament(1, 0).unwrap();
    let id = *t.players().keys().next().unwrap();

    assert_sums_up_to_one(t.create_match_players([id, id]));
    assert_sums_up_to_one(t.create_match_players([id, id, id]));
    assert_sums_up_to_one(t.create_match_players([id, id, id, id]));
    assert_sums_up_to_one(t.create_match_players([id, id, id, id, id]));
}

#[test]
fn winner_gains_elo() {
    for i in 0..4 {
        let mut tourn = Tournament::generate_tournament(4, 0).unwrap();
        let ids = tourn.players().keys().copied().collect_vec();
        let mut match_ids = [0; 4];
        match_ids.copy_from_slice(&ids);
        let matchup = tourn.create_match(match_ids).unwrap();
        let starting_elo = matchup.players().get(i).unwrap().stats().elo();
        let winner = match_ids.get(i).unwrap();
        let record = matchup.record(*winner).unwrap();
        tourn.register_record(record).unwrap();
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
        let ids = tourn.players().keys().copied().collect_vec();
        let winner_id = &ids[winner_i];
        let mut match_ids = [0; 4];
        match_ids.copy_from_slice(&ids);
        let matchup = tourn.create_match(match_ids)?;
        for loser_i in 0..4 {
            let mut tourn = tourn.clone();
            let matchup = matchup.clone();
            if winner_i == loser_i {
                continue;
            }
            let loser_id = &ids[loser_i];
            let starting_elo = matchup.players().get(loser_i).unwrap().stats().elo();
            tourn.register_record(matchup.record(*winner_id)?)?;
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
    tourn.register_record(matchup.record(id)?)?;
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
    mu.clone().record(player_e).unwrap_err();
    mu.record(u32::MAX).unwrap_err();
}
