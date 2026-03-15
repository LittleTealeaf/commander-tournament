//! Tests for Player, Player Registration, Player Info

use edh_tourn::tournament::Tournament;
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
    let t = Tournament::new();
    assert!(t.get_player_name(&5).is_none());
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
    let t = Tournament::new();
    assert!(t.get_player_info(&5).is_none());
}
