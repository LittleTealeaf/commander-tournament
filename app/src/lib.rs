pub mod components;
pub mod message;
pub mod services;
pub mod traits;
pub mod views;
pub mod style;

use std::path::PathBuf;

use edh_tourn::tournament::Tournament;
use iced::Task;

use crate::{
    message::Message,
    traits::Component,
    views::{View, home},
};

#[derive(Debug, Default)]
pub struct App {
    tournament: Tournament,
    home: home::State,
    error: Option<String>,
    file: Option<PathBuf>,
    views: Vec<View>,
}

impl App {
    pub fn boot() -> (Self, Task<Message>) {
        (Self::default(), Task::none())
    }

    #[must_use]
    pub const fn tournament(&self) -> &Tournament {
        &self.tournament
    }
}

impl Component for App {
    type Message = Message;
    type Context<'a> = ();
    type OutMessage = ();
}
