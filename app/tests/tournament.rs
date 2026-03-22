use app::core::tournament::Action;
use approx::assert_relative_eq;
use edh_tourn::{
    game::{entry::GameEntry, record::GameRecord},
    player::info::PlayerInfo,
    tournament::Tournament,
};

#[test]
fn register() {
    let mut tournament = Tournament::new();
    let info = PlayerInfo::new("Hello".to_owned());
    Action::Register(info.clone())
        .apply(&mut tournament)
        .unwrap();
    let _ = tournament.get_player_id(info.name()).unwrap();
}

#[test]
fn set_player_info() {
    let mut tournament = Tournament::new();
    let id = tournament.register_debug_player().unwrap();
    let name = tournament.get_player_name(&id).unwrap();
    let new_name = format!("new-{name}");
    let info = PlayerInfo::new(new_name.clone());
    Action::SetPlayerInfo(id, info)
        .apply(&mut tournament)
        .unwrap();
    assert_eq!(&new_name, tournament.get_player_name(&id).unwrap());

    Action::SetPlayerInfo(id, PlayerInfo::new(String::new()))
        .apply(&mut tournament)
        .unwrap_err();
}

#[test]
fn delete_player() {
    let mut tournament = Tournament::generate_tournament(50, 50).unwrap();
    // Gets an id of a player that is in at least one game
    let id = tournament
        .games()
        .iter()
        .map(GameRecord::winner)
        .next()
        .unwrap();

    Action::DeletePlayer(id).apply(&mut tournament).unwrap();

    // Player does not exist
    assert!(tournament.get_registered_player(id).is_none());

    // No games have the player
    for game in tournament.games() {
        assert!(!game.has_player(id));
    }
}

#[test]
fn delete_game() {
    let mut tournament = Tournament::generate_tournament(20, 100).unwrap();
    let id = 5;
    let next_record = GameEntry::from(tournament.games().get(id + 1).unwrap().clone());
    Action::DeleteGame(id).apply(&mut tournament).unwrap();
    let record = GameEntry::from(tournament.games().get(id).unwrap().clone());
    assert_eq!(record, next_record);
    assert_eq!(99, tournament.games().len());
}

#[test]
fn record_game() {
    let mut tournament = Tournament::generate_tournament(20, 50).unwrap();
    let game = tournament.random_game().unwrap();
    let matchup = tournament.create_match(*game.players()).unwrap();
    let record = matchup.record(game.winner()).unwrap();

    Action::Record(Box::new(record))
        .apply(&mut tournament)
        .unwrap();
    assert_eq!(51, tournament.games().len());
}

#[test]
fn reload() {
    let mut tournament = Tournament::generate_tournament(20, 20).unwrap();
    let initial = tournament.snapshot();
    Action::Reload.apply(&mut tournament).unwrap();
    assert_eq!(initial + 1, tournament.snapshot());
}

#[test]
fn set_game_config() {
    let mut tournament = Tournament::new();
    let id = tournament.register_debug_player().unwrap();
    let starting_elo = tournament.get_player_or_default_stats(id).elo();
    let mut config = tournament.game_config().clone();
    config.starting_elo += 1500.0;
    Action::SetGameConfig(config)
        .apply(&mut tournament)
        .unwrap();
    let ending_elo = tournament.get_player_or_default_stats(id).elo();
    assert_relative_eq!(starting_elo + 1500.0, ending_elo);
}

#[test]
fn set_ranking_config() {
    let mut tournament = Tournament::new();
    let start_ranking = tournament.ranking_config().clone();
    let mut new_ranking = start_ranking.clone();
    new_ranking.lost_with += 1;
    Action::SetRankingConfig(new_ranking)
        .apply(&mut tournament)
        .unwrap();
    assert_eq!(
        start_ranking.lost_with + 1,
        tournament.ranking_config().lost_with
    );
}
