use std::collections::HashSet;

use edh_tourn::{
    player::{PlayerId, RegisteredPlayer},
    tournament::Tournament,
};
use itertools::Itertools;

use crate::{
    components::scrollable_table,
    effect::Effect,
    icons::ToColorIcons,
    popup::Popup,
    traits::{Component, ComponentUpdate},
};

use iced::{
    Length,
    alignment::Vertical,
    widget::{button, checkbox, row, space, table, text},
};

#[derive(Debug, Clone, Default)]
pub struct FilterPlayersComponent {
    removed: HashSet<PlayerId>,
}

impl FilterPlayersComponent {
    pub fn new<I>(filtered: I) -> Self
    where
        I: IntoIterator<Item = PlayerId>,
    {
        Self {
            removed: filtered.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum FilterPlayersMsg {
    Filter(PlayerId, bool),
    FilterAll(bool),
    ToggleAll,
    Submit,
    Cancel,
}

#[derive(Debug, Clone)]
pub enum FilterPlayersOut {
    Submit(im::HashSet<PlayerId>),
    Cancel,
}

impl Component for FilterPlayersComponent {
    type Message = FilterPlayersMsg;
    type OutMessage = FilterPlayersOut;
}

impl FilterPlayersComponent {
    #[must_use]
    pub fn to_popup<'a>(&'a self, tourn: &'a Tournament) -> Popup<'a, FilterPlayersMsg> {
        Popup::new(
            "Filter Players",
            scrollable_table(table(
                [
                    // table::column("", |player: RegisteredPlayer<'_>| {
                    // }),
                    table::column("Name", |player: RegisteredPlayer<'_>| {
                        row![
                            checkbox(!self.removed.contains(&player.id()))
                                .label(player.info().display_name())
                                .on_toggle(move |val| FilterPlayersMsg::Filter(player.id(), val))
                                .width(Length::Shrink),
                            space().width(Length::Fill),
                            row(player.info().color_identity().to_icons().map(Into::into))
                                .spacing(3)
                                .align_y(Vertical::Center)
                        ]
                        .spacing(3)
                        .align_y(Vertical::Center)
                    }),
                    table::column("Elo", |p: RegisteredPlayer<'_>| {
                        text(format!("{:.0}", p.stats().elo())).size(12)
                    }),
                ],
                tourn
                    .registered_players()
                    .sorted_by(|a, b| a.stats().elo().total_cmp(&b.stats().elo()).reverse()),
            ))
            .into(),
            vec![
                button("Toggle").on_press(FilterPlayersMsg::ToggleAll).into(),
                if self.removed.is_empty() {
                    button("Clear All").on_press(FilterPlayersMsg::FilterAll(false))
                } else {
                    button("Select All").on_press(FilterPlayersMsg::FilterAll(true))
                }
                .into(),
                space().width(Length::Fill).into(),
                button("Submit").on_press(FilterPlayersMsg::Submit).into(),
                button("Cancel").on_press(FilterPlayersMsg::Cancel).into(),
            ],
        )
    }
}

impl ComponentUpdate for FilterPlayersComponent {
    type UpdateContext<'a> = &'a Tournament;

    fn update(
        &mut self,
        message: Self::Message,
        context: Self::UpdateContext<'_>,
    ) -> anyhow::Result<crate::effect::Effect<Self::Message, Self::OutMessage>> {
        match message {
            FilterPlayersMsg::ToggleAll => {
                self.removed = context
                    .players()
                    .keys()
                    .copied()
                    .collect::<HashSet<_>>()
                    .difference(&self.removed)
                    .copied()
                    .collect();
                Effect::done()
            }
            FilterPlayersMsg::FilterAll(val) => {
                if val {
                    self.removed.clear();
                } else {
                    self.removed = context.players().keys().copied().collect();
                }
                Effect::done()
            }
            FilterPlayersMsg::Filter(player_id, val) => {
                context.require_id_registered(player_id)?;
                if val {
                    self.removed.remove(&player_id);
                } else {
                    self.removed.insert(player_id);
                }
                Effect::done()
            }
            FilterPlayersMsg::Submit => {
                Effect::out(FilterPlayersOut::Submit(self.removed.iter().copied().collect())).ok()
            }
            FilterPlayersMsg::Cancel => Effect::out(FilterPlayersOut::Cancel).ok(),
        }
    }
}
