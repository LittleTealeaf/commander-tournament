use commander_tournament::{
    error::TournamentError,
    info::{MtgColor, PlayerInfo},
};
use iced::Element;

use crate::app::Message;

pub struct ViewPlayerInfo {
    id: usize,
    info: PlayerInfo,
}

pub enum MessagePlayerInfo {
    SetName(String),
    SetDescription(String),
    ToggleColor(MtgColor),
    SetMoxfieldId(String),
}

impl From<MessagePlayerInfo> for Message {
    fn from(value: MessagePlayerInfo) -> Self {
        Self::ViewPlayerInfo(value)
    }
}

impl ViewPlayerInfo {
    pub fn update(&mut self, message: MessagePlayerInfo) -> Result<(), TournamentError> {
        match message {
            MessagePlayerInfo::SetName(name) => {
                self.info.set_name(name);
            }
            MessagePlayerInfo::SetDescription(description) => {
                self.info.set_description(description);
            }
            MessagePlayerInfo::ToggleColor(color) => {
                self.info.toggle_color(color);
            }
            MessagePlayerInfo::SetMoxfieldId(id) => {
                if id.is_empty() {
                    self.info.set_moxfield_id(None);
                } else {
                    self.info.set_moxfield_id(Some(id));
                }
            }
        }

        Ok(())
    }

    pub fn view(&self) -> Element<'_, Message> {
        todo!()
    }
}

