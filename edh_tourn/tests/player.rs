//! Tests for Player, Player Registration, Player Info

use edh_tourn::{
    Tournament,
    player::{
        color::{ColorIdentity, MtgColor},
        info::PlayerInfo,
    },
};

mod player_info {
    use super::*;

    const TEST_MOXFIELD_ID: &str = "BtCcQ8eWg0uT8n4fFPK3Xg";

    fn new_player_info() -> PlayerInfo {
        PlayerInfo::new("test".to_owned())
    }

    #[test]
    fn moxfield_id_none_by_default() {
        assert!(new_player_info().moxfield_id().is_none());
    }

    #[test]
    fn moxfield_link_returns_none() {
        assert!(new_player_info().moxfield_link().is_none());
    }

    #[test]
    fn moxfield_goldfish_returns_none() {
        assert!(new_player_info().moxfield_goldfish_link().is_none());
    }

    #[test]
    fn set_moxfield_accepts_ids() {
        let mut info = new_player_info();
        info.set_moxfield_id(TEST_MOXFIELD_ID.to_owned());
        assert!(info.moxfield_id().is_some());
    }

    #[test]
    fn set_moxfield_accepts_deck_links() {
        let mut info = new_player_info();
        info.set_moxfield_id(format!("https://moxfield.com/decks/{TEST_MOXFIELD_ID}"));
        assert!(info.moxfield_id().is_some());
    }

    #[test]
    fn set_moxfield_accepts_goldfish_links() {
        let mut info = new_player_info();
        info.set_moxfield_id(format!(
            "https://moxfield.com/decks/{TEST_MOXFIELD_ID}/goldfish"
        ));
        assert!(info.moxfield_id().is_some());
    }

    #[test]
    fn get_moxfield_link_returns_link() {
        let mut info = new_player_info();
        info.set_moxfield_id(format!("https://moxfield.com/decks/{TEST_MOXFIELD_ID}"));
        assert!(info.moxfield_link().is_some());
    }

    #[test]
    fn get_moxfield_goldfish_returns_link() {
        let mut info = new_player_info();
        info.set_moxfield_id(format!("https://moxfield.com/decks/{TEST_MOXFIELD_ID}"));
        assert!(info.moxfield_goldfish_link().is_some());
    }

    #[test]
    fn toggle_color() {
        let mut info = new_player_info();
        info.toggle_color(MtgColor::Blue);
        info.toggle_color(MtgColor::Blue);
        assert_eq!(ColorIdentity::COLORLESS, *info.color_identity());
    }
}

mod player_registration {

    use super::*;

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
}
