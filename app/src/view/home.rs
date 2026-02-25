use iced::{Task, widget::row};

use crate::{
    App,
    logic::Message,
    traits::{HandleMessage, ViewApp},
    view::home::{
        leaderboard::LeaderboardColumn,
        matchup::{MatchupMessage, MatchupView},
    },
};

mod leaderboard;
mod matchup;

pub struct HomeState {
    leaderboard_sort_column: LeaderboardColumn,
    leaderboard_sort_asc: bool,
    matchup_view: MatchupView,
}

impl Default for HomeState {
    fn default() -> Self {
        Self {
            leaderboard_sort_column: LeaderboardColumn::Elo,
            leaderboard_sort_asc: false,
            matchup_view: MatchupView::default(),
        }
    }
}

#[derive(Clone)]
pub enum HomeMessage {
    SortLeaderboardBy(LeaderboardColumn),
    MatchupMessage(MatchupMessage),
}

impl From<HomeMessage> for Message {
    fn from(value: HomeMessage) -> Self {
        Self::Home(value)
    }
}

impl HandleMessage<HomeMessage> for App {
    fn update(&mut self, msg: HomeMessage) -> anyhow::Result<iced::Task<Message>> {
        match msg {
            HomeMessage::SortLeaderboardBy(sort_column) => {
                if self.home.leaderboard_sort_column.eq(&sort_column) {
                    self.home.leaderboard_sort_asc = !self.home.leaderboard_sort_asc;
                } else {
                    self.home.leaderboard_sort_column = sort_column;
                    self.home.leaderboard_sort_asc = matches!(sort_column, LeaderboardColumn::Name);
                }

                Ok(Task::none())
            }
            HomeMessage::MatchupMessage(msg) => self.update(msg),
        }
    }
}

impl ViewApp for HomeState {
    fn view(app: &App) -> iced::Element<'_, Message> {
        row![app.view_home_leaderboard()].into()
    }
}
