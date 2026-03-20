use anyhow::anyhow;

use edh_tourn::{player::RegisteredPlayer, ranking::RankingMethod, tournament::Tournament};
use iced::widget::text;

use crate::traits::{Component, ComponentUpdate, ComponentView, Effect};

#[derive(Debug, Default)]
pub struct State {
    method: RankingMethod,
    player: Option<u32>,
}

#[derive(Debug, Clone)]
pub enum Message {
    SelectPlayer(u32),
    AddTopThree,
    SetMethod(RankingMethod),
}

#[derive(Debug)]
pub enum OutMessage {
    LoadGame([u32; 4]),
}

impl Component for State {
    type OutMessage = OutMessage;
    type Message = Message;
    type Context<'a> = &'a Tournament;
}

impl ComponentUpdate for State {
    fn update(
        &mut self,
        message: Self::Message,
        context: Self::Context<'_>,
    ) -> anyhow::Result<crate::traits::Effect<Self::Message, Self::OutMessage>> {
        match message {
            Message::SelectPlayer(id) => {
                context.require_id_registered(id)?;
                self.player = Some(id);
                Effect::ok()
            }
            Message::AddTopThree => {
                let Some(id) = self.player else {
                    return Err(anyhow!("Player not specified"));
                };
                let ranked = context.get_player_ranked(id, self.method)?;
                let mut ranked_iter = ranked.iter().map(RegisteredPlayer::id);
                let players = [
                    id,
                    ranked_iter
                        .next()
                        .ok_or_else(|| anyhow!("No players found in ranking"))?,
                    ranked_iter
                        .next()
                        .ok_or_else(|| anyhow!("Only 1 player found in ranking"))?,
                    ranked_iter
                        .next()
                        .ok_or_else(|| anyhow!("Only 2 players found in ranking"))?,
                ];

                Effect::out(OutMessage::LoadGame(players))
            }
            Message::SetMethod(method) => {
                self.method = method;
                Effect::ok()
            }
        }
    }
}

impl ComponentView for State {
    fn view<'a>(&'a self, _: Self::Context<'a>) -> iced::Element<'a, Self::Message> {
        text("ranking").into()
    }
}
