pub mod config;
pub mod games;
pub mod home;
pub mod player;

use crate::app::{
    App,
    view::{home::view_home, player::EditPlayer},
};
use iced::{
    Alignment, Element,
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

        container(column![content]).into()
    }
}

fn render_error(error: &str) -> Element<'_, Message> {
    container(row![
        text(format!("Error: {error}")),
        button("Close").on_press(Message::ClearError)
    ])
    .align_x(Alignment::Center)
    .align_y(Alignment::Center)
    .into()
}
