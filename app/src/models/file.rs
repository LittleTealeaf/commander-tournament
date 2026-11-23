use std::path::{Path, PathBuf};

use directories::UserDirs;
use iced_tea::Signal;
use rfd::AsyncFileDialog;
use serde::{Deserialize, Serialize};

use crate::services::system::{
    accepted_file_types, load_from_file_async, require_extension, serialize_by_extension,
};

#[derive(Debug, Default)]
pub enum FileModel<T>
where
    T: for<'a> Deserialize<'a> + Serialize + Clone,
{
    #[default]
    Closed,
    Opened {
        data: T,
        path: Option<PathBuf>,
        is_saving: bool,
    },
}

impl<T> FileModel<T>
where
    T: for<'a> Deserialize<'a> + Serialize + Clone,
{
    pub fn create(&mut self, data: T) {
        *self = Self::Opened {
            data,
            path: None,
            is_saving: false,
        }
    }

    pub const fn data(&self) -> Option<&T> {
        match self {
            Self::Opened { data, .. } => Some(data),
            Self::Closed => None,
        }
    }

    pub const fn data_mut(&mut self) -> Option<&mut T> {
        match self {
            Self::Opened { data, .. } => Some(data),
            Self::Closed => None,
        }
    }

    pub const fn is_saving(&self) -> bool {
        match self {
            Self::Opened { is_saving, .. } => *is_saving,
            Self::Closed => false,
        }
    }

    pub const fn save_path(&self) -> Option<&PathBuf> {
        match self {
            Self::Opened { path, .. } => path.as_ref(),
            Self::Closed => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum FileModelMsg<T>
where
    T: for<'a> Deserialize<'a> + Serialize + Clone,
{
    Close,
    FileOpened(PathBuf, Box<T>),
    SaveFile(PathBuf),
    FileSaved(PathBuf),
    Save,
    SaveAs,
    Open,
    OpenFile(PathBuf),
    Error(String),
    Cancelled,
}

#[derive(Debug, Clone)]
pub enum FileModelOut {
    Opened(PathBuf),
    Saved(PathBuf),
    Closed,
}

impl<T> iced_tea::Model for FileModel<T>
where
    T: for<'a> Deserialize<'a> + Serialize + Clone + Send + 'static,
{
    type Message = FileModelMsg<T>;

    type OutMessage = FileModelOut;

    type Context<'b>
        = ()
    where
        Self: 'b;

    fn update<'a>(
        &'a mut self,
        message: Self::Message,
        (): Self::Context<'a>,
    ) -> anyhow::Result<Signal<Self::Message, Self::OutMessage>> {
        match message {
            FileModelMsg::Close => {
                if !self.is_saving() {
                    *self = Self::Closed;
                }
                Signal::done()
            }
            FileModelMsg::OpenFile(path_buf) => Ok(Signal::perform(
                load_from_file_async(path_buf.clone()),
                move |res| match res {
                    Ok(value) => FileModelMsg::FileOpened(path_buf.clone(), value),
                    Err(err) => FileModelMsg::Error(err.to_string()),
                },
            )),
            FileModelMsg::FileOpened(path, data) => {
                if !self.is_saving() {
                    *self = Self::Opened {
                        data: *data,
                        path: Some(path),
                        is_saving: false,
                    };
                }
                Signal::done()
            }
            FileModelMsg::SaveFile(path_buf) => {
                let Self::Opened { data, is_saving, .. } = self else {
                    return Signal::done();
                };

                if *is_saving {
                    return Signal::done();
                }
                *is_saving = true;
                let extension = require_extension(&path_buf)?;
                let serialized = serialize_by_extension(data, extension)?;
                let path = path_buf.clone();
                Ok(Signal::perform(
                    async move { async_fs::write(path, serialized).await },
                    move |res| match res {
                        Ok(()) => FileModelMsg::FileSaved(path_buf.clone()),
                        Err(e) => FileModelMsg::Error(e.to_string()),
                    },
                ))
            }
            FileModelMsg::FileSaved(path_buf) => {
                let Self::Opened { path, is_saving, .. } = self else {
                    return Err(anyhow::anyhow!("File saved without being opened"));
                };

                *is_saving = false;
                *path = Some(path_buf);
                Signal::done()
            }
            FileModelMsg::Save => Ok(self.save_path().map_or_else(
                || Signal::msg(FileModelMsg::SaveAs),
                |path| Signal::msg(FileModelMsg::SaveFile(path.clone())),
            )),
            FileModelMsg::SaveAs => Ok(Signal::future({
                let current_file = self.save_path().cloned();
                async move {
                    let base_dir = current_file
                        .and_then(|path| path.parent().map(Path::to_path_buf))
                        .or_else(|| UserDirs::new()?.document_dir().map(Path::to_path_buf))
                        .unwrap_or_else(|| PathBuf::from("."));
                    let dialog = AsyncFileDialog::new()
                        .add_filter("formats", &accepted_file_types())
                        .set_directory(base_dir)
                        .set_title("Save File")
                        .save_file();
                    let result = dialog.await;
                    result.map_or(FileModelMsg::Cancelled, |file| {
                        let path = file.path().to_path_buf();
                        FileModelMsg::SaveFile(path)
                    })
                }
            })),
            FileModelMsg::Open => Ok(Signal::future({
                let current_file = self.save_path().cloned();
                async move {
                    let base_dir = current_file
                        .and_then(|path| path.parent().map(Path::to_path_buf))
                        .or_else(|| UserDirs::new()?.document_dir().map(Path::to_path_buf))
                        .unwrap_or_else(|| PathBuf::from("."));
                    let dialog = AsyncFileDialog::new()
                        .add_filter("formats", &accepted_file_types())
                        .set_directory(base_dir)
                        .set_title("Open File");
                    let result = dialog.pick_file().await;
                    result.map_or(FileModelMsg::Cancelled, |file| {
                        let path = file.path().to_path_buf();
                        FileModelMsg::OpenFile(path)
                    })
                }
            })),
            FileModelMsg::Error(error) => Err(anyhow::anyhow!("File Error: {error}")),
            FileModelMsg::Cancelled => Signal::done(),
        }
    }
}
