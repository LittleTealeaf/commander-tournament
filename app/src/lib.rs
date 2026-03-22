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
    modified: bool,
    home: home::Home,
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

    pub fn push_view<V>(&mut self, view: V)
    where
        V: Into<View>,
    {
        self.views.push(view.into());
    }

    pub fn pop_view(&mut self) {
        let _ = self.views.pop();
    }

    pub fn clear_views(&mut self) {
        self.views.clear();
    }
}

impl Component for App {
    type Message = Message;
    type OutMessage = ();
}
