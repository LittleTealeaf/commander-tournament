mod components;
mod traits;

use commander_tournament::Tournament;
use iced::Element;

use crate::app::traits::{HandleMessage, View};

pub fn launch() -> iced::Result {
    fn updater(app: &mut App, message: Message) {
        let result = app.update(message);
        if let Err(res) = result {
            let msg = res.to_string();
            app.error = Some(msg);
        }
    }
    iced::run(updater, view)
}

#[derive(Default)]
struct App {
    tournament: Tournament,
    error: Option<String>,
}

pub enum Message {
    ClearError,
}

impl HandleMessage<Message> for App {
    fn update(&mut self, msg: Message) -> anyhow::Result<()> {
        match msg {
            Message::ClearError => {
                self.error = None;
            }
        }
        Ok(())
    }
}

impl View for App {
    fn view(app: &App) -> Element<'_, Message> {
        todo!()
    }
}
