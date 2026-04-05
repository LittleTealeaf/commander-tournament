use crate::{effect::Effect, player_details::PlayerDetailsOut, traits::ComponentUpdate};

use super::{PlayerDetails, PlayerDetailsMsg};

impl ComponentUpdate for PlayerDetails {
    type UpdateContext<'a> = ();
    fn update(
        &mut self,
        message: Self::Message,
        (): Self::UpdateContext<'_>,
    ) -> anyhow::Result<Effect<Self::Message, Self::OutMessage>> {
        match message {
            PlayerDetailsMsg::SelectPlayerReference(id) => {
                if Some(id) == self.id {
                    Effect::done()
                } else {
                    Effect::out(PlayerDetailsOut::OpenPlayerDetails(id)).ok()
                }
            }
            PlayerDetailsMsg::SaveAndClose => {
                self.info.set_description(self.description.text());
                Effect::out(PlayerDetailsOut::SaveAndClose(self.id, self.info.clone())).ok()
            }
            PlayerDetailsMsg::SetName(name) => {
                self.info.set_name(name);
                self.modified = true;
                Effect::done()
            }
            PlayerDetailsMsg::EditDescription(action) => {
                self.description.perform(action);
                self.modified = true;
                Effect::done()
            }
            PlayerDetailsMsg::SetMoxfieldId(id) => {
                self.info.set_moxfield_id(id.clone());
                self.moxfield_id = id;
                self.modified = true;
                Effect::done()
            }
            PlayerDetailsMsg::ToggleColor(mtg_color) => {
                self.info.toggle_color(mtg_color);
                self.modified = true;
                Effect::done()
            }
            PlayerDetailsMsg::SetStatsTab(stats_tab) => {
                self.stats = stats_tab;
                Effect::done()
            }
            PlayerDetailsMsg::OpenLink(link) => Effect::out(PlayerDetailsOut::OpenLink(link)).ok(),
            PlayerDetailsMsg::ConfirmDelete => {
                if self.confirm_delete {
                    self.id
                        .map(|id| Effect::out(PlayerDetailsOut::DeletePlayer(id)))
                        .unwrap_or_default()
                        .ok()
                } else {
                    Effect::done()
                }
            }
            PlayerDetailsMsg::RequestDelete => {
                self.confirm_delete = true;
                Effect::done()
            }
            PlayerDetailsMsg::OpenNextPlayerMatch => self
                .id
                .map(|id| Effect::out(PlayerDetailsOut::OpenPlayerMatches(id)))
                .unwrap_or_default()
                .ok(),
            PlayerDetailsMsg::Close => Effect::out(PlayerDetailsOut::Close).ok(),
            PlayerDetailsMsg::CancelDelete => {
                self.confirm_delete = false;
                Effect::done()
            }
        }
    }
}
