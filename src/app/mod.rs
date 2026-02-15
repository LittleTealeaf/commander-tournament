#![allow(dead_code)]
mod message;
mod traits;
mod view;

use std::{cell::RefCell, path::PathBuf, rc::Rc};

use appconfig::AppConfigManager;
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
            Ok(task) => task,
            Err(res) => {
                let msg = res.to_string();
                app.error = Some(msg);
                Task::none()
            }
        }
    }
    iced::run(updater, App::view)
}

struct App {
    tournament: Tournament,
    error: Option<String>,
    screen: Option<Screen>,
    home: AppHome,
    file: Option<PathBuf>,
    config: AppConfigManager<AppConfig>,
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug, Default)]
struct AppConfig {
    last_opened_file: Option<PathBuf>,
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
            file: None,
            config: AppConfigManager::new(
                Rc::from(RefCell::from(AppConfig::default())),
                "TournamentManager",
                "Tealeaf",
            )
            .with_auto_saving(true),
        }
    }
}
