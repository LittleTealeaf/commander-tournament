use core::cmp::Ordering;

use edh_tourn::player::RegisteredPlayer;
use iced::{
    Element, Padding,
    widget::{button, container, row, scrollable, space, table, text},
};
use itertools::Itertools;
use nerd_font_symbols::md::{MD_ARROW_DOWN, MD_ARROW_UP, MD_PLAYLIST_PLUS};

use crate::{
    App,
    logic::Message,
    view::{
        home::{HomeMessage, matchmaker::MatchMakerMessage},
        player::ViewPlayerMessage,
    },
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LeaderboardColumn {
    Name,
    Elo,
    WR,
    Games,
    Wins,
}

impl App {
    fn get_sorted_players(&self) -> impl Iterator<Item = RegisteredPlayer<'_>> {
        self.tournament.get_registered_players().sorted_by(|a, b| {
            let sort = match self.home.leaderboard_sort_column {
                LeaderboardColumn::Name => a.info().name().cmp(b.info().name()),
                LeaderboardColumn::Elo => a.stats().elo().total_cmp(&b.stats().elo()),
                LeaderboardColumn::WR => a
                    .stats()
                    .wr()
                    .partial_cmp(&b.stats().wr())
                    .unwrap_or(Ordering::Equal),
                LeaderboardColumn::Games => a.stats().games().cmp(&b.stats().games()),
                LeaderboardColumn::Wins => a.stats().wins().cmp(&b.stats().wins()),
            };
            if self.home.leaderboard_sort_asc {
                sort
            } else {
                sort.reverse()
            }
        })
    }

    #[must_use]
    pub fn view_home_leaderboard(&self) -> Element<'_, Message> {
        let players = self.get_sorted_players();

        let ord_char = if self.home.leaderboard_sort_asc {
            MD_ARROW_DOWN
        } else {
            MD_ARROW_UP
        };

        let col_header = |label: &str, col: LeaderboardColumn| {
            button(text(if self.home.leaderboard_sort_column == col {
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
                    |p: RegisteredPlayer<'_>| {
                        button(text(p.info().name().clone()).size(12))
                            .style(button::text)
                            .on_press(ViewPlayerMessage::Open(Some(p.id())).into())
                    },
                ),
                table::column(
                    col_header("Elo", LeaderboardColumn::Elo),
                    |p: RegisteredPlayer<'_>| text(format!("{:.0}", p.stats().elo())).size(12),
                ),
                table::column(
                    col_header("Games", LeaderboardColumn::Games),
                    |p: RegisteredPlayer<'_>| text(p.stats().games()).size(12),
                ),
                table::column(
                    col_header("Wins", LeaderboardColumn::Wins),
                    |p: RegisteredPlayer<'_>| text(p.stats().wins()).size(12),
                ),
                table::column(
                    col_header("WR", LeaderboardColumn::WR),
                    |p: RegisteredPlayer<'_>| {
                        text(
                            p.stats()
                                .wr()
                                .map(|wr| format!("{:.1}%", wr * 100.0))
                                .unwrap_or_default(),
                        )
                        .size(12)
                    },
                ),
                table::column(
                    button("+").on_press(ViewPlayerMessage::Open(None).into()),
                    |p: RegisteredPlayer<'_>| {
                        button(MD_PLAYLIST_PLUS)
                            .style(button::text)
                            .on_press(MatchMakerMessage::Player(Some(p.id())).into())
                    },
                ),
            ],
            players,
        );

        container(scrollable(row![tbl, space().width(15)]))
            .padding(Padding::new(10f32))
            .into()
    }
}
