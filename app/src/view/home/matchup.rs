use edh_tourn::{Tournament, error::TournamentError, game::Matchup};
use iced::Task;

use crate::{App, logic::Message, traits::HandleMessage, view::home::HomeMessage};

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
    const fn set_player(&mut self, position: MatchupPlayerPosition, value: Option<u32>) {
        match position {
            MatchupPlayerPosition::PlayerA => self.player_a = value,
            MatchupPlayerPosition::PlayerB => self.player_b = value,
            MatchupPlayerPosition::PlayerC => self.player_c = value,
            MatchupPlayerPosition::PlayerD => self.player_d = value,
        }
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
pub enum MatchupPlayerPosition {
    PlayerA,
    PlayerB,
    PlayerC,
    PlayerD,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum MatchupMessage {
    SetPlayer(MatchupPlayerPosition, Option<u32>),
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

                self.tournament.register_match(matchup.clone(), winner)?;
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

#[cfg(test)]
mod tests {

}

