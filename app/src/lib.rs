pub mod app;
pub mod widgets;
pub mod components;
pub mod core;
pub mod effect;
pub mod error;
pub mod home;
pub mod play;
pub mod player_details;
pub mod services;
pub mod style;
pub mod traits;

use std::path::PathBuf;

use edh_tourn::tournament::Tournament;
use iced::Task;

use crate::{
    app::{Message, View},
    core::state::AppState,
    traits::Component,
};

#[derive(Debug, Default)]
pub struct App {
    tournament: Tournament,
    modified: bool,
    is_saving: bool,
    home: home::Home,
    error: Option<String>,
    file: Option<PathBuf>,
    views: Vec<View>,
    state: Option<AppState>,
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
        const APP_TITLE: &str = "Commander Tournament";
        if self.modified {
            format!("* {APP_TITLE}")
        } else {
            APP_TITLE.to_owned()
        }
    }
}

impl Component for App {
    type Message = Message;
    type OutMessage = ();
}
