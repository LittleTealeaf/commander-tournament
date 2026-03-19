use edh_tourn::tournament::Tournament;

use crate::traits::{Component, ComponentUpdate, ComponentView, Effect, HandleMessage};

pub mod game_record;
pub mod leaderboard;
pub mod ranking;

#[derive(Debug)]
pub struct State {
    leaderboard: leaderboard::State,
    game_record: game_record::State,
    ranking: ranking::State,
}

#[derive(Debug, Clone, derive_more::From)]
pub enum Message {
    Leaderboard(leaderboard::Message),
    GameRecord(game_record::Message),
    Ranking(ranking::Message),
}

#[derive(Debug, Clone)]
pub enum OutMessage {
    OpenPlayerDetails(Option<u32>),
}

impl Component for State {
    type Message = Message;
    type OutMessage = OutMessage;
    type Context<'a> = &'a Tournament;
}

impl ComponentView for State {
    fn view<'a>(&'a self, _context: Self::Context<'a>) -> iced::Element<'a, Self::Message> {
        todo!()
    }
}

impl ComponentUpdate for State {
    fn update(
        &mut self,
        message: Self::Message,
        context: Self::Context<'_>,
    ) -> anyhow::Result<Effect<Self::Message, Self::OutMessage>> {
        match message {
            Message::Leaderboard(message) => self.handle_message(message, context),
            Message::GameRecord(message) => self.handle_message(message, context),
            Message::Ranking(message) => self.handle_message(message, context),
        }
    }
}

impl HandleMessage<leaderboard::Message> for State {
    fn handle_message(
        &mut self,
        message: leaderboard::Message,
        context: Self::Context<'_>,
    ) -> anyhow::Result<Effect<Self::Message, Self::OutMessage>> {
        self.leaderboard
            .update(message, context)?
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
        context: Self::Context<'_>,
    ) -> anyhow::Result<Effect<Self::Message, Self::OutMessage>> {
        let _effect = self.game_record.update(message, context)?;

        Effect::ok()
    }
}

impl HandleMessage<ranking::Message> for State {
    fn handle_message(
        &mut self,
        message: ranking::Message,
        context: Self::Context<'_>,
    ) -> anyhow::Result<Effect<Self::Message, Self::OutMessage>> {
        self.ranking
            .update(message, context)?
            .map(|_message| todo!())
    }
}
