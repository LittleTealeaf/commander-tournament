mod update;
mod view;

use edh_tourn::player::{PlayerId, RegisteredPlayer, color::MtgColor, info::PlayerInfo};
use iced::widget::{button, text_editor};
use nerd_font_symbols::md::{MD_CONTENT_SAVE, MD_DELETE};

use crate::{traits::Component, views::ViewScreen};

#[derive(Debug, Clone)]
pub struct PlayerView {
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
    Close,
    SaveAndClose,
    SetName(String),
    EditDescription(text_editor::Action),
    SetMoxfieldId(String),
    ToggleColor(MtgColor),
    SetStatsTab(StatsTab),
    SelectPlayerReference(PlayerId),
    OpenLink(String),
    SetIsPrecon(bool),
    /// Opens the dialog to delete the player
    RequestDelete,
    ConfirmDelete,
    OpenNextPlayerMatch,
}

#[derive(Debug)]
pub enum PlayerDetailsOut {
    Close,
    SaveAndClose(Option<PlayerId>, PlayerInfo),
    OpenPlayerDetails(PlayerId),
    DeletePlayer(PlayerId),
    OpenLink(String),
    OpenPlayerMatches(PlayerId),
}

impl PlayerView {
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

impl Component for PlayerView {
    type Message = PlayerDetailsMsg;
    type OutMessage = PlayerDetailsOut;
}

impl ViewScreen for PlayerView {
    const CLOSE_MESSAGE: Self::Message = PlayerDetailsMsg::Close;

    fn primary_actions<'a>(
        &'a self,
        _: Self::ViewContext<'a>,
    ) -> impl IntoIterator<Item = iced::widget::Button<'a, Self::Message>> {
        [button(MD_CONTENT_SAVE)
            .on_press_maybe(self.modified.then_some(PlayerDetailsMsg::SaveAndClose))]
    }

    fn secondary_actions<'a>(
        &'a self,
        _: Self::ViewContext<'a>,
    ) -> impl IntoIterator<Item = button::Button<'a, Self::Message>> {
        self.id.is_some().then(|| {
            button(MD_DELETE)
                .style(button::danger)
                .on_press(PlayerDetailsMsg::RequestDelete)
        })
    }

    fn title<'a>(&'a self, _: Self::ViewContext<'a>) -> String {
        if self.id.is_some() {
            self.initial_name.clone()
        } else {
            "New Player".to_owned()
        }
    }
}
