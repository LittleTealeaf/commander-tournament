use core::cmp::Ordering;

use edh_tourn::{
    player::{PlayerId, RegisteredPlayer},
    tournament::Tournament,
};
use iced::{
    Padding,
    widget::{button, container, row, scrollable, space, table, text},
};
use itertools::Itertools;
use nerd_font_symbols::md::{MD_ARROW_DOWN, MD_ARROW_UP, MD_SWORD_CROSS};

use crate::{
    effect::Effect,
    traits::{Component, ComponentUpdate, ComponentView},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum Column {
    Name,
    #[default]
    Elo,
    Games,
    Wins,
    WinRate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SortDirection {
    Ascending,
    #[default]
    Descending,
}

impl SortDirection {
    #[must_use]
    pub const fn reverse(self) -> Self {
        match self {
            Self::Ascending => Self::Descending,
            Self::Descending => Self::Ascending,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Leaderboard {
    column: Column,
    direction: SortDirection,
}

#[derive(Debug, Clone)]
pub enum LeaderboardMsg {
    Sort(Column),
    NewPlayer,
    OpenPlayer(PlayerId),
    RankPlayer(PlayerId),
}

#[derive(Debug, Clone)]
pub enum LeaderboardOut {
    RankPlayer(PlayerId),
    OpenPlayerDetails(PlayerId),
    OpenNewPlayer,
}

impl Leaderboard {
    fn sort_players<'a>(
        &self,
        players: impl IntoIterator<Item = RegisteredPlayer<'a>>,
    ) -> impl Iterator<Item = RegisteredPlayer<'a>> {
        players.into_iter().sorted_by(|a, b| {
            let cmp = match self.column {
                Column::Name => a.info().name().cmp(b.info().name()),
                Column::Elo => a.stats().elo().total_cmp(&b.stats().elo()),
                Column::Wins => a.stats().wins().cmp(&b.stats().wins()),
                Column::WinRate => a
                    .stats()
                    .wr()
                    .partial_cmp(&b.stats().wr())
                    .unwrap_or(Ordering::Equal),
                Column::Games => a.stats().games().cmp(&b.stats().games()),
            }
            .then_with(|| a.id().cmp(&b.id()));

            match self.direction {
                SortDirection::Ascending => cmp,
                SortDirection::Descending => cmp.reverse(),
            }
        })
    }
}

impl Component for Leaderboard {
    type Message = LeaderboardMsg;
    type OutMessage = LeaderboardOut;
}

impl ComponentUpdate for Leaderboard {
    type UpdateContext<'a> = ();
    fn update(
        &mut self,
        message: Self::Message,
        (): Self::UpdateContext<'_>,
    ) -> anyhow::Result<Effect<Self::Message, Self::OutMessage>> {
        match message {
            LeaderboardMsg::OpenPlayer(id) => {
                Effect::out(LeaderboardOut::OpenPlayerDetails(id)).ok()
            }
            LeaderboardMsg::NewPlayer => Effect::out(LeaderboardOut::OpenNewPlayer).ok(),
            LeaderboardMsg::RankPlayer(id) => Effect::Out(LeaderboardOut::RankPlayer(id)).ok(),
            LeaderboardMsg::Sort(column) => {
                if self.column == column {
                    self.direction = self.direction.reverse();
                } else {
                    self.column = column;
                    self.direction = if column == Column::Name {
                        SortDirection::Ascending
                    } else {
                        SortDirection::Descending
                    };
                }
                Effect::done()
            }
        }
    }
}

impl ComponentView for Leaderboard {
    type ViewContext<'a>
        = &'a Tournament
    where
        Self: 'a;

    fn view<'a>(&'a self, context: Self::ViewContext<'a>) -> iced::Element<'a, Self::Message> {
        let players = self.sort_players(context.get_registered_players());

        let ord_char = match self.direction {
            SortDirection::Ascending => MD_ARROW_UP,
            SortDirection::Descending => MD_ARROW_DOWN,
        };

        let col_header = |label: &str, col: Column| {
            button(text(if self.column == col {
                format!("{label} {ord_char}")
            } else {
                format!("{label}  ")
            }))
            .style(button::text)
            .on_press(LeaderboardMsg::Sort(col))
        };

        let tbl = table(
            [
                table::column(
                    col_header("Name", Column::Name),
                    |p: RegisteredPlayer<'_>| {
                        button(text(p.info().display_name()).size(12))
                            .style(button::text)
                            .on_press(LeaderboardMsg::OpenPlayer(p.id()))
                    },
                ),
                table::column(col_header("Elo", Column::Elo), |p: RegisteredPlayer<'_>| {
                    text(format!("{:.0}", p.stats().elo())).size(12)
                }),
                table::column(
                    col_header("Games", Column::Games),
                    |p: RegisteredPlayer<'_>| text(p.stats().games()).size(12),
                ),
                table::column(
                    col_header("Wins", Column::Wins),
                    |p: RegisteredPlayer<'_>| text(p.stats().wins()).size(12),
                ),
                table::column(
                    col_header("WR", Column::WinRate),
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
                    button("+").on_press(LeaderboardMsg::NewPlayer),
                    |p: RegisteredPlayer<'_>| {
                        button(MD_SWORD_CROSS)
                            .style(button::text)
                            .on_press(LeaderboardMsg::RankPlayer(p.id()))
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
