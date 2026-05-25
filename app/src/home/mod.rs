use std::path::PathBuf;

use edh_tourn::{game::record::GameRecord, player::PlayerId, tournament::Tournament};
use iced::{
    Length,
    widget::{column, container, responsive, row, text},
};

use crate::{
    components::tab_bar,
    effect::Effect,
    home::leaderboard::{Leaderboard, LeaderboardMsg, LeaderboardOut},
    traits::{Component, ComponentUpdate, ComponentView},
    views::play::PlayMode,
};

pub mod leaderboard;

const SCREEN_WIDTH_BREAKPOINT: f32 = 1250.0;

#[derive(Debug, Default)]
pub struct Home {
    leaderboard: Leaderboard,
    tab: HomeTab,
}

#[derive(Debug, Clone, Copy, derive_more::Display, Default, PartialEq, Eq)]
pub enum HomeTab {
    #[default]
    Leaderboard,
    PlayGame,
}

#[derive(Debug, Clone, derive_more::From)]
pub enum HomeMsg {
    Leaderboard(LeaderboardMsg),
    SetTab(HomeTab),
}

#[derive(Debug, Clone, derive_more::From)]
pub enum HomeOut {
    #[from(skip)]
    OpenPlayView(PlayMode),
    RecordGame(Box<GameRecord>),
    OpenPlayerDetails(PlayerId),
    OpenNewPlayer,
}

impl Component for Home {
    type Message = HomeMsg;
    type OutMessage = HomeOut;
}

impl ComponentView for Home {
    type ViewContext<'a>
        = &'a Tournament
    where
        Self: 'a;
    fn view<'a>(&'a self, context: Self::ViewContext<'a>) -> iced::Element<'a, Self::Message> {
        responsive(|size: iced::Size| match size.width {
            ..=SCREEN_WIDTH_BREAKPOINT => column![
                tab_bar(
                    &self.tab,
                    [HomeTab::Leaderboard, HomeTab::PlayGame],
                    HomeMsg::SetTab
                ),
                container(match self.tab {
                    HomeTab::Leaderboard => self.leaderboard.view_into(context),
                    HomeTab::PlayGame => iced::widget::text("GamePlay").into(),
                })
                .width(Length::Fill)
            ]
            .into(),
            _ => row![
                container(self.leaderboard.view_into(context)).width(Length::FillPortion(3)),
                container(text("hello")).width(Length::FillPortion(2))
            ]
            .into(),
        })
        .into()
    }
}

impl ComponentUpdate for Home {
    type UpdateContext<'a> = (&'a Tournament, &'a Option<PathBuf>);
    fn update(
        &mut self,
        message: Self::Message,
        _: Self::UpdateContext<'_>,
    ) -> anyhow::Result<Effect<Self::Message, Self::OutMessage>> {
        match message {
            HomeMsg::Leaderboard(message) => self.leaderboard.map_update(message, (), |msg| match msg {
                LeaderboardOut::RankPlayer(id) => {
                    Effect::out(HomeOut::OpenPlayView(PlayMode::player(id))).ok()
                }
                LeaderboardOut::OpenPlayerDetails(player_id) => {
                    Effect::out(HomeOut::OpenPlayerDetails(player_id)).ok()
                }
                LeaderboardOut::OpenNewPlayer => Effect::out(HomeOut::OpenNewPlayer).ok(),
            }),
            HomeMsg::SetTab(home_tab) => {
                self.tab = home_tab;
                Effect::done()
            }
        }
    }
}
