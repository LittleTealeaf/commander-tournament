use core::fmt::Write;

use edh_tourn::{analytics::aggregate::AggregateStats, player::color::ColorIdentity, tournament::Tournament};
use iced::{
    Length,
    widget::{column, container, row, space, table, text},
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
pub struct StatsReview;

#[derive(Clone, Debug)]
pub enum StatsReviewMsg {}

#[derive(Clone, Debug)]
pub enum StatsReviewOut {}

impl Component for StatsReview {
    type Message = StatsReviewMsg;
    type OutMessage = StatsReviewOut;
}

impl ComponentUpdate for StatsReview {
    type UpdateContext<'a> = ();
    fn update(
        &mut self,
        _: Self::Message,
        (): Self::UpdateContext<'_>,
    ) -> anyhow::Result<crate::effect::Effect<Self::Message, Self::OutMessage>> {
        Effect::done()
    }
}

impl ComponentView for StatsReview {
    type ViewContext<'a>
        = &'a Tournament
    where
        Self: 'a;

    fn view<'a>(&'a self, context: Self::ViewContext<'a>) -> iced::Element<'a, Self::Message> {
        let aggregated = context.analytics().aggregated_identity_stats();
        let missing = ColorIdentity::IDENTITIES
            .into_iter()
            .filter(|identity| !aggregated.keys().contains(identity))
            .collect::<Vec<_>>();

        let identities = aggregated.into_iter().sorted_by(|(ida, a), (idb, b)| {
            a.avg_elo()
                .unwrap_or(0f64)
                .total_cmp(&b.avg_elo().unwrap_or(0f64))
                .then_with(|| ida.cmp(idb))
                .reverse()
        });

        container(
            column![
                text("Aggregated by Color Identity").size(18),
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
                )),
                container(
                    text(missing.into_iter().enumerate().fold(
                        String::from("No Decks: "),
                        |mut acc, (i, item)| {
                            if i > 0 {
                                let _ = write!(acc, ", ");
                            }
                            let _ = write!(acc, "{item}");
                            acc
                        }
                    ))
                    .size(13)
                    .font(FONT_ITALIC)
                )
                .padding(10)
            ]
            .spacing(10),
        )
        .padding(25)
        .height(Length::Shrink)
        .into()
    }
}
