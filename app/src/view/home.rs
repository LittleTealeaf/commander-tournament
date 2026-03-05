use iced::widget::{button, column, row, space};

use crate::{
    App,
    logic::{Message, file::FileMessage},
    traits::{HandleMessage, View},
    view::{
        home::{
            leaderboard::LeaderboardColumn,
            matchmaker::{MatchMakerMessage, MatchMakerView},
            matchup::{MatchupMessage, MatchupView},
        },
        player::ViewPlayerMessage,
    },
};

mod leaderboard;
mod matchmaker;
mod matchup;

pub struct HomeState {
    leaderboard_sort_column: LeaderboardColumn,
    leaderboard_sort_asc: bool,
    matchup_view: MatchupView,
    matchmaker: MatchMakerView,
}

impl Default for HomeState {
    fn default() -> Self {
        Self {
            leaderboard_sort_column: LeaderboardColumn::Elo,
            leaderboard_sort_asc: false,
            matchup_view: MatchupView::default(),
            matchmaker: MatchMakerView::default(),
        }
    }
}

#[derive(Clone)]
pub enum HomeMessage {
    SortLeaderboardBy(LeaderboardColumn),
    MatchupMessage(MatchupMessage),
    MatchmakerMessage(MatchMakerMessage),
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

                Message::done()
            }
            HomeMessage::MatchupMessage(msg) => self.update(msg),
            HomeMessage::MatchmakerMessage(msg) => self.update(msg),
        }
    }
}

impl View<HomeState> for App {
    fn view<'a>(&'a self, _: &'a HomeState) -> iced::Element<'a, Message> {
        column![
            row![
                button("New Player").on_press(ViewPlayerMessage::Open(None).into()),
                space().width(15.0),
                button("Open").on_press(FileMessage::OpenFile.into()),
                button("Save").on_press(FileMessage::Save.into()),
                button("Save As")
                    .on_press_maybe(self.file.is_some().then_some(FileMessage::SaveAs.into())),
                button("New").on_press(FileMessage::New.into()),
            ],
            self.view_home_leaderboard()
        ]
        .into()
    }
}
