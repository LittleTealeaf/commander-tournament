use std::path::PathBuf;

use edh_tourn::{game::record::GameRecord, tournament::Tournament};
use iced::widget::{column, container, row, rule};

use crate::traits::{Component, ComponentUpdate, ComponentView, Effect, HandleMessage};

pub mod game_record;
pub mod leaderboard;
pub mod menu;
pub mod ranking;

#[derive(Debug, Default)]
pub struct State {
    menu: menu::State,
    leaderboard: leaderboard::State,
    game_record: game_record::State,
    ranking: ranking::State,
}

#[derive(Debug, Clone, derive_more::From)]
pub enum Message {
    Leaderboard(leaderboard::Message),
    GameRecord(game_record::Message),
    Ranking(ranking::Message),
    Menu(menu::Message),
}

#[derive(Debug, Clone)]
pub enum OutMessage {
    OpenPlayerDetails(Option<u32>),
    RegisterRecord(Box<GameRecord>),
    MenuMessage(menu::Message),
}

impl Component for State {
    type Message = Message;
    type OutMessage = OutMessage;
}

impl ComponentView for State {
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

impl ComponentUpdate for State {
    type UpdateContext<'a> = (&'a Tournament, &'a Option<PathBuf>);
    fn update(
        &mut self,
        message: Self::Message,
        context: Self::UpdateContext<'_>,
    ) -> anyhow::Result<Effect<Self::Message, Self::OutMessage>> {
        match message {
            Message::Leaderboard(message) => self.handle_message(message, context),
            Message::GameRecord(message) => self.handle_message(message, context),
            Message::Ranking(message) => self.handle_message(message, context),
            Message::Menu(message) => Effect::out(OutMessage::MenuMessage(message)),
        }
    }
}

impl HandleMessage<leaderboard::Message> for State {
    fn handle_message(
        &mut self,
        message: leaderboard::Message,
        context: Self::UpdateContext<'_>,
    ) -> anyhow::Result<Effect<Self::Message, Self::OutMessage>> {
        self.leaderboard
            .update(message, ())?
            .map(|message| match message {
                leaderboard::OutMessage::OpenPlayerDetails(maybe_id) => {
                    Effect::out(OutMessage::OpenPlayerDetails(maybe_id))
                }
                leaderboard::OutMessage::RankPlayer(id) => {
                    self.handle_message(ranking::Message::SelectPlayer(id), context)
                }
            })
    }
}

impl HandleMessage<game_record::Message> for State {
    fn handle_message(
        &mut self,
        message: game_record::Message,
        (tournament, _): Self::UpdateContext<'_>,
    ) -> anyhow::Result<Effect<Self::Message, Self::OutMessage>> {
        self.game_record
            .update(message, tournament)?
            .map(|message| match message {
                game_record::OutMessage::SubmitRecord(game_record) => {
                    Effect::out(OutMessage::RegisterRecord(game_record))
                }
            })
    }
}

impl HandleMessage<ranking::Message> for State {
    fn handle_message(
        &mut self,
        message: ranking::Message,
        context: Self::UpdateContext<'_>,
    ) -> anyhow::Result<Effect<Self::Message, Self::OutMessage>> {
        let (tournament, _) = context;
        self.ranking
            .update(message, tournament)?
            .map(|message| match message {
                ranking::OutMessage::LoadGame(players) => {
                    self.handle_message(game_record::Message::SetPlayers(players), context)
                }
                ranking::OutMessage::OpenPlayerDetails(id) => {
                    Effect::out(OutMessage::OpenPlayerDetails(Some(id)))
                }
            })
    }
}
