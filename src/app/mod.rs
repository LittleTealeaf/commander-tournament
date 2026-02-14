mod components;
mod traits;

use commander_tournament::Tournament;
use iced::{Element, Task};
use opener::open_browser;

use crate::app::{
    components::leaderboard::LeaderboardComponent,
    traits::{HandleMessage, View},
};

pub fn launch() -> iced::Result {
    fn updater(app: &mut App, message: Message) -> Task<Message> {
        match app.update(message) {
            Ok(Some(task)) => task,
            Err(res) => {
                let msg = res.to_string();
                app.error = Some(msg);
                Task::none()
            }
            Ok(None) => Task::none(),
        }
    }
    iced::run(updater, App::view)
}

#[derive(Default)]
struct App {
    tournament: Tournament,
    error: Option<String>,
}

#[derive(Clone)]
pub enum Message {
    ClearError,
    OpenLink(String),
}

impl HandleMessage<Message> for App {
    fn update(&mut self, msg: Message) -> anyhow::Result<Option<Task<Message>>> {
        match msg {
            Message::ClearError => {
                self.error = None;
                Ok(None)
            }
            Message::OpenLink(link) => {
                open_browser(link)?;
                Ok(None)
            }
        }
    }
}

impl View for App {
    fn view(app: &App) -> Element<'_, Message> {
        LeaderboardComponent::view(app)
    }
}
