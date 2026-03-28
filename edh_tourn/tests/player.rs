//! Tests for Player, Player Registration, Player Info
use edh_tourn::{player::info::PlayerInfo, tournament::Tournament};

#[test]
fn get_player_name() {
    let mut t = Tournament::new();
    let id = t.register_player("test".to_owned()).unwrap();
    assert_eq!(
        "test",
        t.get_player_name(&id).expect("Expected Player to be found")
    );
}

#[test]
fn get_invalid_player_name() {
    let mut t = Tournament::new();
    let id = t.register_debug_player().unwrap();
    t.unregister_player(id).unwrap();
    assert!(t.get_player_name(&id).is_none());
}

#[test]
fn get_player_info() {
    let mut t = Tournament::new();
    let id = t.register_player("test".to_owned()).unwrap();
    let info = t
        .get_player_info(&id)
        .expect("Expected Info to be Returned");
    assert_eq!("test", info.name(), "Expected correct info to be returned");
}

#[test]
fn get_invalid_player_info() {
    let mut t = Tournament::new();
    let id = t.register_debug_player().unwrap();
    t.unregister_player(id).unwrap();
    assert!(t.get_player_info(&id).is_none());
}

#[test]
fn register_name_already_exists() {
    let mut t = Tournament::new();
    let _ = t.register_player("name".to_owned());
    t.register_player("name".to_owned()).unwrap_err();
}

#[test]
fn register_with_info_name_already_exists() {
    let mut t = Tournament::new();
    let _ = t.register_player("name".to_owned());
    t.register_player_with_info(PlayerInfo::new("name".to_owned()))
        .unwrap_err();
}

#[test]
fn set_player_info_duplicate_name() {
    let mut t = Tournament::new();
    let id_1 = t.register_debug_player().unwrap();
    let id_2 = t.register_debug_player().unwrap();
    let name = t.get_player_name(&id_1).unwrap().to_owned();
    let mut info = t.get_player_info(&id_2).unwrap().clone();
    info.set_name(name);
    t.set_player_info(id_2, info).unwrap_err();
}

#[test]
fn set_player_info_updates_get_id() {
    let mut t = Tournament::new();
    let id = t.register_player("tester".to_owned()).unwrap();
    let mut info = t.get_player_info(&id).unwrap().clone();
    info.set_name("testing".to_owned());
    t.set_player_info(id, info).unwrap();
    let new_id = t.get_player_id(&"testing".to_owned()).unwrap();
    assert_eq!(id, new_id);
}

#[test]
fn register_player_updates_get_id() {
    let mut t = Tournament::new();
    t.register_player("test".to_owned()).unwrap();
    t.get_player_id(&"test".to_owned()).unwrap();
}

#[test]
fn unregister_player_removes_player() {
    let mut t = Tournament::generate_tournament(10, 20).unwrap();
    let (id, info) = t.players().iter().next().unwrap();
    let id = *id;
    let info = info.clone();
    t.unregister_player(id).unwrap();

    assert!(
        t.get_player_id(info.name()).is_none(),
        "Name still exists for unregistered player"
    );
    assert!(
        t.get_player_info(&id).is_none(),
        "Found info for the unregistered player"
    );

    for game in t.games() {
        assert!(
            !game.has_player(id),
            "Found game with the unregistered player"
        );
    }
}
