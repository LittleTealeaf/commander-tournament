use iced::{
    Element, Padding,
    widget::{button, container, row, scrollable, space, table, text},
};
use itertools::Itertools;

use crate::{
    tournament::PlayerStats,
    ui::{Message, TournamentApp},
};

pub fn leaderboard(app: &TournamentApp) -> Element<'_, Message> {
    #[derive(Clone)]
    struct Player<'a> {
        name: &'a String,
        stats: &'a PlayerStats,
    }

    let table_widget = table(
        [
            table::column("", |p: Player<'_>| button("+").on_press(Message::AddPlayerToNextSlot(p.name.clone()))),
            table::column("Deck", |p: Player<'_>| text(p.name).size(12)),
            table::column("Elo", |p: Player<'_>| {
                text(format!("{:.0}", p.stats.elo())).size(12)
            }),
            table::column("Games", |p: Player<'_>| text(p.stats.games()).size(12)),
            table::column("Wins", |p: Player<'_>| text(p.stats.wins()).size(12)),
            table::column("WR", |p: Player<'_>| {
                text(
                    p.stats
                        .wr()
                        .map(|wr| format!("{:.1}%", wr * 100.0))
                        .unwrap_or_default(),
                )
                .size(12)
            }),
            table::column("Edit", |p: Player<'_>| {
                button(text("✏").size(14))
                    .on_press_with(|| {
                        Message::SetChangePlayerName(Some((Some(p.name.clone()), p.name.clone())))
                    })
                    .padding(4)
                    .width(iced::Length::Fixed(28.0))
            }),
        ],
        app.tournament
            .players()
            .iter()
            .map(|(name, stats)| Player { name, stats })
            .sorted_by(|a, b| a.stats.elo().total_cmp(&b.stats.elo()))
            .rev(),
    );

    container(scrollable(row![table_widget, space().width(15)]))
        .padding(Padding::new(10f32))
        .into()
}
