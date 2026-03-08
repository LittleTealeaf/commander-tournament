pub mod config_matchmaker;
pub mod confirm;
pub mod home;
pub mod player;

use iced::{
    Alignment, Element, Length,
    alignment::Horizontal,
    widget::{button, column, container, row, rule, space, text},
};
use itertools::Itertools;

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

impl Scene {
    pub fn title(&self) -> String {
        match self {
            Scene::Player(view_player_scene) => view_player_scene.title(),
            Scene::Confirm(confirm_prompt) => confirm_prompt.title(),
            Scene::MatchmakerConfig(_) => "Matchmaker Configuration".to_owned(),
        }
    }
}

impl View<Scene> for App {
    fn view<'a>(&'a self, scene: &'a Scene) -> Element<'a, Message> {
        match scene {
            Scene::Player(view_player_scene) => self.view(view_player_scene),
            Scene::Confirm(confirm_prompt) => self.view(confirm_prompt),
            Scene::MatchmakerConfig(view_matchmaker_config) => self.view(view_matchmaker_config),
        }
    }
}

impl App {
    #[must_use]
    pub fn app_view(&self) -> Element<'_, Message> {
        if let Some(error) = &self.error {
            return error_screen(error);
        }
        let mut iter = self.scenes.iter().rev();

        let content = iter.next().map_or_else(
            || container(self.view(&self.home)),
            |scene| {
                let nav = iter.rev().map(Scene::title).join("  ");
                container(column![
                    (!nav.is_empty()).then_some(column![
                        row![
                            text(format!("{nav}  {}", scene.title())),
                            space().width(Length::Fill),
                            button("Close All")
                                .style(button::text)
                                .on_press(Message::CloseAllScenes),
                        ],
                        rule::horizontal(2)
                    ]),
                    self.view(scene)
                ])
            },
        );
        container(content).padding(10).into()
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
