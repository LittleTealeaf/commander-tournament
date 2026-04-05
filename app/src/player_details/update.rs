use crate::{
    components::confirm::ConfirmDialog, effect::Effect, player_details::PlayerDetailsOut,
    traits::ComponentUpdate,
};

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
                Effect::done()
            }
            PlayerDetailsMsg::EditDescription(action) => {
                self.description.perform(action);
                Effect::done()
            }
            PlayerDetailsMsg::SetMoxfieldId(id) => {
                self.info.set_moxfield_id(id.clone());
                self.moxfield_id = id;
                Effect::done()
            }
            PlayerDetailsMsg::ToggleColor(mtg_color) => {
                self.info.toggle_color(mtg_color);
                Effect::done()
            }
            PlayerDetailsMsg::SetStatsTab(stats_tab) => {
                self.stats = stats_tab;
                Effect::done()
            }
            PlayerDetailsMsg::OpenLink(link) => Effect::out(PlayerDetailsOut::OpenLink(link)).ok(),
            PlayerDetailsMsg::ConfirmDelete => self
                .id
                .map(|id| Effect::out(PlayerDetailsOut::DeletePlayer(id)))
                .unwrap_or_default()
                .ok(),
            PlayerDetailsMsg::DeletePlayer => Effect::out(PlayerDetailsOut::ConfirmDialog(
                Box::new(ConfirmDialog::new(
                    format!("Delete {}", self.initial_name),
                    format!(
                        "Are you sure you want to delete {} and all games they were a part of?",
                        self.initial_name
                    ),
                    PlayerDetailsMsg::ConfirmDelete,
                    None,
                )),
            ))
            .ok(),
            PlayerDetailsMsg::OpenNextPlayerMatch => self
                .id
                .map(|id| Effect::out(PlayerDetailsOut::OpenPlayerMatches(id)))
                .unwrap_or_default()
                .ok(),
            PlayerDetailsMsg::Close => Effect::out(PlayerDetailsOut::Close).ok(),
        }
    }
}
