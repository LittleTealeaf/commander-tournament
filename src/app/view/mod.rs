pub mod config;
pub mod games;
pub mod home;
pub mod player;

use crate::app::{
    App,
    view::{home::view_home, player::EditPlayer},
};
use iced::{
    Alignment, Element, Length,
    alignment::Horizontal,
    widget::{button, column, container, row, text},
};

use crate::app::{message::Message, traits::View};

pub enum Screen {
    Player(EditPlayer),
}

impl View for App {
    fn view(app: &App) -> Element<'_, Message> {
        if let Some(err) = &app.error {
            return render_error(err);
        }
        let content = match &app.screen {
            Some(screen) => match screen {
                Screen::Player(p) => p.view(),
            },
            None => view_home(app),
        };

        container(column![render_toolbar(app), content]).into()
    }
}

fn render_toolbar(_: &App) -> Element<'_, Message> {
    row![
        button("New Player").on_press(Message::EditPlayer(None)),
        button("Open").on_press(Message::Open),
        button("SaveAs").on_press(Message::SaveAs),
    ]
    .into()
}

fn render_error(error: &str) -> Element<'_, Message> {
    container(
        column![
            text(format!("Error: {error}")),
            button("Close").on_press(Message::ClearError)
        ]
        .align_x(Horizontal::Center),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .align_x(Alignment::Center)
    .align_y(Alignment::Center)
    .into()
}
