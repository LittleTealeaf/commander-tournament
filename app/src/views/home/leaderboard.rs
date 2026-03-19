use core::cmp::Ordering;

use edh_tourn::{player::RegisteredPlayer, tournament::Tournament};
use itertools::Itertools;

use crate::traits::{Component, Effect};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Column {
    Name,
    Elo,
    Games,
    Wins,
    WinRate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortDirection {
    Ascending,
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

#[derive(Debug, Clone)]
pub struct State {
    column: Column,
    direction: SortDirection,
}

#[derive(Debug, Clone)]
pub enum Message {
    Sort(Column),
    NewPlayer,
    OpenPlayer(u32),
    RankPlayer(u32),
}

#[derive(Debug, Clone)]
pub enum OutMessage {
    OpenPlayerDetails(Option<u32>),
    RankPlayer(u32),
}

impl State {
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

impl Component for State {
    type Message = Message;
    type OutMessage = OutMessage;
    type Context<'a> = &'a Tournament;

    fn update(
        &mut self,
        message: Self::Message,
        _: &Tournament,
    ) -> anyhow::Result<Effect<Self::Message, Self::OutMessage>>{
        match message {
            Message::OpenPlayer(id) => Effect::out(OutMessage::OpenPlayerDetails(Some(id))),
            Message::NewPlayer => Effect::out(OutMessage::OpenPlayerDetails(None)),
            Message::RankPlayer(id) => Effect::out(OutMessage::RankPlayer(id)),
            Message::Sort(column) => {
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
                Effect::ok()
            }
        }
    }
    fn view(&self, context: &Tournament) -> iced::Element<'_, Self::Message> {
        todo!()
    }
}
