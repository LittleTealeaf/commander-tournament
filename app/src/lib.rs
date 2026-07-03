pub mod app;
pub mod icons;
pub mod components;
pub mod core;
pub mod effect;
pub mod fonts;
pub mod home;
pub mod popup;
pub mod services;
pub mod traits;
pub mod views;

use std::path::PathBuf;

use edh_tourn::tournament::Tournament;
use iced::{Subscription, Task, event};

use crate::{
    app::{Message, View},
    core::{file::FileAction, state::AppState},
    traits::Component,
};

#[derive(Debug, Default)]
pub struct App {
    tournament: Tournament,
    modified: bool,
    is_saving: bool,
    menu: app::Menu,
    home: home::Home,
    file: Option<PathBuf>,
    views: Vec<View>,
    state: Option<AppState>,
    close_requested: bool,
    // If Some, then show a confirmation dialog
    // that performs the given action on confirmation
    overwrite_requested: Option<FileAction>,
    error: Option<String>,
}

impl App {
    pub fn boot() -> (Self, Task<Message>) {
        (
            Self::default(),
            Task::future(async { Message::AppStateLoaded(AppState::load().await.ok()) }),
        )
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
        event::listen_with(|event, _status, _window| -> Option<Message> { Message::from_event(event) })
    }
}

impl Component for App {
    type Message = Message;
    type OutMessage = ();
}
