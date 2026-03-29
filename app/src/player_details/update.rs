use crate::{
    components::prompt::{self, PromptComponent, PromptMessage},
    effect::Effect,
    player_details::PlayerDetailsOut,
    traits::ComponentUpdate,
};

use super::{PlayerDetails, PlayerDetailsMsg};

impl From<prompt::PromptMessage> for PlayerDetailsMsg {
    fn from(value: prompt::PromptMessage) -> Self {
        Self::Dialog(value)
    }
}

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
                Effect::out(PlayerDetailsOut::Save(self.id, self.info.clone()))
                    .chain(Effect::Out(PlayerDetailsOut::Close))
                    .ok()
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
            PlayerDetailsMsg::Close => Effect::Out(PlayerDetailsOut::Close).ok(),
            PlayerDetailsMsg::Dialog(message) => {
                if let Some(dialog) = &mut self.prompt_confirm_delete {
                    return dialog.mapped_update(message, (), |message| match message {
                        PromptMessage::Accept => self
                            .id
                            .map(|id| Effect::out(PlayerDetailsOut::DeletePlayer(id)))
                            .unwrap_or_default()
                            .chain(Effect::Out(PlayerDetailsOut::Close))
                            .ok(),
                        PromptMessage::Cancel => Effect::msg(PlayerDetailsMsg::CancelDelete).ok(),
                    });
                }
                Effect::done()
            }
            PlayerDetailsMsg::CancelDelete => {
                self.prompt_confirm_delete = None;
                Effect::done()
            }
            PlayerDetailsMsg::DeletePlayer => {
                self.prompt_confirm_delete = Some(PromptComponent::new(
                    format!("Delete {}?", self.initial_name),
                    format!(
                        "Are you sure you want to delete {} and all games they were a part of?",
                        self.initial_name
                    ),
                ));
                Effect::done()
            }
        }
    }
}
