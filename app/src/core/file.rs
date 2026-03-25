use std::path::PathBuf;

use edh_tourn::tournament::Tournament;
use iced::Task;
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

async fn open_dialog() -> Message {
    let dialog = AsyncFileDialog::new()
        .add_filter("formats", &accepted_file_types())
        .set_directory(".")
        .set_title("Open Tournament");

    let result = dialog.pick_file().await;

    result
        .map_or(FileAction::Cancelled, |file| {
            let path = file.path().to_path_buf();
            FileAction::OpenFile(path)
        })
        .into()
}

async fn save_dialog() -> Message {
    let dialog = AsyncFileDialog::new()
        .add_filter("formats", &accepted_file_types())
        .set_directory(".")
        .set_title("Save Tournament")
        .save_file();

    let result = dialog.await;

    result
        .map_or(FileAction::Cancelled, |file| {
            let path = file.path().to_path_buf();
            FileAction::SaveFile(path)
        })
        .into()
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
            FileAction::Open => Effect::future(open_dialog()).ok(),
            FileAction::OpenFile(path_buf) => Effect::Task(Task::perform(
                load_from_file_async(path_buf.clone()),
                |res| match res {
                    Ok(value) => FileAction::FileOpened(path_buf, value).into(),
                    Err(err) => Message::Error(err.to_string()),
                },
            ))
            .ok(),
            FileAction::SaveFile(path_buf) => {
                if self.is_saving {
                    return Effect::done();
                }
                self.is_saving = true;
                let extension = require_extension(&path_buf)?;
                let serialized = serialize_by_extension(&self.tournament, extension)?;
                let future = async move {
                    let res = async_fs::write(path_buf.clone(), serialized).await;
                    match res {
                        Ok(()) => FileAction::FileSaved.into(),
                        Err(e) => Message::Error(e.to_string()),
                    }
                };
                Effect::future(future).ok()
            }
            FileAction::Save => {
                if let Some(path) = &self.file {
                    self.handle_message(FileAction::SaveFile(path.clone()), ())
                } else {
                    self.handle_message(FileAction::SaveAs, ())
                }
            }
            FileAction::SaveAs => Effect::future(save_dialog()).ok(),
            FileAction::FileOpened(path, tournament) => {
                self.tournament = *tournament;
                self.file = Some(path.clone());
                self.modified = false;
                self.handle_message(crate::core::state::AppStateMsg::SetOpenedFile(path), ())
            }
            FileAction::FileSaved => {
                self.is_saving = false;
                self.modified = false;
                Effect::done()
            }
            FileAction::Cancelled => Effect::done(),
        }
    }
}
