use std::fs;
use std::path::PathBuf;

use crate::app::view::Screen;
use crate::app::view::home::HomeMessage;
use crate::app::view::player::{EditPlayer, EditPlayerMessage};

use super::App;
use super::traits::HandleMessage;
use commander_tournament::Tournament;
use iced::Task;
use opener::open_browser;
use rfd::AsyncFileDialog;
use ron::ser::PrettyConfig;

#[derive(Clone)]
pub enum Message {
    ClearError,
    OpenLink(String),
    EditPlayer(Option<u32>),
    CloseEditPlayer(bool),
    EditPlayerAction(EditPlayerMessage),
    HomeAction(HomeMessage),
    SaveToFile(PathBuf),
    OpenFile(PathBuf),
    LoadSerialized(String),
    Error(String),
    Save,
    SaveAs,
    Open,
    New,
    None,
}

impl HandleMessage<Message> for App {
    fn update(&mut self, msg: Message) -> anyhow::Result<Option<Task<Message>>> {
        match msg {
            Message::ClearError => {
                self.error = None;
                Ok(None)
            }
            Message::OpenLink(link) => {
                open_browser(link)?;
                Ok(None)
            }
            Message::EditPlayer(some_id) => {
                self.screen = Some(Screen::Player(EditPlayer::create(
                    &self.tournament,
                    some_id,
                )));
                Ok(None)
            }
            Message::CloseEditPlayer(submit) => {
                if let Some(Screen::Player(player)) = &mut self.screen {
                    if submit {
                        player.submit(&mut self.tournament)?;
                    }

                    self.screen = None;
                }
                Ok(None)
            }
            Message::EditPlayerAction(msg) => {
                if let Some(Screen::Player(player)) = &mut self.screen {
                    player.update(msg)
                } else {
                    Ok(None)
                }
            }
            Message::HomeAction(action) => self.update(action),
            Message::SaveToFile(path) => Ok(Some(Task::perform(
                async_fs::write(
                    path,
                    ron::ser::to_string_pretty(&self.tournament, PrettyConfig::default())?,
                ),
                |result| match result {
                    Ok(()) => Message::None,
                    Err(error) => Message::Error(error.to_string()),
                },
            ))),
            Message::OpenFile(path) => Ok(Some(Task::perform(
                async_fs::read_to_string(path),
                |result| match result {
                    Ok(string) => Message::LoadSerialized(string),
                    Err(err) => Message::Error(err.to_string()),
                },
            ))),
            Message::LoadSerialized(serialized) => {
                self.tournament = ron::de::from_str(&serialized)?;
                Ok(None)
            }
            Message::Save => self.update(
                self.file
                    .clone()
                    .map(Message::OpenFile)
                    .unwrap_or(Message::SaveAs),
            ),
            Message::SaveAs => Ok(Some(Task::perform(
                AsyncFileDialog::new()
                    .set_title("Save Tournament As")
                    .add_filter("app", &["ron"])
                    .set_directory(".")
                    .save_file(),
                |res| {
                    if let Some(file_handle) = res {
                        let path_buf = file_handle.path().to_path_buf();
                        Message::SaveToFile(path_buf)
                    } else {
                        Message::None
                    }
                },
            ))),
            Message::Open => Ok(Some(Task::perform(
                AsyncFileDialog::new()
                    .add_filter("app", &["ron"])
                    .set_directory(".")
                    .pick_file(),
                |res| {
                    if let Some(file_handle) = res {
                        let path_buf = file_handle.path().to_path_buf();
                        Message::OpenFile(path_buf)
                    } else {
                        Message::None
                    }
                },
            ))),
            Message::New => {
                self.file = None;
                self.tournament = Tournament::default();
                Ok(None)
            }
            Message::Error(error) => {
                self.error = Some(error);
                Ok(None)
            }
            Message::None => Ok(None),
        }
    }
}
