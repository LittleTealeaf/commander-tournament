use std::path::PathBuf;

use edh_tourn::tournament::Tournament;
use iced::Task;

use crate::{
    App,
    message::Message,
    services::system::{load_from_file_async, require_extension, serialize_by_extension},
    traits::{Effect, HandleMessage},
};

#[derive(Clone, Debug)]
pub enum TournFileMessage {
    Open,
    LoadTournament(PathBuf),
    TournamentLoaded(Box<Tournament>),
    SaveTournament(PathBuf),
    TournamentSaved,
    Save,
    SaveAs,
}

impl HandleMessage<TournFileMessage> for App {
    fn handle_message(
        &mut self,
        message: TournFileMessage,
        (): Self::Context<'_>,
    ) -> anyhow::Result<crate::traits::Effect<Self::Message, Self::OutMessage>> {
        match message {
            TournFileMessage::Open => todo!(),
            TournFileMessage::LoadTournament(path_buf) => Effect::task(Task::perform(
                load_from_file_async(path_buf),
                |res| match res {
                    Ok(value) => TournFileMessage::TournamentLoaded(value).into(),
                    Err(err) => Message::Error(err.to_string()),
                },
            )),
            TournFileMessage::SaveTournament(path_buf) => {
                let extension = require_extension(&path_buf)?;
                let serialized = serialize_by_extension(&self.tournament, extension)?;
                Effect::task(Task::perform(
                    async_fs::write(path_buf.clone(), serialized),
                    |res| match res {
                        Ok(()) => TournFileMessage::TournamentSaved.into(),
                        Err(e) => Message::Error(e.to_string()),
                    },
                ))
            }
            TournFileMessage::Save => todo!(),
            TournFileMessage::SaveAs => todo!(),
            TournFileMessage::TournamentLoaded(tournament) => {
                self.tournament = *tournament;
                Effect::ok()
            }
            TournFileMessage::TournamentSaved => todo!(),
        }
    }
}
