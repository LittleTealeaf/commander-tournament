mod update;
mod view;

use edh_tourn::ranking::RankingMethod;

use crate::traits::Component;

#[derive(Debug, Default)]
pub struct State {
    method: RankingMethod,
    player: Option<u32>,
}

#[derive(Debug, Clone)]
pub enum Message {
    SelectPlayer(u32),
    OpenPlayerDetails(u32),
    AddTopThree,
    SetMethod(RankingMethod),
}

#[derive(Debug)]
pub enum OutMessage {
    LoadGame([u32; 4]),
    OpenPlayerDetails(u32),
}

impl Component for State {
    type OutMessage = OutMessage;
    type Message = Message;
}
