pub mod home;
pub mod view_player;

use iced::{
    Alignment, Element, Length,
    alignment::Horizontal,
    widget::{button, column, container, row, space, text},
};

use crate::{
    App,
    logic::{Message, file::FileMessage},
    traits::{View, ViewApp},
    view::{
        home::HomeState,
        view_player::{ViewPlayerMessage, ViewPlayerScene},
    },
};

pub enum Scene {
    Player(ViewPlayerScene),
}

impl App {
    #[must_use]
    pub fn view(&self) -> Element<'_, Message> {
        if let Some(error) = &self.error {
            return error_screen(error);
        }
        let screen = self.scenes.last().map_or_else(
            || HomeState::view(self),
            |scene| match scene {
                Scene::Player(scene) => scene.view(),
            },
        );

        column![self.toolbar(), screen,].into()
    }

    fn toolbar(&self) -> Element<'_, Message> {
        row![
            button("New Player").on_press(ViewPlayerMessage::Open(None).into()),
            space().width(15.0),
            button("Open").on_press(FileMessage::OpenFile.into()),
            button("Save").on_press(FileMessage::Save.into()),
            button("Save As")
                .on_press_maybe(self.file.is_some().then_some(FileMessage::SaveAs.into())),
            button("New").on_press(FileMessage::New.into()),
        ]
        .into()
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
