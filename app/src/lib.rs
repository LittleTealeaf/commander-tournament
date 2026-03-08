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
            && let Some(ref path) = cfg.last_opened
            && let Ok(file) = File::open(path)
            && let Ok(tournament) = ron::de::from_reader(&file)
        {
            app.tournament = tournament;
            app.file = Some(path.clone());
            return app;
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

#[cfg(test)]
mod boot_tests {
    use super::*;
    use std::{env, io::Write};
    use tempfile::NamedTempFile;

    use serial_test::serial;

    #[serial]
    #[test]
    fn boot_prefers_config() {
        // point config directory at a temp location
        let tmp = tempfile::tempdir().unwrap();
        unsafe {
            env::set_var("XDG_CONFIG_HOME", tmp.path());
        }

        let tournament = Tournament::sample_game();
        let mut file = NamedTempFile::new().unwrap();
        let serialized = ron::to_string(&tournament).unwrap();
        // write via the handle so the file isn't removed until we drop `file`
        file.write_all(serialized.as_bytes()).unwrap();
        let pathbuf = file.path().to_path_buf();
        assert!(pathbuf.exists(), "temporary tournament file disappeared");

        // sanity-check we can read the tournament back ourselves
        {
            let mut f = std::fs::File::open(&pathbuf).unwrap();
            let loaded: Tournament = ron::de::from_reader(&mut f).unwrap();
            assert_eq!(loaded, tournament);
        }

        // write a config that refers to our temp file
        let cfg = config::SystemConfig {
            last_opened: Some(pathbuf.clone()),
        };
        cfg.save().unwrap();

        let app = App::boot();
        assert_eq!(app.file.as_ref(), Some(&pathbuf));
        assert_eq!(app.tournament, tournament);
    }
}
