pub mod app;
pub mod core;
pub mod effect;
pub mod error;
pub mod home;
pub mod modals;
pub mod play;
pub mod player_details;
pub mod services;
pub mod style;
pub mod traits;
pub mod widgets;

use std::path::PathBuf;

use edh_tourn::tournament::Tournament;
use iced::{Event, Subscription, Task, event, window};

use crate::{
    app::{Message, View},
    core::state::AppState,
    modals::Modal,
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
    modals: Vec<Modal<Message>>,
    state: Option<AppState>,
    close_requested: bool,
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

    pub fn subscription(&self) -> Subscription<Message> {
        event::listen_with(|event, _status, _window| -> Option<Message> {
            (event == Event::Window(window::Event::CloseRequested))
                .then_some(Message::QuitRequested)
        })
    }
}

impl Component for App {
    type Message = Message;
    type OutMessage = ();
}
