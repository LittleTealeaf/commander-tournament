use edh_tourn::tournament::Tournament;
use iced_tea::{Model, Signal};

use crate::{
    popup::confirm::ConfirmPopup,
    views::player::{PlayerDetailsOut, PlayerView},
};

use super::PlayerDetailsMsg;

impl Model for PlayerView {
    type Message = PlayerDetailsMsg;
    type OutMessage = PlayerDetailsOut;
    type Context<'a> = &'a Tournament;

    fn update<'a>(
        &'a mut self,
        message: Self::Message,
        _context: Self::Context<'a>,
    ) -> anyhow::Result<Signal<Self::Message, Self::OutMessage>> {
        match message {
            PlayerDetailsMsg::SetArchived(is_archived) => {
                self.info.set_is_archived(is_archived);
                self.modified = true;
                Signal::done()
            }
            PlayerDetailsMsg::SetIsPrecon(is_precon) => {
                self.info.set_is_precon(is_precon);
                self.modified = true;
                Signal::done()
            }
            PlayerDetailsMsg::SelectPlayerReference(id) => {
                if Some(id) == self.id {
                    Signal::done()
                } else {
                    Signal::out(PlayerDetailsOut::OpenPlayerDetails(id)).ok()
                }
            }
            PlayerDetailsMsg::SaveAndClose => {
                self.info.set_description(self.description.text());
                Signal::out(PlayerDetailsOut::SaveAndClose(self.id, self.info.clone())).ok()
            }
            PlayerDetailsMsg::SetName(name) => {
                self.info.set_name(name);
                self.modified = true;
                Signal::done()
            }
            PlayerDetailsMsg::EditDescription(action) => {
                self.description.perform(action);
                self.modified = true;
                Signal::done()
            }
            PlayerDetailsMsg::SetMoxfieldId(id) => {
                self.info.set_moxfield_id(id.clone());
                self.moxfield_id = id;
                self.modified = true;
                Signal::done()
            }
            PlayerDetailsMsg::ToggleColor(mtg_color) => {
                self.info.toggle_color(mtg_color);
                self.modified = true;
                Signal::done()
            }
            PlayerDetailsMsg::SetStatsTab(stats_tab) => {
                self.stats = stats_tab;
                Signal::done()
            }
            PlayerDetailsMsg::OpenLink(link) => Signal::out(PlayerDetailsOut::OpenLink(link)).ok(),
            PlayerDetailsMsg::ConfirmDelete => self
                .id
                .map(|id| Signal::out(PlayerDetailsOut::DeletePlayer(id)))
                .unwrap_or_default()
                .ok(),
            PlayerDetailsMsg::RequestDelete => {
                self.confirm_popup = Some(ConfirmPopup::new(
                    format!("Delete {}", self.initial_name),
                    format!(
                        "Are you sure you want to delete the player \"{}\"? All games this player has participated in will also be deleted.",
                        self.initial_name
                    ),
                    PlayerDetailsMsg::ConfirmDelete,
                    PlayerDetailsMsg::ClearRequest,
                ));
                Signal::done()
            }
            PlayerDetailsMsg::OpenNextPlayerMatch => self
                .id
                .map(|id| Signal::out(PlayerDetailsOut::OpenPlayerMatches(id)))
                .unwrap_or_default()
                .ok(),
            PlayerDetailsMsg::RequestClose => {
                if self.modified {
                    self.confirm_popup = Some(ConfirmPopup::new(
                        "Close without saving?".to_owned(),
                        "Are you sure you want to close without saving your changes?".to_owned(),
                        PlayerDetailsMsg::Close,
                        PlayerDetailsMsg::ClearRequest,
                    ));
                    Signal::done()
                } else {
                    Signal::out(PlayerDetailsOut::Close).ok()
                }
            }
            PlayerDetailsMsg::Close => Signal::out(PlayerDetailsOut::Close).ok(),
            PlayerDetailsMsg::ClearRequest => {
                self.confirm_popup = None;
                Signal::done()
            }
        }
    }
}
