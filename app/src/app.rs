use std::path::PathBuf;

use edh_tourn::tournament::Tournament;

use crate::views::home;

#[derive(Debug)]
pub struct App {
    tournament: Tournament,
    home: home::State,
    error: Option<String>,
    file: Option<PathBuf>,
}

impl App {
    #[must_use]
    pub const fn tournament(&self) -> &Tournament {
        &self.tournament
    }
}
