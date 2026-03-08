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

pub mod config; // configuration handling for the application

#[derive(Default, Debug)]
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
        // attempt to load the last-opened file from the system config first
        let mut app = Self::default();

        if let Ok(cfg) = config::SystemConfig::load()
            && let Some(path) = cfg.last_opened
        {
            match File::open(&path) {
                Ok(file) => match ron::de::from_reader(file) {
                    Ok(tournament) => {
                        app.tournament = tournament;
                        app.file = Some(path);
                        return app;
                    }
                    Err(e) => {
                        eprintln!(
                            "Failed to deserialize last opened tournament from {}: {e}",
                            path.display()
                        );
                    }
                },
                Err(e) => {
                    eprintln!(
                        "Failed to open last opened tournament from {}: {e}",
                        path.display()
                    );
                }
            }
        }

        // fall back to implicit "game.ron" in the current directory for backwards compatibility
        if Path::new("game.ron").exists()
            && let Ok(file) = File::open("game.ron")
            && let Ok(tournament) = ron::de::from_reader(file)
        {
            app.tournament = tournament;
            app.file = Some(Path::new("game.ron").to_path_buf());
        }

        app
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
