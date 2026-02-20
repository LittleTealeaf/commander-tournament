pub mod home;

use iced::{
    Alignment, Element, Length,
    alignment::Horizontal,
    widget::{button, column, container, row, space, text},
};

use crate::{
    App,
    logic::{Message, file::FileMessage},
};

impl App {
    #[must_use]
    pub fn view(&self) -> Element<'_, Message> {
        if let Some(error) = &self.error {
            return error_screen(error);
        }

        column![self.toolbar(),].into()
    }

    fn toolbar(&self) -> Element<'_, Message> {
        row![
            button("New Player"),
            space().width(15.0),
            button("Open").on_press(FileMessage::OpenFile.into()),
            button("Save").on_press(FileMessage::Save.into()),
            button("Save As")
                .on_press_maybe(self.file.is_some().then_some(FileMessage::SaveAs.into())),
            button("New").on_press(FileMessage::New.into())
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
