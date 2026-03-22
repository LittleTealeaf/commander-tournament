use edh_tourn::{
    error::TournamentError,
    game::{matchup::Matchup, record::GameRecord},
    tournament::Tournament,
};
use iced::{
    Alignment, Length, Task,
    alignment::Vertical,
    widget::{button, column, container, pick_list, row, space, text},
};
use itertools::Itertools;
use nerd_font_symbols::md::{MD_CANCEL, MD_LINK_VARIANT, MD_LINK_VARIANT_PLUS};

// Assuming you have these imported from your definitions
use crate::{
    services::system::open_link,
    traits::{Component, ComponentUpdate, ComponentView, Effect},
};

#[derive(Default, Debug)]
pub struct State {
    player_a: Option<u32>,
    player_b: Option<u32>,
    player_c: Option<u32>,
    player_d: Option<u32>,
    matchup: Option<Matchup>,
    winner: Option<u32>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Player {
    PlayerA,
    PlayerB,
    PlayerC,
    PlayerD,
}

impl Player {
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
pub enum Message {
    SetPlayers([u32; 4]),
    SetPlayer(Player, Option<u32>),
    SetWinner(Option<u32>),
    AddPlayer(u32),
    SubmitGame,
    Clear,
    // Added local messages for handling links before escalating to OutMessage
    OpenLink(String),
    OpenLinks(Vec<String>),
    Nothing,
}

#[derive(Debug)]
pub enum OutMessage {
    SubmitRecord(Box<GameRecord>),
}

impl State {
    const fn set_player(&mut self, position: Player, value: Option<u32>) {
        match position {
            Player::PlayerA => self.player_a = value,
            Player::PlayerB => self.player_b = value,
            Player::PlayerC => self.player_c = value,
            Player::PlayerD => self.player_d = value,
        }
    }

    #[must_use]
    const fn get_player(&self, position: Player) -> Option<&u32> {
        match position {
            Player::PlayerA => self.player_a.as_ref(),
            Player::PlayerB => self.player_b.as_ref(),
            Player::PlayerC => self.player_c.as_ref(),
            Player::PlayerD => self.player_d.as_ref(),
        }
    }

    pub fn add_player(&mut self, id: u32) {
        for player in Player::PLAYERS {
            if self.get_player(player).is_none() {
                self.set_player(player, Some(id));
                return;
            }
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

impl Component for State {
    type Message = Message;
    type OutMessage = OutMessage;
}

impl ComponentUpdate for State {
    type UpdateContext<'a> = &'a Tournament;
    fn update(
        &mut self,
        message: Self::Message,
        context: Self::UpdateContext<'_>,
    ) -> anyhow::Result<Effect<Self::Message, Self::OutMessage>> {
        match message {
            Message::SetPlayers([a, b, c, d]) => {
                self.player_a = Some(a);
                self.player_b = Some(b);
                self.player_c = Some(c);
                self.player_d = Some(d);
                self.update_matchup(context)?;
                Effect::done()
            }
            Message::SetPlayer(position, value) => {
                self.set_player(position, value);
                self.update_matchup(context)?;
                Effect::done()
            }
            Message::SetWinner(value) => {
                self.winner = value;
                Effect::done()
            }
            Message::AddPlayer(player) => {
                self.add_player(player);
                self.update_matchup(context)?;
                Effect::done()
            }
            Message::SubmitGame => {
                let (Some(matchup), Some(winner)) = (&self.matchup, self.winner) else {
                    return Effect::done();
                };

                let record = matchup.clone().record(winner)?;

                *self = Self::default();

                Effect::out(OutMessage::SubmitRecord(Box::new(record)))
            }
            Message::Clear => {
                *self = Self::default();
                Effect::done()
            }
            Message::OpenLink(link) => Effect::task(Task::future(async {
                let _ = open_link(link).await;
                Message::Nothing
            })),
            Message::OpenLinks(links) => Effect::task(Task::future(async {
                for link in links {
                    let _ = open_link(link).await;
                }
                Message::Nothing
            })),
            Message::Nothing => Effect::done(),
        }
    }
}

impl ComponentView for State {
    type ViewContext<'a>
        = &'a Tournament
    where
        Self: 'a;
    fn view<'a>(&'a self, context: Self::ViewContext<'a>) -> iced::Element<'a, Self::Message> {
        let players = context
            .get_registered_players()
            .sorted_by(|a, b| a.info().name().cmp(b.info().name()))
            .collect_vec();

        let match_players = Player::PLAYERS.map(|position| {
            let id = self.get_player(position).copied();
            let entry = id.and_then(|id| context.get_registered_player(id));

            let text_stats = entry.map(|p| {
                let stats = p.stats();
                let str_wr = stats.wr().map_or_else(
                    || "--% WR".to_owned(),
                    |wr| format!("{}% WR", (wr * 100.0).round()),
                );
                text(format!("{} Elo, {str_wr}", stats.elo().round()))
            });

            let text_expected = self.matchup.as_ref().and_then(|matchup| {
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
                Message::SetPlayer(position, Some(option.id()))
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

        let players_col = column(match_players).spacing(15);

        let title = text("Record Game")
            .size(20)
            .align_x(Alignment::Center)
            .width(Length::Fill);

        let current_players = Player::PLAYERS
            .iter()
            .filter_map(|player| self.get_player(*player).copied())
            .filter_map(|id| context.get_registered_player(id))
            .collect_vec();

        let winner_entry = self.winner.and_then(|id| context.get_registered_player(id));

        let winner_selector = row![
            text("Winner: ").size(17),
            pick_list(current_players, winner_entry, |picked| {
                Message::SetWinner(Some(picked.id()))
            })
            .width(Length::Fill),
            button(MD_LINK_VARIANT_PLUS).on_press_maybe({
                let links = Player::PLAYERS
                    .into_iter()
                    .filter_map(|position| {
                        let id = self.get_player(position)?;
                        let info = context.get_player_info(id)?;
                        info.moxfield_goldfish_link()
                    })
                    .collect_vec();
                if links.is_empty() {
                    None
                } else {
                    Some(Message::OpenLinks(links))
                }
            })
        ]
        .spacing(10)
        .align_y(Vertical::Center);

        let results_preview = self.matchup.as_ref().and_then(|matchup| {
            let winner = winner_entry?;
            let increase = matchup.get_player(winner.id())?.elo_win().round();
            let elo = winner.stats().elo().round();
            let elo_result = elo + increase;

            Some(text(format!("Elo: {elo} + {increase} = {elo_result}")))
        });

        let submit = row![
            space().width(Length::Fill),
            results_preview,
            button("Submit").on_press_maybe(
                (self.matchup.is_some() && self.winner.is_some()).then_some(Message::SubmitGame)
            ),
            button(MD_CANCEL).on_press(Message::Clear),
        ]
        .spacing(10)
        .align_y(Vertical::Center);

        container(column![title, players_col, winner_selector, submit].spacing(10))
            .padding(10)
            .into()
    }
}
