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
    logic::{Message, file::load_tournament_sync},
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
    #[must_use]
    pub fn boot() -> Self {
        let config = AppConfig::load().ok();
        let last_opened = config.as_ref().and_then(|config| config.last_opened());
        let tourn = last_opened.and_then(|path| load_tournament_sync(path).ok());
        let last_opened = tourn.is_some().then_some(last_opened).flatten().cloned();

        Self {
            config,
            file: last_opened,
            tournament: tourn.unwrap_or_default(),
            ..Self::default()
        }
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
