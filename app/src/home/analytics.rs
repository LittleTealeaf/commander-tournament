use edh_tourn::{analytics::aggregate::AggregateStats, player::color::ColorIdentity, tournament::Tournament};
use iced::{
    Length,
    widget::{checkbox, column, container, row, space, table, text},
};
use itertools::Itertools;

use crate::{
    components::scrollable_table,
    effect::Effect,
    fonts::FONT_ITALIC,
    icons::{color_icon, colorless_icon},
    traits::{Component, ComponentUpdate, ComponentView},
};

#[derive(Debug, Clone, Default)]
pub struct AnalyticsView {
    include_precons: bool,
}

#[derive(Clone, Debug)]
pub enum AnalyticsMsg {
    SetIncludePrecons(bool),
}

#[derive(Clone, Debug)]
pub enum AnalyticsOut {}

impl Component for AnalyticsView {
    type Message = AnalyticsMsg;
    type OutMessage = AnalyticsOut;
}

impl ComponentUpdate for AnalyticsView {
    type UpdateContext<'a> = ();
    fn update(
        &mut self,
        message: Self::Message,
        (): Self::UpdateContext<'_>,
    ) -> anyhow::Result<crate::effect::Effect<Self::Message, Self::OutMessage>> {
        match message {
            AnalyticsMsg::SetIncludePrecons(value) => {
                self.include_precons = value;
                Effect::done()
            }
        }
    }
}

impl ComponentView for AnalyticsView {
    type ViewContext<'a>
        = &'a Tournament
    where
        Self: 'a;

    fn view<'a>(&'a self, context: Self::ViewContext<'a>) -> iced::Element<'a, Self::Message> {
        let aggregated = context
            .analytics()
            .with_precons(self.include_precons)
            .aggregated_identity_stats();
        let missing = ColorIdentity::IDENTITIES
            .into_iter()
            .filter(|identity| !aggregated.contains_key(identity))
            .join(", ");

        let identities = aggregated.into_iter().sorted_by(|(ida, a), (idb, b)| {
            a.avg_elo()
                .unwrap_or(0f64)
                .total_cmp(&b.avg_elo().unwrap_or(0f64))
                .then_with(|| ida.cmp(idb))
                .reverse()
        });

        container(
            column![
                row![
                    text("Aggregated by Color Identity").size(18),
                    space().width(Length::Fill),
                    checkbox(self.include_precons)
                        .label("Include Precons")
                        .on_toggle(AnalyticsMsg::SetIncludePrecons)
                ],
                scrollable_table(table::table(
                    [
                        table::column("Identity", |(identity, _): (ColorIdentity, AggregateStats)| {
                            row![
                                text(format!("{identity}")),
                                space().width(Length::Fill),
                                if identity.is_colorless() {
                                    row![colorless_icon()]
                                } else {
                                    row(identity.colors().map(color_icon).map(Into::into)).spacing(3)
                                }
                            ]
                        }),
                        table::column("Decks", |(_, stats): (ColorIdentity, AggregateStats)| {
                            text(format!("{}", stats.count()))
                        }),
                        table::column("Games", |(_, stats): (ColorIdentity, AggregateStats)| {
                            text(format!("{}", stats.games()))
                        }),
                        table::column("Avg. Elo", |(_, stats): (ColorIdentity, AggregateStats)| {
                            stats
                                .avg_elo()
                                .map_or_else(|| text("-"), |avg_elo| text(format!("{}", avg_elo.round())))
                        }),
                        table::column("Winrate", |(_, stats): (ColorIdentity, AggregateStats)| {
                            stats
                                .wr()
                                .map_or_else(|| text("-"), |wr| text(format!("{}%", (wr * 100.0).round())))
                        }),
                    ],
                    identities,
                ))
                .height(Length::Fill),
                (!missing.is_empty()).then(|| container(
                    text(format!("Missing: {missing}")).font(FONT_ITALIC).size(14)
                )
                .padding(10))
            ]
            .spacing(10),
        )
        .padding(25)
        .into()
    }
}
