use std::path::PathBuf;

use edh_tourn::{game::record::GameRecord, player::PlayerId, tournament::Tournament};
use iced::widget::{column, container, row, rule};

use crate::{
    effect::Effect,
    home::{
        leaderboard::{Leaderboard, LeaderboardMsg, LeaderboardOut},
        match_recorder::{MatchRecorder, MatchRecorderMsg, MatchRecorderOut},
        menu::{Menu, MenuMsg},
        ranking::{Ranking, RankingMsg, RankingOut},
    },
    traits::{Component, ComponentUpdate, ComponentView},
};

pub mod leaderboard;
pub mod match_recorder;
pub mod menu;
pub mod ranking;

#[derive(Debug, Default)]
pub struct Home {
    menu: Menu,
    leaderboard: Leaderboard,
    game_record: MatchRecorder,
    ranking: Ranking,
}

#[derive(Debug, Clone, derive_more::From)]
pub enum HomeMsg {
    Leaderboard(LeaderboardMsg),
    GameRecord(MatchRecorderMsg),
    Ranking(RankingMsg),
    Menu(MenuMsg),
}

#[derive(Debug, Clone, derive_more::From)]
pub enum HomeOut {
    RecordGame(Box<GameRecord>),
    OpenPlayerDetails(PlayerId),
    OpenNewPlayer,
    OpenLink(String),
    FileNew,
    FileOpen,
    FileSave,
    FileSaveAs,
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
            row![
                container(self.leaderboard.view_into(tournament)),
                rule::vertical(2),
                column![
                    self.game_record.view_into(tournament),
                    rule::horizontal(2),
                    self.ranking.view_into(tournament),
                ]
            ]
        ]
        .into()
    }
}

impl ComponentUpdate for Home {
    type UpdateContext<'a> = (&'a Tournament, &'a Option<PathBuf>);
    fn update(
        &mut self,
        message: Self::Message,
        context: Self::UpdateContext<'_>,
    ) -> anyhow::Result<Effect<Self::Message, Self::OutMessage>> {
        let (tournament, _) = context;
        match message {
            HomeMsg::Leaderboard(message) => {
                self.leaderboard
                    .mapped_update(message, (), |msg| match msg {
                        LeaderboardOut::RankPlayer(id) => {
                            Effect::msg(RankingMsg::SelectPlayer(id)).ok()
                        }
                        LeaderboardOut::OpenPlayerDetails(player_id) => {
                            Effect::out(HomeOut::OpenPlayerDetails(player_id)).ok()
                        }
                        LeaderboardOut::OpenNewPlayer => Effect::out(HomeOut::OpenNewPlayer).ok(),
                    })
            }
            HomeMsg::Ranking(message) => {
                self.ranking
                    .mapped_update(message, tournament, |message| match message {
                        RankingOut::LoadGame(game) => {
                            Effect::msg(MatchRecorderMsg::SetPlayers(game)).ok()
                        }
                        RankingOut::OpenPlayerDetails(player_id) => {
                            Effect::Out(HomeOut::OpenPlayerDetails(player_id)).ok()
                        }
                    })
            }
            HomeMsg::GameRecord(message) => {
                self.game_record
                    .mapped_update(message, tournament, |out| match out {
                        MatchRecorderOut::OpenLink(link) => {
                            Effect::Out(HomeOut::OpenLink(link)).ok()
                        }
                        MatchRecorderOut::RecordGame(record) => {
                            Effect::out(HomeOut::RecordGame(record)).ok()
                        }
                    })
            }
            HomeMsg::Menu(message) => self.menu.mapped_update(message, (), |out| {
                Effect::out(match out {
                    MenuMsg::New => HomeOut::FileNew,
                    MenuMsg::Open => HomeOut::FileOpen,
                    MenuMsg::Save => HomeOut::FileSave,
                    MenuMsg::SaveAs => HomeOut::FileSaveAs,
                })
                .ok()
            }),
        }
    }
}
