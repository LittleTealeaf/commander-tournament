use edh_tourn::{
    Tournament,
    error::TournamentError,
    game::{match_player::MatchPlayer, matchup::Matchup},
};
use iced::{widget::pick_list, Task};
use itertools::Itertools;

use crate::{
    App,
    logic::Message,
    traits::{HandleMessage, View},
    view::home::HomeMessage,
};

#[derive(Default, Debug)]
pub struct MatchupView {
    player_a: Option<u32>,
    player_b: Option<u32>,
    player_c: Option<u32>,
    player_d: Option<u32>,
    matchup: Option<Matchup>,
    winner: Option<u32>,
}

impl MatchupView {
    const fn set_player(&mut self, position: MatchViewPlayer, value: Option<u32>) {
        match position {
            MatchViewPlayer::PlayerA => self.player_a = value,
            MatchViewPlayer::PlayerB => self.player_b = value,
            MatchViewPlayer::PlayerC => self.player_c = value,
            MatchViewPlayer::PlayerD => self.player_d = value,
        }
    }

    const fn get_player(&self, position: MatchViewPlayer) -> &Option<u32> {
        match position {
            MatchViewPlayer::PlayerA => &self.player_a,
            MatchViewPlayer::PlayerB => &self.player_b,
            MatchViewPlayer::PlayerC => &self.player_c,
            MatchViewPlayer::PlayerD => &self.player_d,
        }
    }

    fn get_matchup_player(&self, position: MatchViewPlayer) -> Option<&MatchPlayer> {
        let [player_a, player_b, player_c, player_d] = self.matchup.as_ref()?.players();
        Some(match position {
            MatchViewPlayer::PlayerA => player_a,
            MatchViewPlayer::PlayerB => player_b,
            MatchViewPlayer::PlayerC => player_c,
            MatchViewPlayer::PlayerD => player_d,
        })
    }

    fn players(&self) -> Option<[u32; 4]> {
        Some([
            self.player_a?,
            self.player_b?,
            self.player_c?,
            self.player_d?,
        ])
    }

    fn update_matchup(&mut self, tournament: &Tournament) -> Result<(), TournamentError> {
        self.matchup = match self.players() {
            Some(players) => Some(tournament.create_match(players)?),
            None => None,
        };
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MatchViewPlayer {
    PlayerA,
    PlayerB,
    PlayerC,
    PlayerD,
}

impl MatchViewPlayer {
    const PLAYERS: [Self; 4] = [Self::PlayerA, Self::PlayerB, Self::PlayerC, Self::PlayerD];

    fn number(&self) -> usize {
        match self {
            MatchViewPlayer::PlayerA => 1,
            MatchViewPlayer::PlayerB => 2,
            MatchViewPlayer::PlayerC => 3,
            MatchViewPlayer::PlayerD => 4,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum MatchupMessage {
    SetPlayer(MatchViewPlayer, Option<u32>),
    SetWinner(Option<u32>),
    SubmitGame,
    Clear,
}

impl From<MatchupMessage> for Message {
    fn from(value: MatchupMessage) -> Self {
        Self::Home(HomeMessage::MatchupMessage(value))
    }
}

impl HandleMessage<MatchupMessage> for App {
    fn update(&mut self, msg: MatchupMessage) -> anyhow::Result<iced::Task<crate::logic::Message>> {
        let view = &mut self.home.matchup_view;

        match msg {
            MatchupMessage::SetPlayer(position, value) => {
                view.set_player(position, value);
                view.update_matchup(&self.tournament)?;
                Ok(Task::none())
            }
            MatchupMessage::SetWinner(value) => {
                view.winner = value;
                Ok(Task::none())
            }
            MatchupMessage::SubmitGame => {
                let (Some(matchup), Some(winner)) = (&view.matchup, view.winner) else {
                    return Ok(Task::none());
                };

                self.tournament
                    .register_record(matchup.clone().record(winner)?)?;
                *view = MatchupView::default();

                Ok(Task::none())
            }
            MatchupMessage::Clear => {
                *view = MatchupView::default();

                Ok(Task::none())
            }
        }
    }
}

impl View<MatchupView> for App {
    fn view<'a>(&'a self, scene: &'a MatchupView) -> iced::Element<'a, Message> {
        let players = self
            .tournament()
            .players()
            .keys()
            .filter_map(|id| Some((*id, self.tournament().get_player_info(id)?)))
            .sorted_by_key(|(id, info)| info.name())
            .collect_vec();


        let match_players = MatchViewPlayer::PLAYERS.map(|position| {
            let id = scene.get_player(position).as_ref().copied();
            let info = id.and_then(|id| self.tournament.get_player_info(&id));
            let match_player = scene.get_matchup_player(position);

            let selector = pick_list(players.clone(), , on_selected)








        });
    }
}
