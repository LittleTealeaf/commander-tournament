//! Tests for Player, Player Registration, Player Info
use edh_tourn::{
    player::{color::ColorIdentity, info::PlayerInfo},
    tournament::Tournament,
};

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
    let info = t.get_player_info(&id).expect("Expected Info to be Returned");
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
        assert!(!game.has_player(id), "Found game with the unregistered player");
    }
}

#[test]
fn require_id_registered() {
    let mut t = Tournament::new();
    let id = t.register_debug_player().unwrap();

    t.require_id_registered(id).unwrap();
    t.unregister_player(id).unwrap();
    t.require_id_registered(id).unwrap_err();
}

#[test]
fn get_or_register_player() {
    let mut t = Tournament::new();
    let id = t.register_debug_player().unwrap();
    let name = t.get_player_name(&id).unwrap().to_owned();

    let found_id = t.get_or_register_player(name.clone()).unwrap();
    assert_eq!(found_id, id, "Expected original player id to be returned");

    let cloned = t.clone();
    let new_name = format!("new_{name}");
    let new_id = t.get_or_register_player(new_name).unwrap();
    assert_ne!(new_id, id, "Expected different name to return a new id");
    assert!(
        !cloned.is_id_registered(&new_id),
        "Expected id to not exist before"
    );
}

#[test]
fn update_or_register_player_with_info() {
    let mut t = Tournament::new();
    t.register_debug_player().unwrap();
    let mut info = PlayerInfo::new("Testing".to_owned());
    info.set_color_identity(ColorIdentity::RED);
    let id = t.update_or_register_player_with_info(info.clone()).unwrap();
    let found_info = t.get_player_info(&id).unwrap();
    assert_eq!(
        found_info.color_identity(),
        ColorIdentity::RED,
        "Info was not properly specified"
    );

    info.set_color_identity(ColorIdentity::BLUE);
    let new_id = t.update_or_register_player_with_info(info.clone()).unwrap();
    assert_eq!(id, new_id, "Expected id to be found");

    let found_info = t.get_player_info(&id).unwrap();
    assert_eq!(
        found_info.color_identity(),
        ColorIdentity::BLUE,
        "Info was not properly specified"
    );
}

#[test]
fn register_player_with_info() {
    let mut t = Tournament::new();
    let info = PlayerInfo::new("Testing".to_owned());
    let id = t.register_player_with_info(info.clone()).unwrap();
    assert_eq!(t.get_player_info(&id).unwrap(), &info);

    t.register_player_with_info(info).unwrap_err();
    t.register_player_with_info(PlayerInfo::new(String::new()))
        .unwrap_err();
}

#[test]
fn get_registered_player() {
    let mut t = Tournament::new();
    let id = t.register_debug_player().unwrap();
    for _ in 0..4 {
        t.register_debug_player().unwrap();
    }
    for _ in 0..4 {
        let game = t.random_game().unwrap();
        t.record_entry(game).unwrap();
    }

    let player = t.get_registered_player(id).unwrap();
    assert_eq!(player.id(), id);

    let info = t.get_player_info(&id).unwrap();
    assert_eq!(player.info(), info);

    let stats = t.get_player_or_default_stats(id);
    assert_eq!(player.stats(), stats);

    t.unregister_player(id).unwrap();
    assert!(t.get_registered_player(id).is_none());
}

#[test]
fn get_player_display_name() {
    const NAME: &str = "Test";
    let mut t = Tournament::new();

    let id = t.register_player(NAME.to_owned()).unwrap();
    let mut info = t.get_player_info(&id).unwrap().clone();
    assert_eq!(t.get_player_display_name(&id).unwrap(), info.display_name());

    info.set_precon(true);
    t.set_player_info(id, info.clone()).unwrap();
    assert_eq!(t.get_player_display_name(&id).unwrap(), info.display_name());

    t.unregister_player(id).unwrap();
    assert!(t.get_player_display_name(&id).is_none());
}
