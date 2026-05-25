use std::path::PathBuf;

use edh_tourn::{game::record::GameRecord, player::PlayerId, tournament::Tournament};
use iced::widget::container;

use crate::{
    effect::Effect,
    home::leaderboard::{Leaderboard, LeaderboardMsg, LeaderboardOut},
    traits::{Component, ComponentUpdate, ComponentView},
    views::play::PlayMode,
};

pub mod leaderboard;

#[derive(Debug, Default)]
pub struct Home {
    leaderboard: Leaderboard,
}

#[derive(Debug, Clone, derive_more::From)]
pub enum HomeMsg {
    Leaderboard(LeaderboardMsg),
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
        container(self.leaderboard.view_into(context)).into()
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
        }
    }
}
