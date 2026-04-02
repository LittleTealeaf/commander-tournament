use edh_tourn::{
    game::{matchup::Matchup, record::GameRecord},
    player::PlayerId,
    tournament::Tournament,
};
use iced::{
    Length,
    widget::{button, column, container, pick_list, row, space, text},
};
use itertools::Itertools;
use nerd_font_symbols::md::{MD_LINK_VARIANT, MD_LINK_VARIANT_PLUS};

use crate::{
    effect::Effect,
    traits::{Component, ComponentUpdate, ComponentView},
};

#[derive(Debug, Clone)]
pub struct MatchPreview {
    matchup: Matchup,
    winner: Option<PlayerId>,
}

impl MatchPreview {
    #[must_use]
    pub const fn new(matchup: Matchup) -> Self {
        Self {
            matchup,
            winner: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum MatchPreviewMsg {
    SelectWinner(PlayerId),
    OpenLink(String),
    OpenMatchLinks,
    Submit,
}

#[derive(Debug, Clone)]
pub enum MatchPreviewOut {
    RecordGame(Box<GameRecord>),
    OpenLink(String),
}

impl Component for MatchPreview {
    type Message = MatchPreviewMsg;
    type OutMessage = MatchPreviewOut;
}

impl ComponentUpdate for MatchPreview {
    type UpdateContext<'a> = &'a Tournament;
    fn update(
        &mut self,
        message: Self::Message,
        context: Self::UpdateContext<'_>,
    ) -> anyhow::Result<crate::effect::Effect<Self::Message, Self::OutMessage>> {
        match message {
            MatchPreviewMsg::Submit => {
                if let Some(winner) = &self.winner {
                    Effect::out(MatchPreviewOut::RecordGame(Box::new(
                        self.matchup.clone().record(*winner)?,
                    )))
                    .ok()
                } else {
                    Effect::done()
                }
            }
            MatchPreviewMsg::OpenLink(link) => Effect::out(MatchPreviewOut::OpenLink(link)).ok(),
            MatchPreviewMsg::OpenMatchLinks => Effect::sequence(
                self.matchup
                    .players()
                    .iter()
                    .filter_map(|player| {
                        context
                            .get_registered_player(player.id())
                            .and_then(|reg| reg.info().moxfield_goldfish_link())
                    })
                    .map(|link| Effect::out(MatchPreviewOut::OpenLink(link))),
            )
            .ok(),
            MatchPreviewMsg::SelectWinner(player_id) => {
                self.winner = Some(player_id);
                Effect::done()
            }
        }
    }
}

impl ComponentView for MatchPreview {
    type ViewContext<'a>
        = &'a Tournament
    where
        Self: 'a;
    fn view<'a>(&'a self, context: Self::ViewContext<'a>) -> iced::Element<'a, Self::Message> {
        let tourn = context;

        let player_views = self.matchup.players().iter().filter_map(|player| {
            let info = tourn.get_player_info(&player.id())?;

            let text_stats = {
                let stats = player.stats();
                let str_wr = stats.wr().map_or_else(
                    || "--% WR".to_owned(),
                    |wr| format!("{}% WR", (wr * 100.0).round()),
                );
                text(format!("{} Elo, {str_wr}", stats.elo().round()))
            };

            let text_expected = {
                text(format!(
                    "Expected: {}% (+{}/-{})",
                    (player.expected() * 100f64).round(),
                    player.elo_win().round(),
                    player.elo_loss().round()
                ))
            };

            let player_info = row![
                text_stats,
                text(""),
                space().width(Length::Fill),
                text_expected
            ];

            Some(
                container(
                    column![
                        row![
                            text(info.name()),
                            button(MD_LINK_VARIANT).on_press_maybe(
                                info.moxfield_goldfish_link().map(MatchPreviewMsg::OpenLink)
                            )
                        ],
                        player_info
                    ]
                    .spacing(5),
                )
                .into(),
            )
        });

        let players_col = column(player_views).spacing(15);

        let winner_options = self
            .matchup
            .players()
            .iter()
            .filter_map(|player| tourn.get_registered_player(player.id()))
            .collect_vec();

        let selected_winner = self
            .winner
            .as_ref()
            .and_then(|id| tourn.get_registered_player(*id));

        let winner_selector = row![
            text("Winner: ").size(27),
            pick_list(winner_options, selected_winner, |picked| {
                MatchPreviewMsg::SelectWinner(picked.id())
            })
            .width(Length::Fill),
            button(MD_LINK_VARIANT_PLUS).on_press(MatchPreviewMsg::OpenMatchLinks)
        ];

        container(
            column![
                players_col,
                winner_selector,
                button("Submit").on_press(MatchPreviewMsg::Submit)
            ]
            .spacing(10),
        )
        .padding(10)
        .into()
    }
}
