use crate::{
    components::prompt::{self, DialogPrompt},
    core::{message::Message, tournament::TournamentAction},
    player_details::PlayerDetailsOut,
    traits::{ComponentUpdate, Effect},
};

use super::PlayerDetailsMsg;

impl From<prompt::Message> for super::PlayerDetailsMsg {
    fn from(value: prompt::Message) -> Self {
        Self::Dialog(value)
    }
}

impl ComponentUpdate for super::PlayerDetails {
    type UpdateContext<'a> = ();
    fn update(
        &mut self,
        message: Self::Message,
        (): Self::UpdateContext<'_>,
    ) -> anyhow::Result<crate::traits::Effect<Self::Message, Self::OutMessage>> {
        match message {
            PlayerDetailsMsg::SelectPlayerReference(id) => {
                if Some(id) == self.id {
                    Effect::done()
                } else {
                    Effect::global(Message::OpenPlayerDetails(Some(id)))
                }
            }
            PlayerDetailsMsg::SaveAndClose => {
                self.info.set_description(self.description.text());
                Effect::global(match self.id {
                    Some(id) => TournamentAction::SetPlayerInfo(id, self.info.clone()),
                    None => TournamentAction::Register(self.info.clone()),
                })?
                .then(Effect::out(PlayerDetailsOut::Close))
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
            PlayerDetailsMsg::OpenLink(link) => Effect::global(Message::OpenLink(link)),
            PlayerDetailsMsg::Close => Effect::out(PlayerDetailsOut::Close),
            PlayerDetailsMsg::Dialog(message) => {
                if let Some(dialog) = &mut self.prompt_confirm_delete {
                    return dialog.update(message, ())?.map(|message| match message {
                        crate::components::prompt::Message::Accept => {
                            self.id.map_or_else(Effect::done, |id| {
                                Effect::global(TournamentAction::DeletePlayer(id))?
                                    .then(Effect::out(PlayerDetailsOut::Close))
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
            PlayerDetailsMsg::DeletePlayer => {
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
