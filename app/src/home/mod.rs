use std::path::PathBuf;

use edh_tourn::tournament::Tournament;
use iced::widget::{column, container, row, rule};

use crate::{
    core::tournament::TournamentAction, effect::Effect, home::{
        leaderboard::{Leaderboard, LeaderboardMsg},
        match_recorder::{MatchRecorder, MatchRecorderMsg},
        menu::{Menu, MenuMsg},
        ranking::{Ranking, RankingMsg},
    }, traits::{Component, ComponentUpdate, ComponentView, HandleMessage}
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

impl Component for Home {
    type Message = HomeMsg;
    type OutMessage = ();
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
        match message {
            HomeMsg::Leaderboard(message) => self.handle_message(message, context),
            HomeMsg::GameRecord(message) => self.handle_message(message, context),
            HomeMsg::Ranking(message) => self.handle_message(message, context),
            HomeMsg::Menu(message) => self.handle_message(message, context),
        }
    }
}

impl HandleMessage<MenuMsg> for Home {
    fn handle_message(
        &mut self,
        message: MenuMsg,
        _: Self::UpdateContext<'_>,
    ) -> anyhow::Result<Effect<Self::Message, Self::OutMessage>> {
        self.menu.update(message, ())?.map_empty()
    }
}

impl HandleMessage<LeaderboardMsg> for Home {
    fn handle_message(
        &mut self,
        message: LeaderboardMsg,
        context: Self::UpdateContext<'_>,
    ) -> anyhow::Result<Effect<Self::Message, Self::OutMessage>> {
        self.leaderboard
            .update(message, ())?
            .map(|message| match message {
                leaderboard::LeaderboardOut::RankPlayer(id) => {
                    self.handle_message(RankingMsg::SelectPlayer(id), context)
                }
            })
    }
}

impl HandleMessage<MatchRecorderMsg> for Home {
    fn handle_message(
        &mut self,
        message: MatchRecorderMsg,
        (tournament, _): Self::UpdateContext<'_>,
    ) -> anyhow::Result<Effect<Self::Message, Self::OutMessage>> {
        self.game_record
            .update(message, tournament)?
            .map(|message| match message {
                match_recorder::MatchRecorderOut::SubmitRecord(game_record) => {
                    Effect::global(TournamentAction::Record(game_record)).ok()
                }
            })
    }
}

impl HandleMessage<RankingMsg> for Home {
    fn handle_message(
        &mut self,
        message: RankingMsg,
        context: Self::UpdateContext<'_>,
    ) -> anyhow::Result<Effect<Self::Message, Self::OutMessage>> {
        let (tournament, _) = context;
        self.ranking
            .update(message, tournament)?
            .map(|message| match message {
                ranking::RankingOut::LoadGame(players) => {
                    self.handle_message(MatchRecorderMsg::SetPlayers(players), context)
                }
            })
    }
}
