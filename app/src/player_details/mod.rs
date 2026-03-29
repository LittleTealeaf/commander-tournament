mod update;
mod view;

use edh_tourn::player::{PlayerId, RegisteredPlayer, color::MtgColor, info::PlayerInfo};
use iced::widget::text_editor;

use crate::{components::confirm::ConfirmDialog, traits::Component};

#[derive(Debug, Clone)]
pub struct PlayerDetails {
    id: Option<PlayerId>,
    initial_name: String,
    info: PlayerInfo,
    moxfield_id: String,
    modified: bool,
    description: text_editor::Content,
    stats: StatsTab,
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
pub enum PlayerDetailsMsg {
    SaveAndClose,
    Close,
    SetName(String),
    EditDescription(text_editor::Action),
    SetMoxfieldId(String),
    ToggleColor(MtgColor),
    SetStatsTab(StatsTab),
    SelectPlayerReference(PlayerId),
    OpenLink(String),
    /// Opens the dialog to delete the player
    DeletePlayer,
    ConfirmDelete,
}

#[derive(Debug)]
pub enum PlayerDetailsOut {
    Save(Option<PlayerId>, PlayerInfo),
    OpenPlayerDetails(PlayerId),
    DeletePlayer(PlayerId),
    OpenLink(String),
    ConfirmDialog(Box<ConfirmDialog<PlayerDetailsMsg>>),
    Close,
}

impl PlayerDetails {
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
        }
    }
}

impl Component for PlayerDetails {
    type Message = PlayerDetailsMsg;
    type OutMessage = PlayerDetailsOut;
}
