use std::cmp::Ordering;

use commander_tournament::{info::PlayerInfo, stats::PlayerStats};
use iced::{
    Element, Padding,
    widget::{button, container, row, scrollable, space, table, text},
};
use itertools::Itertools;

use crate::app::{App, Message, view::home::HomeMessage};

#[derive(Clone)]
struct Player<'a> {
    id: u32,
    info: &'a PlayerInfo,
    stats: Option<&'a PlayerStats>,
}

impl<'a> Player<'a> {
    fn get_stats<'b>(&'b self, default: &'b PlayerStats) -> &'b PlayerStats {
        self.stats.unwrap_or(default)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LeaderboardColumn {
    Name,
    Elo,
    WR,
    Games,
    Wins,
}

pub fn view_leaderboard<'a>(app: &'a App) -> Element<'a, Message> {
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
            let sort = match app.home.leaderboard_sort_column {
                LeaderboardColumn::Name => a.info.name().cmp(b.info.name()),
                LeaderboardColumn::Elo => a
                    .get_stats(&default_stats)
                    .elo()
                    .total_cmp(&b.get_stats(&default_stats).elo()),
                LeaderboardColumn::WR => a
                    .get_stats(&default_stats)
                    .wr()
                    .partial_cmp(&b.get_stats(&default_stats).wr())
                    .unwrap_or(Ordering::Equal),
                LeaderboardColumn::Games => a
                    .get_stats(&default_stats)
                    .games()
                    .cmp(&b.get_stats(&default_stats).games()),
                LeaderboardColumn::Wins => a
                    .get_stats(&default_stats)
                    .wins()
                    .cmp(&b.get_stats(&default_stats).wins()),
            };
            if app.home.leaderboard_sort_asc {
                sort
            } else {
                sort.reverse()
            }
        });

    let ord_char = if app.home.leaderboard_sort_asc {
        ""
    } else {
        ""
    };

    let col_header = |label: &str, col: LeaderboardColumn| {
        button(text(if app.home.leaderboard_sort_column == col {
            format!("{label} {ord_char}")
        } else {
            format!("{label}  ")
        }))
        .style(button::text)
        .on_press(HomeMessage::SortLeaderboardBy(col).into())
    };

    let tbl = table(
        [
            table::column(
                col_header("Name", LeaderboardColumn::Name),
                |p: Player<'_>| {
                    button(text(p.info.name()).size(12))
                        .style(button::text)
                        .on_press_maybe(p.info.moxfield_link().map(Message::OpenLink))
                },
            ),
            table::column(
                col_header("Elo", LeaderboardColumn::Elo),
                |p: Player<'_>| {
                    text(format!("{:.0}", p.stats.unwrap_or(&default_stats).elo())).size(12)
                },
            ),
            table::column(
                col_header("Games", LeaderboardColumn::Games),
                |p: Player<'_>| text(p.stats.unwrap_or(&default_stats).games()).size(12),
            ),
            table::column(
                col_header("Wins", LeaderboardColumn::Wins),
                |p: Player<'_>| text(p.stats.unwrap_or(&default_stats).wins()).size(12),
            ),
            table::column(col_header("WR", LeaderboardColumn::WR), |p: Player<'_>| {
                text(
                    p.stats
                        .unwrap_or(&default_stats)
                        .wr()
                        .map(|wr| format!("{:.1}%", wr * 100.0))
                        .unwrap_or_default(),
                )
                .size(12)
            }),
            table::column("", |p: Player<'_>| {
                button("").on_press(Message::EditPlayer(Some(p.id)))
            }),
        ],
        players,
    );

    container(scrollable(row![tbl, space().width(15)]))
        .padding(Padding::new(10f32))
        .into()
}
