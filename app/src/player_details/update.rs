use iced::Task;

use crate::{
    components::prompt::{self, DialogPrompt},
    player_details::OutMessage,
    services::system::open_link,
    traits::{ComponentUpdate, Effect},
};

use super::Message;

impl From<prompt::Message> for super::Message {
    fn from(value: prompt::Message) -> Self {
        Self::Dialog(value)
    }
}

impl ComponentUpdate for super::State {
    type UpdateContext<'a> = ();
    fn update(
        &mut self,
        message: Self::Message,
        (): Self::UpdateContext<'_>,
    ) -> anyhow::Result<crate::traits::Effect<Self::Message, Self::OutMessage>> {
        match message {
            Message::SelectPlayerReference(id) => {
                if Some(id) == self.id {
                    Effect::done()
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
                Effect::done()
            }
            Message::EditDescription(action) => {
                self.description.perform(action);
                Effect::done()
            }
            Message::SetMoxfieldId(id) => {
                self.info.set_moxfield_id(id.clone());
                self.moxfield_id = id;
                Effect::done()
            }
            Message::ToggleColor(mtg_color) => {
                self.info.toggle_color(mtg_color);
                Effect::done()
            }
            Message::SetStatsTab(stats_tab) => {
                self.stats = stats_tab;
                Effect::done()
            }
            Message::OpenLink(link) => Effect::task(Task::future(async {
                let _ = open_link(link).await;
                Message::Nothing
            })),
            Message::Nothing => Effect::done(),
            Message::Close => Effect::out(OutMessage::Close),
            Message::Dialog(message) => {
                if let Some(dialog) = &mut self.prompt_confirm_delete {
                    return dialog.update(message, ())?.map(|message| match message {
                        crate::components::prompt::Message::Accept => {
                            self.id.map_or_else(Effect::done, |id| {
                                Effect::out(OutMessage::DeletePlayer(id))
                            })
                        }
                        crate::components::prompt::Message::Cancel => {
                            self.prompt_confirm_delete = None;
                            Effect::done()
                        }
                    });
                }
                Effect::done()
            }
            Message::DeletePlayer => {
                self.prompt_confirm_delete = Some(DialogPrompt::new(
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
