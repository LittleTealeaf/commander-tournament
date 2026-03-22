use opener::open_browser;

use crate::{
    components::prompt::{self, DialogPrompt},
    traits::{ComponentUpdate, Effect},
    views::player_details::OutMessage,
};

use super::Message;

impl From<prompt::Message> for super::Message {
    fn from(value: prompt::Message) -> Self {
        Self::Dialog(value)
    }
}

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
            Message::Dialog(message) => {
                if let Some(dialog) = &mut self.prompt_confirm_delete {
                    return dialog.update(message, ())?.map(|message| match message {
                        crate::components::prompt::Message::Accept => {
                            self.id.map_or_else(Effect::ok, |id| {
                                Effect::out(OutMessage::DeletePlayer(id))
                            })
                        }
                        crate::components::prompt::Message::Cancel => {
                            self.prompt_confirm_delete = None;
                            Effect::ok()
                        }
                    });
                }
                Effect::ok()
            }
            Message::DeletePlayer => {
                self.prompt_confirm_delete = Some(DialogPrompt::new(
                    format!("Delete {}?", self.initial_name),
                    format!(
                        "Are you sure you want to delete {} and all games they were a part of?",
                        self.initial_name
                    ),
                ));
                Effect::ok()
            }
        }
    }
}
