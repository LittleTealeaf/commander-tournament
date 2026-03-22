use std::path::PathBuf;

use edh_tourn::tournament::Tournament;
use iced::{Task, futures::FutureExt};
use rfd::AsyncFileDialog;

use crate::{
    App,
    core::message::Message,
    services::system::{
        accepted_file_types, load_from_file_async, require_extension, serialize_by_extension,
    },
    traits::{Effect, HandleMessage},
};

#[derive(Clone, Debug)]
pub enum FileAction {
    Open,
    New,
    OpenFile(PathBuf),
    FileOpened(PathBuf, Box<Tournament>),
    SaveFile(PathBuf),
    FileSaved,
    Save,
    SaveAs,
    Cancelled,
}

impl HandleMessage<FileAction> for App {
    fn handle_message(
        &mut self,
        message: FileAction,
        (): Self::UpdateContext<'_>,
    ) -> anyhow::Result<crate::traits::Effect<Self::Message, Self::OutMessage>> {
        match message {
            FileAction::New => {
                self.tournament = Tournament::new();
                self.file = None;
                Effect::done()
            }
            FileAction::Open => Effect::task(Task::perform(
                AsyncFileDialog::new()
                    .add_filter("formats", &accepted_file_types())
                    .set_directory(".")
                    .set_title("Open Tournament")
                    .pick_file()
                    .then(async |res| res.map(|handle| handle.path().to_path_buf())),
                |result| {
                    result
                        .map_or(FileAction::Cancelled, FileAction::OpenFile)
                        .into()
                },
            )),
            FileAction::OpenFile(path_buf) => Effect::task(Task::perform(
                load_from_file_async(path_buf.clone()),
                |res| match res {
                    Ok(value) => FileAction::FileOpened(path_buf, value).into(),
                    Err(err) => Message::Error(err.to_string()),
                },
            )),
            FileAction::SaveFile(path_buf) => {
                let extension = require_extension(&path_buf)?;
                let serialized = serialize_by_extension(&self.tournament, extension)?;
                Effect::task(Task::perform(
                    async_fs::write(path_buf.clone(), serialized),
                    |res| match res {
                        Ok(()) => FileAction::FileSaved.into(),
                        Err(e) => Message::Error(e.to_string()),
                    },
                ))
            }
            FileAction::Save => {
                if let Some(path) = &self.file {
                    self.handle_message(FileAction::SaveFile(path.clone()), ())
                } else {
                    self.handle_message(FileAction::SaveAs, ())
                }
            }
            FileAction::SaveAs => {
                let future = async {
                    let result = AsyncFileDialog::new()
                        .add_filter("formats", &accepted_file_types())
                        .set_directory(".")
                        .set_title("Save Tournament")
                        .save_file()
                        .await;
                    result
                        .map_or(FileAction::Cancelled, |file| {
                            FileAction::SaveFile(file.path().to_path_buf())
                        })
                        .into()
                };
                Effect::task(Task::future(future))
            }
            FileAction::FileOpened(path, tournament) => {
                self.tournament = *tournament;
                self.file = Some(path.clone());
                self.handle_message(
                    crate::core::settings::AppSettingsMsg::SetOpenedFile(path),
                    (),
                )
            }
            FileAction::Cancelled | FileAction::FileSaved => Effect::done(),
        }
    }
}
