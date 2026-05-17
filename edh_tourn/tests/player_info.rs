use edh_tourn::player::info::PlayerInfo;

const PLAYER_INFO: PlayerInfo = PlayerInfo::new(String::new());
const TEST_MOXFIELD_ID: &str = "BtCcQ8eWg0uT8n4fFPK3Xg";

mod name {
    use super::*;
    #[test]
    fn new_name() {
        let name = "hello".to_owned();
        let info = PlayerInfo::new(name);
        assert_eq!("hello", info.name());
    }

    #[test]
    fn set_name() {
        let mut info = PLAYER_INFO;
        assert_ne!(info.name(), "hello");
        info.set_name("hello".to_owned());
        assert_eq!(info.name(), "hello");
    }
}

mod display_name {
    use super::*;

    #[test]
    fn defaults_to_name() {
        let name = "hello".to_owned();
        let mut info = PlayerInfo::new(name);
        info.set_precon(false);
        assert_eq!("hello", info.display_name());
    }

    #[test]
    fn precon_has_suffix() {
        let name = "hello".to_owned();
        let mut info = PlayerInfo::new(name);
        info.set_precon(true);
        assert_eq!("hello (Precon)", info.display_name());
    }
}

mod description {
    use super::*;

    #[test]
    fn default_is_empty() {
        assert!(PLAYER_INFO.description().is_empty());
    }

    #[test]
    fn set_description() {
        let desc = "DESCRIPTION";
        let mut info = PLAYER_INFO;
        info.set_description(desc.to_owned());
        assert_eq!(info.description(), desc);
    }

    #[test]
    fn with_description() {
        let desc = "DESCRIPTION";
        let info = PLAYER_INFO.with_description(desc.to_owned());
        assert_eq!(info.description(), desc);
    }
}

mod moxfield_id {
    use super::*;

    #[test]
    fn none_by_default() {
        assert!(PLAYER_INFO.moxfield_id().is_none());
    }

    #[test]
    fn link_returns_none() {
        assert!(PLAYER_INFO.moxfield_link().is_none());
    }

    #[test]
    fn goldfish_returns_none() {
        assert!(PLAYER_INFO.moxfield_goldfish_link().is_none());
    }

    #[test]
    fn accepts_ids() {
        let mut info = PLAYER_INFO;
        info.set_moxfield_id(TEST_MOXFIELD_ID.to_owned());
        assert!(info.moxfield_id().is_some());
    }

    #[test]
    fn accepts_deck_links() {
        let mut info = PLAYER_INFO;
        info.set_moxfield_id(format!("https://moxfield.com/decks/{TEST_MOXFIELD_ID}"));
        assert!(info.moxfield_id().is_some());
    }

    #[test]
    fn accepts_goldfish_links() {
        let mut info = PLAYER_INFO;
        info.set_moxfield_id(format!("https://moxfield.com/decks/{TEST_MOXFIELD_ID}/goldfish"));
        assert!(info.moxfield_id().is_some());
    }

    #[test]
    fn returns_link() {
        let mut info = PLAYER_INFO;
        info.set_moxfield_id(format!("https://moxfield.com/decks/{TEST_MOXFIELD_ID}"));
        assert!(info.moxfield_link().is_some());
    }

    #[test]
    fn returns_goldfish_link() {
        let mut info = PLAYER_INFO;
        info.set_moxfield_id(format!("https://moxfield.com/decks/{TEST_MOXFIELD_ID}"));
        assert!(info.moxfield_goldfish_link().is_some());
    }

    #[test]
    fn clear_id() {
        let mut info = PLAYER_INFO.with_moxfield_id(TEST_MOXFIELD_ID.to_owned());
        info.clear_moxfield_id();
        assert!(info.moxfield_id().is_none());
    }
}

mod color {
    use super::*;
    use edh_tourn::player::color::{ColorIdentity, MtgColor};

    #[test]
    fn default_colorless() {
        assert_eq!(PLAYER_INFO.color_identity(), ColorIdentity::COLORLESS);
    }

    #[test]
    fn set_color_identity() {
        let mut info = PLAYER_INFO;
        for identity in ColorIdentity::IDENTITIES {
            info.set_color_identity(identity);
            assert_eq!(info.color_identity(), identity);
        }
    }

    #[test]
    fn with_color_identity() {
        for identity in ColorIdentity::IDENTITIES {
            assert_eq!(
                PLAYER_INFO.with_color_identity(identity).color_identity(),
                identity
            );
        }
    }

    #[test]
    fn add_color() {
        for identity in ColorIdentity::IDENTITIES {
            for color in MtgColor::COLORS {
                let expected = identity + color;
                let mut info = PLAYER_INFO;
                info.set_color_identity(identity);
                info.add_color(color);
                assert_eq!(info.color_identity(), expected);
            }
        }
    }

    #[test]
    fn remove_color() {
        for identity in ColorIdentity::IDENTITIES {
            for color in MtgColor::COLORS {
                let expected = identity - color;
                let mut info = PLAYER_INFO;
                info.set_color_identity(identity);
                info.remove_color(color);
                assert_eq!(info.color_identity(), expected);
            }
        }
    }

    #[test]
    fn toggle_color() {
        for identity in ColorIdentity::IDENTITIES {
            let mut info = PLAYER_INFO.with_color_identity(identity);
            for color in MtgColor::COLORS {
                let has_color = identity.has_color(color);
                let expected = if has_color {
                    identity - color
                } else {
                    identity + color
                };
                info.toggle_color(color);
                assert_eq!(info.color_identity(), expected);
                info.toggle_color(color);
                assert_eq!(info.color_identity(), identity);
            }
        }
    }
}

mod precon {
    use super::*;

    #[test]
    fn defaults_no_precon() {
        assert!(!PLAYER_INFO.is_precon());
    }

    #[test]
    fn sets_precon() {
        let mut info = PLAYER_INFO;
        assert!(!info.is_precon());
        info.set_precon(true);
        assert!(info.is_precon());
    }
}
