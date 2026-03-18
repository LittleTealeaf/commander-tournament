use crate::{
    App,
    logic::Message,
    traits::{HandleMessage, View},
    view::Scene,
};

#[derive(Debug, Clone, Default)]
pub struct TournamentStatsView;

#[derive(Debug, Clone)]
pub enum TournamentStatsMessage {
    Open,
    Close,
}

impl From<TournamentStatsMessage> for Message {
    fn from(value: TournamentStatsMessage) -> Self {
        Self::TournamentStats(value)
    }
}

impl HandleMessage<TournamentStatsMessage> for App {
    fn update(&mut self, msg: TournamentStatsMessage) -> anyhow::Result<iced::Task<Message>> {
        let Some(Scene::TournamentStats(_scene)) = self.scenes.pop() else {
            if matches!(msg, TournamentStatsMessage::Open) {
                self.scenes
                    .push(Scene::TournamentStats(TournamentStatsView));
            }
            return Message::done();
        };

        todo!()
    }
}

impl View<TournamentStatsView> for App {
    fn view<'a>(
        &'a self,
        _scene: &'a TournamentStatsView,
    ) -> iced::Element<'a, crate::logic::Message> {
        todo!()
    }
}
