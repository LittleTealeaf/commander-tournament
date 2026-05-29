//! Tests for Player, Player Registration, Player Info
use edh_tourn::{
    player::{color::ColorIdentity, info::PlayerInfo},
    tournament::Tournament,
};

mod get_player_name {
    use super::*;

    #[test]
    fn gets_name() {
        let mut t = Tournament::new();
        let id = t.register_player("test".to_owned()).unwrap();
        assert_eq!(
            "test",
            t.get_player_name(&id).expect("Expected Player to be found")
        );
    }

    #[test]
    fn returns_none_when_invalid() {
        let mut t = Tournament::new();
        let id = t.register_debug_player().unwrap();
        t.unregister_player(id).unwrap();
        assert!(t.get_player_name(&id).is_none());
    }
}

mod get_player_info {
    use super::*;

    #[test]
    fn gets_info() {
        let mut t = Tournament::new();
        let id = t.register_player("test".to_owned()).unwrap();
        let info = t.players().get(&id).unwrap();
        assert_eq!("test", info.name(), "Expected correct info to be returned");
    }

    #[test]
    fn returns_none_when_invalid() {
        let mut t = Tournament::new();
        let id = t.register_debug_player().unwrap();
        t.unregister_player(id).unwrap();
        assert!(!t.players().contains_key(&id));
    }
}

mod register_player {
    use super::*;

    #[test]
    fn with_name() {
        let mut t = Tournament::new();
        let id = t.register_player("name".to_owned()).unwrap();
        assert!(
            t.players().keys().any(|i| i == &id),
            "Expected id to be valid player"
        );
    }

    #[test]
    fn with_info() {
        let mut t = Tournament::new();
        let id = t.register_player(PlayerInfo::new("name".to_owned())).unwrap();
        assert!(
            t.players().keys().any(|i| i == &id),
            "Expected id to be valid player"
        );
    }

    #[test]
    fn name_already_exists() {
        let mut t = Tournament::new();
        let _ = t.register_player("name".to_owned());
        t.register_player("name".to_owned()).unwrap_err();
    }

    #[test]
    fn empty_name() {
        let mut t = Tournament::new();
        t.register_player(String::new()).unwrap_err();
    }

    #[test]
    fn updates_get_id() {
        let mut t = Tournament::new();
        let id = t.register_player("test".to_owned()).unwrap();
        assert_eq!(id, t.get_player_id(&"test".to_owned()).unwrap());
    }
}

mod set_player_info {
    use super::*;

    #[test]
    fn sets_name() {
        let mut t = Tournament::new();
        let id = t.register_player("a".to_owned()).unwrap();
        let info = PlayerInfo::new("b".to_owned());
        t.set_player_info(id, info).unwrap();
        let found_info = t.players().get(&id).unwrap();
        assert_eq!(found_info.name(), "b");
    }

    #[test]
    fn duplicate_name() {
        let mut t = Tournament::new();
        let _ = t.register_player("a".to_owned()).unwrap();
        let b = t.register_player("b".to_owned()).unwrap();
        let info = PlayerInfo::new("a".to_owned());
        t.set_player_info(b, info).unwrap_err();
    }

    #[test]
    fn invalid_name() {
        let mut t = Tournament::new();
        let id = t.register_debug_player().unwrap();
        let info = PlayerInfo::new(String::new());
        t.set_player_info(id, info).unwrap_err();
    }

    #[test]
    fn updates_get_id() {
        let mut t = Tournament::new();
        let id = t.register_player("abc".to_owned()).unwrap();
        let info = PlayerInfo::new("def".to_owned());
        t.set_player_info(id, info).unwrap();
        assert!(t.get_player_id(&"abc".to_owned()).is_none());
        assert_eq!(id, t.get_player_id(&"def".to_owned()).unwrap());
    }
}

mod update_player_info {
    use super::*;

    #[test]
    fn change_color() {
        let mut t = Tournament::new();
        let id = t.register_debug_player().unwrap();
        t.update_player_info(&id, |info| info.with_color_identity(ColorIdentity::BLACK))
            .unwrap();
        let color = t.get_player_info(&id).unwrap().color_identity();
        assert_eq!(color, ColorIdentity::BLACK);
    }

    #[test]
    fn invalid_id() {
        let mut t = Tournament::new();
        let id = t.register_debug_player().unwrap();
        t.unregister_player(id).unwrap();
        t.update_player_info(&id, |info| info.with_color_identity(ColorIdentity::BLACK))
            .unwrap_err();
    }

    #[test]
    fn invalid_name() {
        let mut t = Tournament::new();
        let id = t.register_debug_player().unwrap();
        t.update_player_info(&id, |info| info.with_name(String::new()))
            .unwrap_err();
    }
}

mod unregister_player {
    use super::*;

    #[test]
    fn removes_from_players() {
        let mut t = Tournament::new();
        let id = t.register_debug_player().unwrap();
        t.unregister_player(id).unwrap();
        assert!(!t.players().keys().any(|i| i == &id));
    }

    #[test]
    fn removes_registered_player() {
        let mut t = Tournament::new();
        let id = t.register_debug_player().unwrap();
        t.unregister_player(id).unwrap();
        assert!(t.get_registered_player(id).is_none());
    }

    #[test]
    fn removes_player_info() {
        let mut t = Tournament::new();
        let id = t.register_debug_player().unwrap();
        t.unregister_player(id).unwrap();
        assert!(t.get_player_info(&id).is_none());
    }

    #[test]
    fn removes_from_id_lookups() {
        let mut t = Tournament::new();
        let id = t.register_player("Hello".to_owned()).unwrap();
        t.unregister_player(id).unwrap();
        assert!(t.get_player_id(&"Hello".to_owned()).is_none());
    }

    #[test]
    fn removes_participated_games() {
        const PLAYER_COUNT: usize = 4;
        const GAME_COUNT: usize = 5;
        let mut t = Tournament::generate_tournament(PLAYER_COUNT, GAME_COUNT).unwrap();

        let id = t.register_debug_player().unwrap();
        let matchup = t.matchmaker().create_match(id).unwrap();
        let record = matchup.debug_record().unwrap();
        t.record_game(record).unwrap();

        assert_eq!(t.games().len(), GAME_COUNT + 1);

        t.unregister_player(id).unwrap();

        assert_eq!(t.games().len(), GAME_COUNT);

        for game in t.games() {
            assert!(
                !game.has_player(id),
                "Found game containing the unregistered player"
            );
        }
    }

    #[test]
    fn invalid_id() {
        let mut t = Tournament::new();
        let id = t.register_debug_player().unwrap();
        t.unregister_player(id).unwrap();
        t.unregister_player(id).unwrap_err();
    }
}

mod require_id {
    use super::*;

    #[test]
    fn registered_id() {
        let mut t = Tournament::new();
        let id = t.register_debug_player().unwrap();
        t.require_id_registered(id).unwrap();
    }

    #[test]
    fn unregistered_id() {
        let mut t = Tournament::new();
        let id = t.register_debug_player().unwrap();
        t.unregister_player(id).unwrap();
        t.require_id_registered(id).unwrap_err();
    }
}

mod get_or_register {
    use super::*;

    #[test]
    fn gets_player() {
        let mut t = Tournament::new();
        let id = t.register_player("test").unwrap();
        let found = t.get_or_register_player("test".to_owned()).unwrap();
        assert_eq!(id, found);
    }

    #[test]
    fn registers_player() {
        let mut t = Tournament::new();
        let id = t.get_or_register_player("test".to_owned()).unwrap();
        let name = t.get_player_name(&id).unwrap();
        assert_eq!(name, "test");
    }
}

mod update_or_register {
    use super::*;

    #[test]
    fn new_player() {
        let mut t = Tournament::new();
        t.register_debug_player().unwrap();

        let mut info = PlayerInfo::new("Testing".to_owned());
        info.set_color_identity(ColorIdentity::RED);

        let id = t.update_or_register_player(info).unwrap();
        let found_info = t.get_player_info(&id).unwrap();

        assert_eq!(
            found_info.color_identity(),
            ColorIdentity::RED,
            "Info was not properly specified"
        );
    }

    #[test]
    fn update_existing() {
        let mut t = Tournament::new();
        t.register_debug_player().unwrap();

        let mut info = PlayerInfo::new("Testing".to_owned());
        info.set_color_identity(ColorIdentity::RED);
        let id = t.update_or_register_player(info.clone()).unwrap();

        info.set_color_identity(ColorIdentity::BLUE);
        let new_id = t.update_or_register_player(info).unwrap();

        assert_eq!(id, new_id, "Expected id to be found");

        let found_info = t.get_player_info(&id).unwrap();
        assert_eq!(
            found_info.color_identity(),
            ColorIdentity::BLUE,
            "Info was not properly specified"
        );
    }
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
