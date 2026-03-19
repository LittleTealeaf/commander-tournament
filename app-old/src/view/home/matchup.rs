use edh_tourn::{
    error::TournamentError,
    game::{match_player::MatchPlayer, matchup::Matchup},
    tournament::Tournament,
};
use iced::{
    Alignment, Length, Task,
    alignment::Vertical,
    widget::{button, column, container, pick_list, row, space, text},
};
use itertools::Itertools;
use nerd_font_symbols::md::{MD_CANCEL, MD_LINK_VARIANT, MD_LINK_VARIANT_PLUS};

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

    #[must_use]
    const fn get_player(&self, position: MatchViewPlayer) -> Option<&u32> {
        match position {
            MatchViewPlayer::PlayerA => self.player_a.as_ref(),
            MatchViewPlayer::PlayerB => self.player_b.as_ref(),
            MatchViewPlayer::PlayerC => self.player_c.as_ref(),
            MatchViewPlayer::PlayerD => self.player_d.as_ref(),
        }
    }

    pub fn add_player(&mut self, id: u32) {
        for player in MatchViewPlayer::PLAYERS {
            if self.get_player(player).is_none() {
                self.set_player(player, Some(id));
                return;
            }
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

    const fn number(self) -> usize {
        match self {
            Self::PlayerA => 0,
            Self::PlayerB => 1,
            Self::PlayerC => 2,
            Self::PlayerD => 3,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum MatchupMessage {
    SetPlayer(MatchViewPlayer, Option<u32>),
    SetWinner(Option<u32>),
    AddPlayer(u32),
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
            MatchupMessage::AddPlayer(player) => {
                view.add_player(player);
                view.update_matchup(&self.tournament)?;
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
            .tournament
            .get_registered_players()
            .sorted_by(|a, b| a.info().name().cmp(b.info().name()))
            .collect_vec();

        let match_players = MatchViewPlayer::PLAYERS.map(|position| {
            let id = scene.get_player(position).copied();
            let entry = id.and_then(|id| self.tournament.get_registered_player(id));

            let text_stats = entry.map(|p| {
                let stats = p.stats();
                let str_wr = stats.wr().map_or_else(
                    || "--% WR".to_owned(),
                    |wr| format!("{}% WR", (wr * 100.0).round()),
                );
                text(format!("{} Elo, {str_wr}", stats.elo().round()))
            });

            let text_expected = scene.matchup.as_ref().and_then(|matchup| {
                let player = matchup.players().get(position.number())?;

                Some(text(format!(
                    "Expected: {}% (+{}/-{})",
                    (player.expected() * 100f64).round(),
                    player.elo_win().round(),
                    player.elo_loss().round()
                )))
            });

            let player_info = row![
                text_stats,
                text(""),
                space().width(Length::Fill),
                text_expected
            ];

            let selector = pick_list(players.clone(), entry, move |option| {
                MatchupMessage::SetPlayer(position, Some(option.id())).into()
            })
            .width(Length::Fill);

            container(
                column![
                    row![
                        selector,
                        button(MD_LINK_VARIANT).on_press_maybe(
                            entry
                                .and_then(|entry| entry.info().moxfield_goldfish_link())
                                .map(Message::OpenLink)
                        )
                    ],
                    player_info
                ]
                .spacing(5),
            )
            .into()
        });

        let players = column(match_players).spacing(15);

        let title = text("Record Game")
            .size(20)
            .align_x(Alignment::Center)
            .width(Length::Fill);

        let current_players = MatchViewPlayer::PLAYERS
            .iter()
            .filter_map(|player| scene.get_player(*player).copied())
            .filter_map(|id| self.tournament().get_registered_player(id))
            .collect_vec();

        let winner = scene
            .winner
            .and_then(|id| self.tournament().get_registered_player(id));

        let winner_selector = row![
            text("Winner: ").size(17),
            pick_list(current_players, winner, |picked| {
                MatchupMessage::SetWinner(Some(picked.id())).into()
            })
            .width(Length::Fill),
            button(MD_LINK_VARIANT_PLUS).on_press_maybe({
                let links = MatchViewPlayer::PLAYERS
                    .into_iter()
                    .filter_map(|position| {
                        let id = scene.get_player(position)?;
                        let info = self.tournament().get_player_info(id)?;
                        let link = info.moxfield_goldfish_link()?;
                        Some(Message::OpenLink(link))
                    })
                    .collect_vec();
                if links.is_empty() {
                    None
                } else {
                    Some(Message::Batch(links))
                }
            })
        ]
        .spacing(10)
        .align_y(Vertical::Center);

        let results_preview = scene.matchup.as_ref().and_then(|matchup| {
            let winner = winner?;
            let increase = matchup.get_player(winner.id())?.elo_win().round();
            let elo = winner.stats().elo().round();
            let elo_result = elo + increase;

            Some(text(format!("Elo: {elo} + {increase} = {elo_result}")))
        });

        let submit = row![
            space().width(Length::Fill),
            results_preview,
            button("Submit").on_press_maybe(
                (scene.matchup.is_some() && scene.winner.is_some())
                    .then_some(MatchupMessage::SubmitGame.into())
            ),
            button(MD_CANCEL).on_press(MatchupMessage::Clear.into()),
        ]
        .spacing(10)
        .align_y(Vertical::Center);

        container(column![title, players, winner_selector, submit].spacing(10))
            .padding(10)
            .into()
    }
}
