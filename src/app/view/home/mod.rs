use iced::{Element, Task};

use crate::app::{
    App,
    message::Message,
    traits::HandleMessage,
    view::home::leaderboard::{LeaderboardColumn, view_leaderboard},
};

pub mod leaderboard;

pub struct AppHome {
    leaderboard_sort_column: LeaderboardColumn,
    leaderboard_sort_asc: bool,
}

impl Default for AppHome {
    fn default() -> Self {
        Self {
            leaderboard_sort_asc: false,
            leaderboard_sort_column: LeaderboardColumn::Elo,
        }
    }
}

#[derive(Clone)]
pub enum HomeMessage {
    SortLeaderboardBy(LeaderboardColumn),
}

impl From<HomeMessage> for Message {
    fn from(value: HomeMessage) -> Self {
        Self::HomeAction(value)
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
        }
    }
}

pub fn view_home(app: &App) -> Element<'_, Message> {
    view_leaderboard(app)
}
