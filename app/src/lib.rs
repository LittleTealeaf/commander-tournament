pub mod components;
pub mod core;
pub mod error;
pub mod home;
pub mod player_details;
pub mod services;
pub mod style;
pub mod traits;

use std::path::PathBuf;

use edh_tourn::tournament::Tournament;
use iced::Task;

use crate::{
    core::{message::Message, settings::AppSettings, view::View},
    traits::Component,
};

#[derive(Debug, Default)]
pub struct App {
    tournament: Tournament,
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
}

impl Component for App {
    type Message = Message;
    type OutMessage = ();
}
