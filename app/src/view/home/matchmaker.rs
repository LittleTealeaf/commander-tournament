use edh_tourn::Tournament;
use itertools::Itertools;

use crate::{
    App,
    logic::Message,
    traits::{HandleMessage, View},
};

pub struct MatchMakerView {
    method: MatchMethod,
    player: Option<u32>,
    leaderboard: Vec<u32>,
    show_count: usize,
}

impl Default for MatchMakerView {
    fn default() -> Self {
        Self {
            method: MatchMethod::default(),
            player: None,
            leaderboard: Vec::new(),
            show_count: 5,
        }
    }
}

impl MatchMakerView {
    fn update(&mut self, tournament: &Tournament) -> anyhow::Result<()> {
        let Some(id) = self.player else {
            self.leaderboard = Vec::new();
            return Ok(());
        };

        self.leaderboard = match &self.method {
            MatchMethod::LeastPlayed => tournament.rank_least_played(id)?.collect_vec(),
            MatchMethod::ExpectedNeighbors => tournament.rank_expected_neighbors(id)?.collect_vec(),
            MatchMethod::LossWith => tournament.rank_loss_with(id)?.collect_vec(),
            MatchMethod::Nemesis => tournament.rank_nemesis(id)?.collect_vec(),
            MatchMethod::EloNeighbors => tournament.rank_neighbors(id)?.collect_vec(),
            MatchMethod::WRNeighbors => tournament.rank_wr_neighbors(id)?.collect_vec(),
            MatchMethod::Combined => tournament.rank_combined(id)?.collect_vec(),
        }
        .into_iter()
        .take(self.show_count)
        .collect_vec();

        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum MatchMakerMessage {
    Method(MatchMethod),
    Player(Option<u32>),
    ViewCount(usize),
}

impl HandleMessage<MatchMakerMessage> for App {
    fn update(
        &mut self,
        msg: MatchMakerMessage,
    ) -> anyhow::Result<iced::Task<crate::logic::Message>> {
        let view = &mut self.home.matchmaker;
        match msg {
            MatchMakerMessage::Method(match_method) => {
                view.method = match_method;
                view.update(&self.tournament)?;
                Message::done()
            }
            MatchMakerMessage::Player(player) => {
                view.player = player;
                view.update(&self.tournament)?;
                Message::done()
            }
            MatchMakerMessage::ViewCount(count) => {
                view.show_count = count;
                view.update(&self.tournament)?;
                Message::done()
            }
        }
    }
}

impl View<MatchMakerView> for App {
    fn view<'a>(&'a self, scene: &'a MatchMakerView) -> iced::Element<'a, Message> {
        todo!()
    }
}

#[derive(Clone, Default, Debug, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MatchMethod {
    LeastPlayed,
    ExpectedNeighbors,
    LossWith,
    Nemesis,
    EloNeighbors,
    WRNeighbors,
    #[default]
    Combined,
}
