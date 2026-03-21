mod update;
mod view;

use edh_tourn::{
    player::{RegisteredPlayer, color::MtgColor, info::PlayerInfo},
    tournament::Tournament,
};
use iced::widget::text_editor;

use crate::traits::Component;

#[derive(Debug, Clone)]
pub struct State {
    id: Option<u32>,
    name: String,
    info: PlayerInfo,
    moxfield_id: String,
    modified: bool,
    description: text_editor::Content,
    stats: StatsTab,
}

#[derive(Debug, Clone, Default)]
pub enum StatsTab {
    #[default]
    Games,
    Players,
    Identities,
    Colors,
}

#[derive(Debug, Clone)]
pub enum Message {
    SaveAndClose,
    SetName(String),
    EditDescription(text_editor::Action),
    SetMoxfieldId(String),
    ToggleColor(MtgColor),
    SetStatsTab(StatsTab),
}

#[derive(Debug)]
pub enum OutMessage {
    SaveAndClose(Option<u32>, PlayerInfo),
    Close,
}

impl State {
    #[must_use]
    pub fn new(&self, player: Option<RegisteredPlayer<'_>>) -> Self {
        let id = player.as_ref().map(RegisteredPlayer::id);
        let info = player.map(|p| p.info().clone()).unwrap_or_default();
        let name = info.name().clone();
        let moxfield_id = info.moxfield_id().cloned().unwrap_or_default();
        let description = text_editor::Content::with_text(info.description());

        Self {
            id,
            info,
            name,
            moxfield_id,
            description,
            stats: StatsTab::Games,
            modified: false,
        }
    }
}

impl Component for State {
    type Message = Message;
    type Context<'a> = &'a Tournament;
    type OutMessage = OutMessage;
}
