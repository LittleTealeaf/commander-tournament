use edh_tourn::{
    error::TournamentError,
    game::{matchup::Matchup, record::GameRecord},
    player::PlayerId,
    tournament::Tournament,
};
use iced::{
    Alignment, Length,
    alignment::Vertical,
    widget::{button, column, container, pick_list, row, space, text},
};
use itertools::Itertools;
use nerd_font_symbols::md::{MD_CANCEL, MD_LINK_VARIANT, MD_LINK_VARIANT_PLUS};

// Assuming you have these imported from your definitions
use crate::{
    effect::Effect,
    traits::{Component, ComponentUpdate, ComponentView},
};

#[derive(Default, Debug)]
pub struct MatchRecorder {
    players: [Option<PlayerId>; 4],
    matchup: Option<Matchup>,
    winner: Option<PlayerId>,
}

impl MatchRecorder {
    #[must_use]
    fn get_player(&self, position: usize) -> Option<&PlayerId> {
        self.players.get(position)?.as_ref()
    }

    pub fn add_player(&mut self, id: PlayerId) {
        if let Some(slot) = self.players.iter_mut().find(|p| p.is_none()) {
            *slot = Some(id);
        }
    }

    fn players(&self) -> Option<[PlayerId; 4]> {
        let [a, b, c, d] = self.players;
        Some([a?, b?, c?, d?])
    }

    fn update_matchup(&mut self, tournament: &Tournament) -> Result<(), TournamentError> {
        self.matchup = match self.players() {
            Some(players) => Some(tournament.create_match(players)?),
            None => None,
        };
        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum MatchRecorderMsg {
    SetPlayers([PlayerId; 4]),
    SetPlayer(usize, Option<PlayerId>),
    SetWinner(Option<PlayerId>),
    AddPlayer(PlayerId),
    SubmitGame,
    Clear,
    // Added local messages for handling links before escalating to OutMessage
    OpenLink(String),
    OpenLinks(Vec<String>),
}

#[derive(Clone, PartialEq, Debug)]
pub enum MatchRecorderOut {
    OpenLink(String),
    RecordGame(Box<GameRecord>),
}

impl Component for MatchRecorder {
    type Message = MatchRecorderMsg;
    type OutMessage = MatchRecorderOut;
}

impl ComponentUpdate for MatchRecorder {
    type UpdateContext<'a> = &'a Tournament;
    fn update(
        &mut self,
        message: Self::Message,
        context: Self::UpdateContext<'_>,
    ) -> anyhow::Result<Effect<Self::Message, Self::OutMessage>> {
        let mut modified = false;

        let effect = match message {
            MatchRecorderMsg::SetPlayers(players) => {
                self.players = players.map(Some);
                modified = true;
                Effect::Done
            }
            MatchRecorderMsg::SetPlayer(position, maybe_id) => {
                let value = self.players.get_mut(position).ok_or_else(|| {
                    anyhow::anyhow!("Player Index {position} invalid: Must be 0, 1, 2, or 3")
                })?;
                *value = maybe_id;
                modified = true;
                Effect::Done
            }
            MatchRecorderMsg::SetWinner(value) => {
                self.winner = value;
                Effect::Done
            }
            MatchRecorderMsg::AddPlayer(player) => {
                self.add_player(player);
                modified = true;
                Effect::Done
            }
            MatchRecorderMsg::SubmitGame => {
                let (Some(matchup), Some(winner)) = (&self.matchup, self.winner) else {
                    return Effect::done();
                };

                let record = matchup.clone().record(winner)?;

                Effect::out(MatchRecorderOut::RecordGame(Box::new(record)))
                    .chain(Effect::msg(MatchRecorderMsg::Clear))
            }
            MatchRecorderMsg::Clear => {
                *self = Self::default();
                Effect::Done
            }
            MatchRecorderMsg::OpenLink(link) => Effect::out(MatchRecorderOut::OpenLink(link)),
            MatchRecorderMsg::OpenLinks(links) => Effect::sequence(
                links
                    .into_iter()
                    .map(|link| Effect::out(MatchRecorderOut::OpenLink(link))),
            ),
        };

        if modified {
            self.update_matchup(context)?;
        }

        Ok(effect)
    }
}

impl ComponentView for MatchRecorder {
    type ViewContext<'a>
        = &'a Tournament
    where
        Self: 'a;
    fn view<'a>(&'a self, context: Self::ViewContext<'a>) -> iced::Element<'a, Self::Message> {
        let players = context
            .get_registered_players()
            .sorted_by(|a, b| a.info().name().cmp(b.info().name()))
            .collect_vec();

        let match_players = (0..4).map(|position| {
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
                let player = matchup.players().get(position)?;

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
                MatchRecorderMsg::SetPlayer(position, Some(option.id()))
            })
            .width(Length::Fill);

            container(
                column![
                    row![
                        selector,
                        button(MD_LINK_VARIANT).on_press_maybe(
                            entry
                                .and_then(|entry| entry.info().moxfield_goldfish_link())
                                .map(MatchRecorderMsg::OpenLink)
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

        let current_players = self
            .players
            .iter()
            .flatten()
            .filter_map(|id| context.get_registered_player(*id))
            .collect_vec();

        let winner_entry = self.winner.and_then(|id| context.get_registered_player(id));

        let winner_selector = row![
            text("Winner: ").size(17),
            pick_list(current_players, winner_entry, |picked| {
                MatchRecorderMsg::SetWinner(Some(picked.id()))
            })
            .width(Length::Fill),
            button(MD_LINK_VARIANT_PLUS).on_press_maybe({
                let links = (0..4)
                    .filter_map(|position| {
                        let id = self.get_player(position)?;
                        let info = context.get_player_info(id)?;
                        info.moxfield_goldfish_link()
                    })
                    .collect_vec();
                if links.is_empty() {
                    None
                } else {
                    Some(MatchRecorderMsg::OpenLinks(links))
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
                (self.matchup.is_some() && self.winner.is_some())
                    .then_some(MatchRecorderMsg::SubmitGame)
            ),
            button(MD_CANCEL).on_press(MatchRecorderMsg::Clear),
        ]
        .spacing(10)
        .align_y(Vertical::Center);

        container(column![title, players_col, winner_selector, submit].spacing(10))
            .padding(10)
            .into()
    }
}
