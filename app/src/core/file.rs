use std::path::{Path, PathBuf};

use directories::UserDirs;
use edh_tourn::tournament::Tournament;
use iced::{Task, widget::canvas::path::lyon_path::path_buffer};
use rfd::AsyncFileDialog;

use crate::{
    App,
    app::Message,
    core::state::AppStateMsg,
    effect::Effect,
    services::system::{
        accepted_file_types, load_from_file_async, require_extension, serialize_by_extension,
    },
    traits::HandleMessage,
};

#[derive(Clone, Debug)]
pub enum FileAction {
    Open,
    RequestNew,
    /// Creates a new tournament, bypassing unsaved changes check. Use [`Self::RequestNew`] to prompt
    /// the user if there are unsaved changes
    New,
    RequestOpenFile(PathBuf),
    OpenFile(PathBuf),
    FileOpened(PathBuf, Box<Tournament>),
    SaveFile(PathBuf),
    SaveError(String),
    FileSaved(PathBuf),
    Save,
    SaveAs,
    Cancelled,
}

async fn open_dialog(current_file: Option<PathBuf>) -> Message {
    let base_dir = current_file
        .and_then(|path| path.parent().map(Path::to_path_buf))
        .or_else(|| UserDirs::new()?.document_dir().map(Path::to_path_buf))
        .unwrap_or_else(|| PathBuf::from("."));

    let dialog = AsyncFileDialog::new()
        .add_filter("formats", &accepted_file_types())
        .set_directory(base_dir)
        .set_title("Open Tournament");

    let result = dialog.pick_file().await;

    result
        .map_or(FileAction::Cancelled, |file| {
            let path = file.path().to_path_buf();
            FileAction::OpenFile(path)
        })
        .into()
}

async fn save_dialog(current_file: Option<PathBuf>) -> Message {
    let base_dir = current_file
        .and_then(|path| path.parent().map(Path::to_path_buf))
        .or_else(|| UserDirs::new()?.document_dir().map(Path::to_path_buf))
        .unwrap_or_else(|| PathBuf::from("."));

    let dialog = AsyncFileDialog::new()
        .add_filter("formats", &accepted_file_types())
        .set_directory(base_dir)
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
    ) -> anyhow::Result<crate::effect::Effect<Self::Message, Self::OutMessage>> {
        match message {
            FileAction::RequestNew => {
                if self.modified {
                    Effect::confirm(
                        "Overwrite Tournament?".to_owned(),
                        "All unsaved changes will be lost".to_owned(),
                        Message::TournFile(FileAction::New),
                        None,
                    )
                    .ok()
                } else {
                    Effect::msg(FileAction::New).ok()
                }
            }
            FileAction::New => {
                self.tournament = Tournament::new();
                self.file = None;
                Effect::msg(AppStateMsg::ClearOpenedFile).ok()
            }
            FileAction::Open => Effect::future(open_dialog(self.file.clone())).ok(),
            FileAction::RequestOpenFile(path_buf) => {
                let action = FileAction::OpenFile(path_buf);
                if self.modified {
                    Effect::confirm(
                        "Lose Changes".to_owned(),
                        "All unsaved changes will be lost".to_owned(),
                        Message::TournFile(action),
                        None,
                    )
                    .ok()
                } else {
                    Effect::msg(action).ok()
                }
            }
            FileAction::OpenFile(path_buf) => {
                Effect::perform(load_from_file_async(path_buf.clone()), move |res| match res {
                    Ok(value) => FileAction::FileOpened(path_buf.clone(), value).into(),
                    Err(err) => Message::Error(err.to_string()),
                })
                .ok()
            }
            FileAction::SaveFile(path_buf) => {
                if self.is_saving {
                    return Effect::done();
                }
                self.is_saving = true;
                let extension = require_extension(&path_buf)?;
                let serialized = serialize_by_extension(&self.tournament, extension)?;
                let path = path_buf.clone();

                Effect::perform(
                    async move { async_fs::write(path, serialized).await },
                    move |res| match res {
                        Ok(()) => FileAction::FileSaved(path_buf.clone()).into(),
                        Err(e) => FileAction::SaveError(e.to_string()).into(),
                    },
                )
                .ok()
            }
            FileAction::Save => Effect::msg(
                self.file
                    .as_ref()
                    .map_or(FileAction::SaveAs, |path| FileAction::SaveFile(path.clone())),
            )
            .ok(),
            FileAction::SaveAs => Effect::future(save_dialog(self.file.clone())).ok(),
            FileAction::FileOpened(path, tournament) => {
                self.tournament = *tournament;
                self.file = Some(path.clone());
                self.modified = false;
                Effect::msg(AppStateMsg::SetOpenedFile(path)).ok()
            }
            FileAction::FileSaved(path_buf) => {
                self.file = Some(path_buf.clone());
                self.is_saving = false;
                self.modified = false;
                Effect::msg(AppStateMsg::SetOpenedFile(path_buf)).ok()
            }
            FileAction::Cancelled => Effect::done(),
            FileAction::SaveError(err) => {
                self.is_saving = false;
                Err(anyhow::anyhow!("Failed to save file: {err}"))
            }
        }
    }
}
