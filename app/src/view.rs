pub mod config_matchmaker;
pub mod confirm;
pub mod home;
pub mod player;

use iced::{
    Alignment, Element, Length,
    alignment::Horizontal,
    widget::{button, column, container, text},
};

use crate::{
    App,
    logic::Message,
    traits::View,
    view::{
        config_matchmaker::ViewMatchmakerConfig, confirm::ConfirmPrompt, player::ViewPlayerScene,
    },
};

#[derive(Debug)]
pub enum Scene {
    Player(ViewPlayerScene),
    Confirm(ConfirmPrompt),
    MatchmakerConfig(ViewMatchmakerConfig),
}

impl App {
    #[must_use]
    pub fn app_view(&self) -> Element<'_, Message> {
        if let Some(error) = &self.error {
            return error_screen(error);
        }
        let screen = self.scenes.last().map_or_else(
            || self.view(&self.home),
            |scene| match scene {
                Scene::Player(scene) => self.view(scene),
                Scene::Confirm(prompt) => self.view(prompt),
                Scene::MatchmakerConfig(scene) => self.view(scene),
            },
        );

        container(screen).into()
    }
}

fn error_screen(error: &str) -> Element<'_, Message> {
    container(
        column![
            text(format!("Error: {error}")),
            button("Close").on_press(Message::Error(None))
        ]
        .align_x(Horizontal::Center),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .align_x(Alignment::Center)
    .align_y(Alignment::Center)
    .into()
}
