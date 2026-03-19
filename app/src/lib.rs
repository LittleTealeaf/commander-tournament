pub mod components;
pub mod message;
pub mod services;
pub mod traits;
pub mod views;

use std::path::PathBuf;

use edh_tourn::tournament::Tournament;

use crate::{
    message::Message,
    traits::Component,
    views::{View, home},
};

#[derive(Debug)]
pub struct App {
    tournament: Tournament,
    home: home::State,
    error: Option<String>,
    file: Option<PathBuf>,
    views: Vec<View>,
}

impl App {
    #[must_use]
    pub const fn tournament(&self) -> &Tournament {
        &self.tournament
    }

    #[must_use]
    pub const fn tournament_mut(&mut self) -> &mut Tournament {
        &mut self.tournament
    }
}

impl Component for App {
    type Message = Message;
    type Context<'a> = ();
    type OutMessage = ();
}
