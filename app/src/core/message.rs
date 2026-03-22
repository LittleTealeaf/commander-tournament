use iced::Task;

use crate::{
    App,
    core::{
        file::TournamentFileMessage,
        settings::{self, AppSettings},
        tournament,
        view::View,
    },
    error, home, player_details,
    traits::{ComponentUpdate, Effect},
};

#[derive(Debug, Clone, derive_more::From)]
pub enum Message {
    OnBoot,
    Settings(settings::Message),
    SettingsLoaded(Option<AppSettings>),
    Tournament(tournament::Action),
    TournFile(TournamentFileMessage),
    Error(String),
    ViewHome(home::Message),
    ViewError(error::Message),
    ViewPlayer(player_details::Message),
}

impl App {
    pub fn handle_update(&mut self, message: Message) -> Task<Message> {
        match self.update(message, ()) {
            Ok(Effect::Task(task)) => task,
            Err(error) => {
                self.views
                    .push(View::Error(error::State::new(error.to_string())));
                Task::none()
            }
            _ => Task::none(),
        }
    }
}
