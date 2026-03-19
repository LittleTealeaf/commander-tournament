use edh_tourn::tournament::Tournament;

use crate::traits::{Component, ComponentUpdate};

#[derive(Debug)]
pub struct State {
    player: Option<u32>,
}

#[derive(Debug, Clone)]
pub enum Message {
    SelectPlayer(u32),
}

#[derive(Debug)]
pub enum OutMessage {}

impl Component for State {
    type OutMessage = OutMessage;
    type Message = Message;
    type Context<'a> = &'a Tournament;
}

impl ComponentUpdate for State {
    fn update(
        &mut self,
        _message: Self::Message,
        _context: Self::Context<'_>,
    ) -> anyhow::Result<crate::traits::Effect<Self::Message, Self::OutMessage>> {
        todo!()
    }
}
