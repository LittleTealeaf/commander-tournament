use std::path::PathBuf;

use edh_tourn::tournament::Tournament;
use iced::Task;

pub mod config;
pub mod fonts;
pub mod logic;
#[cfg(feature = "dev")]
pub mod tests;
pub mod traits;
pub mod view;

use crate::{
    config::AppConfig,
    logic::{Message, file::FileMessage},
    traits::HandleMessage,
    view::{Scene, home::HomeState},
};

#[derive(Default, Debug)]
pub struct App {
    tournament: Tournament,
    error: Option<String>,
    file: Option<PathBuf>,
    home: HomeState,
    scenes: Vec<Scene>,
    config: Option<AppConfig>,
}

impl App {
    pub fn boot() -> (Self, Task<Message>) {
        let mut app = Self {
            config: AppConfig::load().ok(),
            ..Self::default()
        };

        let task = app
            .config
            .as_ref()
            .and_then(|config| config.last_opened())
            .cloned()
            .map_or_else(Task::none, |path| {
                app.updater(FileMessage::LoadFromFile(path).into())
            });

        (app, task)
    }

    pub fn updater(&mut self, message: Message) -> Task<Message> {
        match self.update(message) {
            Ok(task) => task,
            Err(res) => {
                let msg = res.to_string();
                self.error = Some(msg);
                Task::none()
            }
        }
    }

    #[must_use]
    pub const fn tournament(&self) -> &Tournament {
        &self.tournament
    }

    pub const fn tournament_mut(&mut self) -> &mut Tournament {
        &mut self.tournament
    }
}
