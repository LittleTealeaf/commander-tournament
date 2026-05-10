use crate::{effect::Effect, traits::ComponentUpdate, views::player::PlayerDetailsOut};

use super::{PlayerDetailsMsg, PlayerView};

impl ComponentUpdate for PlayerView {
    type UpdateContext<'a> = ();
    fn update(
        &mut self,
        message: Self::Message,
        (): Self::UpdateContext<'_>,
    ) -> anyhow::Result<Effect<Self::Message, Self::OutMessage>> {
        match message {
            PlayerDetailsMsg::SetIsPrecon(is_precon) => {
                self.info.set_precon(is_precon);
                self.modified = true;
                Effect::done()
            }
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
            PlayerDetailsMsg::OpenLink(link) => Effect::out(PlayerDetailsOut::OpenLink(link)).ok(), PlayerDetailsMsg::ConfirmDelete => self.id.map(|id| Effect::out(PlayerDetailsOut::DeletePlayer(id))).unwrap_or_default().ok(), PlayerDetailsMsg::RequestDelete => Effect::confirm( format!("Delete {}?", self.initial_name),
                format!(
                    "Are you sure you want to delete the player \"{}\"? All games this player has participated in will also be deleted.",
                    self.initial_name
                ),
                PlayerDetailsMsg::ConfirmDelete,
                None,
            )
            .ok(),
            PlayerDetailsMsg::OpenNextPlayerMatch => self.id.map(|id| Effect::out(PlayerDetailsOut::OpenPlayerMatches(id))).unwrap_or_default().ok(),
            PlayerDetailsMsg::RequestClose => {
                if self.modified {
                    Effect::confirm(
                        "Lose Unsaved Changes?".to_owned(),
                        "There are unsaved changes made to this player".to_owned(),
                        PlayerDetailsMsg::Close,
                        None,
                    )
                    .ok()
                } else {
                    Effect::out(PlayerDetailsOut::Close).ok()
                }
            }
            PlayerDetailsMsg::Close => Effect::out(PlayerDetailsOut::Close).ok(),
        }
    }
}
