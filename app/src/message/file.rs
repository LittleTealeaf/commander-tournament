use std::path::PathBuf;

use edh_tourn::tournament::Tournament;
use iced::{Task, futures::FutureExt};
use rfd::AsyncFileDialog;

use crate::{
    App,
    message::Message,
    services::system::{
        accepted_file_types, load_from_file_async, require_extension, serialize_by_extension,
    },
    traits::{Effect, HandleMessage},
};

#[derive(Clone, Debug)]
pub enum TournamentFileMessage {
    Open,
    New,
    LoadTournament(PathBuf),
    TournamentLoaded(PathBuf, Box<Tournament>),
    SaveTournament(PathBuf),
    None,
    Save,
    SaveAs,
}

impl HandleMessage<TournamentFileMessage> for App {
    fn handle_message(
        &mut self,
        message: TournamentFileMessage,
        (): Self::Context<'_>,
    ) -> anyhow::Result<crate::traits::Effect<Self::Message, Self::OutMessage>> {
        match message {
            TournamentFileMessage::New => Effect::ok(),
            TournamentFileMessage::Open => Effect::task(Task::perform(
                AsyncFileDialog::new()
                    .add_filter("formats", &accepted_file_types())
                    .set_directory(".")
                    .set_title("Open Tournament")
                    .pick_file()
                    .then(async |res| res.map(|handle| handle.path().to_path_buf())),
                |result| {
                    result
                        .map_or(TournamentFileMessage::None, |path| {
                            TournamentFileMessage::LoadTournament(path)
                        })
                        .into()
                },
            )),
            TournamentFileMessage::LoadTournament(path_buf) => Effect::task(Task::perform(
                load_from_file_async(path_buf.clone()),
                |res| match res {
                    Ok(value) => TournamentFileMessage::TournamentLoaded(path_buf, value).into(),
                    Err(err) => Message::Error(err.to_string()),
                },
            )),
            TournamentFileMessage::SaveTournament(path_buf) => {
                let extension = require_extension(&path_buf)?;
                let serialized = serialize_by_extension(&self.tournament, extension)?;
                Effect::task(Task::perform(
                    async_fs::write(path_buf.clone(), serialized),
                    |res| match res {
                        Ok(()) => TournamentFileMessage::None.into(),
                        Err(e) => Message::Error(e.to_string()),
                    },
                ))
            }
            TournamentFileMessage::Save => todo!(),
            TournamentFileMessage::SaveAs => todo!(),
            TournamentFileMessage::TournamentLoaded(path, tournament) => {
                self.tournament = *tournament;
                self.handle_message(crate::settings::Message::SetOpenedFile(path), ())
            }
            TournamentFileMessage::None => todo!(),
        }
    }
}
