use edh_tourn::tournament::Tournament;

use crate::traits::{Component, ComponentView};

#[derive(Debug, Clone)]
pub struct GameStats;

#[derive(Debug, Clone)]
pub enum GameStatsMsg {}

#[derive(Debug)]
pub enum GameStatsOut {
    Close,
}

impl Component for GameStats {
    type Message = GameStatsMsg;
    type OutMessage = GameStatsOut;
}

impl ComponentView for GameStats {
    type ViewContext<'a>
        = &'a Tournament
    where
        Self: 'a;

    fn view<'a>(&'a self, context: Self::ViewContext<'a>) -> iced::Element<'a, Self::Message> {
        let _ = context;

        todo!()
    }
}
