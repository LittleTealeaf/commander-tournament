mod update {
    use app::{
        effect::Effect,
        traits::ComponentUpdate,
        views::game_config::{GameConfigMsg, GameConfigOut, GameConfigView},
    };
    use edh_tourn::{config::game::GameConfig, tournament::Tournament};

    #[test]
    fn test_close() {
        let t = Tournament::new();
        let mut state = GameConfigView::new(GameConfig::new());
        let effect = state.update(GameConfigMsg::Close, &t).unwrap();
        let Effect::Out(msg) = effect else {
            panic!("Expected message to return an out");
        };
        assert!(matches!(msg, GameConfigOut::Close));
    }
}
