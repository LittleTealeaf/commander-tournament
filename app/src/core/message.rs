use iced::Task;

use crate::{
    App,
    core::{
        file::FileAction,
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
    TournFile(FileAction),
    Error(String),
    ViewHome(home::Message),
    ViewError(error::Message),
    ViewPlayer(player_details::Message),
}

impl App {
    pub fn handle_update(&mut self, message: Message) -> Task<Message> {
        let mut messages_to_process = vec![message];
        let mut tasks = vec![];

        while let Some(msg) = messages_to_process.pop() {
            match self.update(msg, ()) {
                Ok(effect) => {
                    let mut effects_to_process = vec![effect];
                    while let Some(eff) = effects_to_process.pop() {
                        match eff {
                            Effect::Task(task) => tasks.push(task),
                            Effect::Global(m) => messages_to_process.push(m),
                            Effect::Batch(batch) => {
                                effects_to_process.extend(batch.into_iter().rev());
                            }
                            Effect::Done | Effect::Out(()) => (),
                        }
                    }
                }
                Err(error) => {
                    self.views
                        .push(View::Error(error::State::new(error.to_string())));
                }
            }
        }

        Task::batch(tasks)
    }
}
