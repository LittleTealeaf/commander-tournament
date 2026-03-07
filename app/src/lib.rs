use std::{
    fs::File,
    path::{Path, PathBuf},
};

use edh_tourn::Tournament;
use iced::Task;

pub mod fonts;
pub mod logic;
#[cfg(feature = "dev")]
pub mod tests;
pub mod traits;
pub mod view;

use crate::{
    logic::Message,
    traits::HandleMessage,
    view::{Scene, home::HomeState},
};

#[derive(Default)]
pub struct App {
    tournament: Tournament,
    error: Option<String>,
    file: Option<PathBuf>,
    home: HomeState,
    scenes: Vec<Scene>,
}

impl App {
    #[must_use]
    pub fn boot() -> Self {
        if Path::new("game.ron").exists()
            && let Ok(file) = File::open("game.ron")
            && let Ok(tournament) = ron::de::from_reader(file)
        {
            return Self {
                tournament,
                file: Some(Path::new("game.ron").to_path_buf()),

                ..Self::default()
            };
        }

        Self::default()
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
}
