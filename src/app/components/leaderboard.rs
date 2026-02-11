use commander_tournament::{info::PlayerInfo, stats::PlayerStats};
use iced::{
    Padding,
    theme::default,
    widget::{container, row, scrollable, space, table, text},
};
use itertools::Itertools;

use crate::app::{App, Message, traits::View};

pub struct LeaderboardComponent;

#[derive(Clone)]
struct Player<'a> {
    id: u32,
    info: &'a PlayerInfo,
    stats: Option<&'a PlayerStats>,
}

impl View for LeaderboardComponent {
    fn view<'a>(app: &'a App) -> iced::Element<'a, Message> {
        let default_stats = app.tournament.create_default_stats();

        let players = app
            .tournament
            .players()
            .iter()
            .map(move |(id, info)| Player {
                id: *id,
                info,
                stats: app.tournament.get_player_stats(id),
            })
            .sorted_by(|a, b| {
                let a_stats = a.stats.unwrap_or(&default_stats);
                let b_stats = b.stats.unwrap_or(&default_stats);
                match a_stats.elo().total_cmp(&b_stats.elo()) {
                    std::cmp::Ordering::Equal => a.id.cmp(&b.id),
                    ord => ord,
                }
            })
            .rev();

        let tbl = table(
            [
                table::column("Deck", |p: Player<'_>| text(p.info.name()).size(12)),
                table::column("Elo", |p: Player<'_>| {
                    text(p.stats.unwrap_or(&default_stats).elo()).size(12)
                }),
                table::column("Games", |p: Player<'_>| {
                    text(p.stats.unwrap_or(&default_stats).games()).size(12)
                }),
                table::column("Wins", |p: Player<'_>| {
                    text(p.stats.unwrap_or(&default_stats).wins()).size(12)
                }),
                table::column("WR", |p: Player<'_>| {
                    text(
                        p.stats
                            .unwrap_or(&default_stats)
                            .wr()
                            .map(|wr| format!("{:.1}%", wr * 100.0))
                            .unwrap_or_default(),
                    )
                    .size(12)
                }),
            ],
            players,
        );

        container(scrollable(row![tbl, space().width(15)]))
            .padding(Padding::new(10f32))
            .into()
    }
}

