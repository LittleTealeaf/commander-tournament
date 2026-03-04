use std::path::PathBuf;

use edh_tourn::Tournament;
use iced::Task;

pub mod logic;
pub mod traits;
pub mod view;
#[cfg(feature = "dev")]
pub mod tests;

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
