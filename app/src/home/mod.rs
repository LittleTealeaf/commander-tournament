use std::path::PathBuf;

use edh_tourn::{game::record::GameRecord, player::PlayerId, tournament::Tournament};
use iced::widget::{column, container};

use crate::{
    effect::Effect,
    home::{
        leaderboard::{Leaderboard, LeaderboardMsg, LeaderboardOut},
        menu::{Menu, MenuMsg},
    },
    traits::{Component, ComponentUpdate, ComponentView},
    views::play::PlayMode,
};

pub mod leaderboard;
pub mod menu;

#[derive(Debug, Default)]
pub struct Home {
    menu: Menu,
    leaderboard: Leaderboard,
}

#[derive(Debug, Clone, derive_more::From)]
pub enum HomeMsg {
    Leaderboard(LeaderboardMsg),
    Menu(MenuMsg),
}

#[derive(Debug, Clone, derive_more::From)]
pub enum HomeOut {
    #[from(skip)]
    OpenPlayView(PlayMode),
    RecordGame(Box<GameRecord>),
    OpenPlayerDetails(PlayerId),
    OpenNewPlayer,
    OpenLink(String),
    FileNew,
    FileOpen,
    FileSave,
    FileSaveAs,
    OpenGameConfig,
}

impl Component for Home {
    type Message = HomeMsg;
    type OutMessage = HomeOut;
}

impl ComponentView for Home {
    type ViewContext<'a>
        = (&'a Tournament, &'a Option<PathBuf>)
    where
        Self: 'a;
    fn view<'a>(&'a self, context: Self::ViewContext<'a>) -> iced::Element<'a, Self::Message> {
        let (tournament, path) = context;
        column![
            self.menu.view_into(path),
            container(self.leaderboard.view_into(tournament)),
        ]
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
            HomeMsg::Leaderboard(message) => {
                self.leaderboard.map_update(message, (), |msg| match msg {
                    LeaderboardOut::RankPlayer(id) => {
                        Effect::out(HomeOut::OpenPlayView(PlayMode::player(id))).ok()
                    }
                    LeaderboardOut::OpenPlayerDetails(player_id) => {
                        Effect::out(HomeOut::OpenPlayerDetails(player_id)).ok()
                    }
                    LeaderboardOut::OpenNewPlayer => Effect::out(HomeOut::OpenNewPlayer).ok(),
                })
            }
            HomeMsg::Menu(message) => self.menu.map_update(message, (), |out| {
                Effect::out(match out {
                    MenuMsg::New => HomeOut::FileNew,
                    MenuMsg::Open => HomeOut::FileOpen,
                    MenuMsg::Save => HomeOut::FileSave,
                    MenuMsg::SaveAs => HomeOut::FileSaveAs,
                    MenuMsg::OpenPlayNext => HomeOut::OpenPlayView(PlayMode::next()),
                    MenuMsg::OpenPlayCustom => HomeOut::OpenPlayView(PlayMode::custom()),
                    MenuMsg::OpenGameConfig => HomeOut::OpenGameConfig,
                })
                .ok()
            }),
        }
    }
}
