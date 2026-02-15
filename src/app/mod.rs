#![allow(dead_code)]
mod message;
mod traits;
mod view;

use commander_tournament::Tournament;
use iced::Task;

use crate::app::{
    message::Message,
    traits::{HandleMessage, View},
    view::{Screen, home::AppHome},
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

struct App {
    tournament: Tournament,
    error: Option<String>,
    screen: Option<Screen>,
    home: AppHome,
}

impl Default for App {
    fn default() -> Self {
        let mut tournament = Tournament::default();
        let _ = tournament.ingest_tsv_games(include_str!("../../data.tsv"));
        Self {
            error: None,
            tournament,
            screen: None,
            home: AppHome::default(),
        }
    }
}
