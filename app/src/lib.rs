pub mod components;
pub mod message;
pub mod services;
pub mod settings;
pub mod style;
pub mod traits;
pub mod views;

use std::path::PathBuf;

use edh_tourn::tournament::Tournament;
use iced::Task;

use crate::{
    message::Message,
    settings::AppSettings,
    traits::Component,
    views::{View, home},
};

#[derive(Debug, Default)]
pub struct App {
    tournament: Tournament,
    modified: bool,
    home: home::State,
    error: Option<String>,
    file: Option<PathBuf>,
    views: Vec<View>,
    settings: Option<AppSettings>,
}

impl App {
    pub fn boot() -> (Self, Task<Message>) {
        (Self::default(), Task::done(Message::OnBoot))
    }

    #[must_use]
    pub const fn tournament(&self) -> &Tournament {
        &self.tournament
    }

    #[must_use]
    pub fn title(&self) -> String {
        if self.modified {
            "* Commander Tournament".to_owned()
        } else {
            "Commander Tournament".to_owned()
        }
    }
}

impl Component for App {
    type Message = Message;
    type OutMessage = ();
}
