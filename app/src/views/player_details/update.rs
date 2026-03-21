use opener::open_browser;

use crate::{
    traits::{ComponentUpdate, Effect},
    views::player_details::OutMessage,
};

use super::Message;

impl ComponentUpdate for super::State {
    fn update(
        &mut self,
        message: Self::Message,
        _: Self::Context<'_>,
    ) -> anyhow::Result<crate::traits::Effect<Self::Message, Self::OutMessage>> {
        match message {
            Message::SelectPlayerReference(id) => {
                if Some(id) == self.id {
                    Effect::ok()
                } else {
                    Effect::out(OutMessage::OpenPlayer(id))
                }
            }
            Message::SaveAndClose => {
                self.info.set_description(self.description.text());
                Effect::out(OutMessage::SaveAndClose(self.id, self.info.clone()))
            }
            Message::SetName(name) => {
                self.info.set_name(name);
                Effect::ok()
            }
            Message::EditDescription(action) => {
                self.description.perform(action);
                Effect::ok()
            }
            Message::SetMoxfieldId(id) => {
                self.info.set_moxfield_id(id.clone());
                self.moxfield_id = id;
                Effect::ok()
            }
            Message::ToggleColor(mtg_color) => {
                self.info.toggle_color(mtg_color);
                Effect::ok()
            }
            Message::SetStatsTab(stats_tab) => {
                self.stats = stats_tab;
                Effect::ok()
            }
            Message::OpenLink(link) => {
                open_browser(link)?;
                Effect::ok()
            }
            Message::Close => Effect::out(OutMessage::Close),
        }
    }
}
