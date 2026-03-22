mod update;
mod view;

use edh_tourn::player::{RegisteredPlayer, color::MtgColor, info::PlayerInfo};
use iced::widget::text_editor;

use crate::{
    components::prompt::{self, DialogPrompt},
    traits::Component,
};

#[derive(Debug, Clone)]
pub struct State {
    id: Option<u32>,
    initial_name: String,
    info: PlayerInfo,
    moxfield_id: String,
    modified: bool,
    description: text_editor::Content,
    stats: StatsTab,
    prompt_confirm_delete: Option<DialogPrompt>,
}

#[derive(Copy, Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, derive_more::Display)]
pub enum StatsTab {
    #[default]
    Games,
    Players,
    Identities,
    Colors,
}

impl StatsTab {
    const VALUES: [Self; 4] = [Self::Games, Self::Players, Self::Identities, Self::Colors];
}

#[derive(Debug, Clone)]
pub enum Message {
    SaveAndClose,
    Close,
    SetName(String),
    EditDescription(text_editor::Action),
    SetMoxfieldId(String),
    ToggleColor(MtgColor),
    SetStatsTab(StatsTab),
    SelectPlayerReference(u32),
    OpenLink(String),
    /// Opens the dialog to delete the player
    DeletePlayer,
    Dialog(prompt::Message),
}

#[derive(Debug)]
pub enum OutMessage {
    OpenPlayer(u32),
    SaveAndClose(Option<u32>, PlayerInfo),
    DeletePlayer(u32),
    Close,
}

impl State {
    #[must_use]
    pub fn new(player: Option<RegisteredPlayer<'_>>) -> Self {
        let id = player.as_ref().map(RegisteredPlayer::id);
        let info = player.map(|p| p.info().clone()).unwrap_or_default();
        let name = info.name().clone();
        let moxfield_id = info.moxfield_id().cloned().unwrap_or_default();
        let description = text_editor::Content::with_text(info.description());

        Self {
            id,
            info,
            initial_name: name,
            moxfield_id,
            description,
            stats: StatsTab::Games,
            modified: false,
            prompt_confirm_delete: None,
        }
    }
}

impl Component for State {
    type Message = Message;
    type OutMessage = OutMessage;
}
