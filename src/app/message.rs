use crate::app::view::Screen;
use crate::app::view::home::HomeMessage;
use crate::app::view::player::{EditPlayer, EditPlayerMessage};

use super::App;
use super::traits::HandleMessage;
use iced::Task;
use opener::open_browser;

#[derive(Clone)]
pub enum Message {
    ClearError,
    OpenLink(String),
    EditPlayer(Option<u32>),
    CloseEditPlayer(bool),
    EditPlayerAction(EditPlayerMessage),
    HomeAction(HomeMessage),
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
        }
    }
}
