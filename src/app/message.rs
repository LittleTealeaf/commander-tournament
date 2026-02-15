use std::path::PathBuf;

use crate::app::view::Screen;
use crate::app::view::home::HomeMessage;
use crate::app::view::player::{EditPlayer, EditPlayerMessage};

use super::App;
use super::traits::HandleMessage;
use commander_tournament::Tournament;
use iced::Task;
use iced::futures::FutureExt;
use opener::open_browser;
use rfd::AsyncFileDialog;
use ron::ser::PrettyConfig;

#[derive(Clone, Default)]
pub enum Message {
    ClearError,
    OpenLink(String),
    EditPlayer(Option<u32>),
    CloseEditPlayer(bool),
    EditPlayerAction(EditPlayerMessage),
    HomeAction(HomeMessage),
    SaveToFile(PathBuf),
    OpenFile(PathBuf),
    LoadTournament(Box<Tournament>),
    LoadSerialized(String),
    Error(String),
    Save,
    SaveAs,
    Open,
    New,
    #[default]
    None,
}

impl Message {
    fn done() -> anyhow::Result<Task<Message>> {
        Ok(Task::none())
    }

    fn handle_result<T, E>(
        on_success: impl Fn(T) -> Self + 'static,
    ) -> impl Fn(Result<T, E>) -> Self
    where
        E: ToString,
    {
        move |result| match result {
            Ok(value) => on_success(value),
            Err(error) => Self::Error(error.to_string()),
        }
    }
}

impl HandleMessage<Message> for App {
    fn update(&mut self, msg: Message) -> anyhow::Result<Task<Message>> {
        match msg {
            Message::ClearError => {
                self.error = None;
                Message::done()
            }
            Message::OpenLink(link) => {
                open_browser(link)?;
                Message::done()
            }
            Message::EditPlayer(some_id) => {
                self.screen = Some(Screen::Player(EditPlayer::create(
                    &self.tournament,
                    some_id,
                )));
                Message::done()
            }
            Message::CloseEditPlayer(submit) => {
                if let Some(Screen::Player(player)) = &mut self.screen {
                    if submit {
                        player.submit(&mut self.tournament)?;
                    }

                    self.screen = None;
                }
                Message::done()
            }
            Message::EditPlayerAction(msg) => {
                if let Some(Screen::Player(player)) = &mut self.screen {
                    player.update(msg)
                } else {
                    Message::done()
                }
            }
            Message::HomeAction(action) => self.update(action),
            Message::SaveToFile(path) => Ok(Task::perform(
                async_fs::write(
                    path,
                    ron::ser::to_string_pretty(&self.tournament, PrettyConfig::default())?,
                ),
                Message::handle_result(|()| Message::None),
            )),
            Message::OpenFile(path) => Ok(Task::perform(
                async_fs::read_to_string(path),
                Message::handle_result(Message::LoadSerialized),
            )),
            Message::LoadSerialized(serialized) => {
                self.tournament = ron::de::from_str(&serialized)?;
                Message::done()
            }
            Message::Save => self.update(
                self.file
                    .clone()
                    .map(Message::OpenFile)
                    .unwrap_or(Message::SaveAs),
            ),
            Message::SaveAs => Ok(Task::perform(
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
            )),
            Message::Open => Ok(Task::perform(
                async {
                    match AsyncFileDialog::new()
                        .add_filter("app", &["ron", "json"])
                        .set_directory(".")
                        .pick_file()
                        .await
                    {
                        None => None,
                        Some(file) => Some(file.read().await),
                    }
                },
                |result| match result {
                    Some(bytes) => match ron::de::from_bytes(&bytes) {
                        Ok(result) => Message::LoadTournament(result),
                        Err(error) => Message::Error(error.to_string()),
                    },
                    None => Message::None,
                },
            )),
            Message::New => {
                self.file = None;
                self.tournament = Tournament::default();
                Message::done()
            }
            Message::Error(error) => {
                self.error = Some(error);
                Message::done()
            }
            Message::None => Message::done(),
            Message::LoadTournament(tournament) => {
                self.tournament = *tournament;
                Message::done()
            }
        }
    }
}
