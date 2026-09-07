mod update {
    use app::views::game_config::{GameConfigMsg, GameConfigOut, GameConfigView};
    use edh_tourn::{config::game::GameConfig, tournament::Tournament};
    use iced_tea::{Model, Signal};

    #[test]
    fn test_close() {
        let t = Tournament::new();
        let mut state = GameConfigView::new(GameConfig::default());
        let effect = state.update(GameConfigMsg::Close, &t).unwrap();
        let Signal::Out(msg) = effect else {
            panic!("Expected message to return an out");
        };
        assert!(matches!(msg, GameConfigOut::Close));
    }

    #[test]
    fn test_save() {
        let t = Tournament::new();
        let mut state = GameConfigView::new(GameConfig::default());
        let effect = state.update(GameConfigMsg::Save, &t).unwrap();
        let Signal::Out(msg) = effect else {
            panic!("Expected message to return an out");
        };
        assert!(matches!(msg, GameConfigOut::SaveAndClose(_)));
    }
}
