use edh_tourn::{
    game::{match_player::MatchPlayer, matchup::Matchup, record::GameRecord},
    player::{PlayerId, info::PlayerInfo},
    tournament::Tournament,
};
use iced::{
    Alignment, Length,
    widget::{button, column, container, pick_list, row, table, text},
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
    ClickPlayer(PlayerId),
    OpenMatchLinks,
    Submit,
}

#[derive(Debug, Clone)]
pub enum MatchPreviewOut {
    OpenPlayerInfo(PlayerId),
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
            MatchPreviewMsg::ClickPlayer(player_id) => {
                Effect::out(MatchPreviewOut::OpenPlayerInfo(player_id)).ok()
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
        type RowType<'b> = (&'b MatchPlayer, &'b PlayerInfo);

        let players = self
            .matchup
            .players()
            .iter()
            .filter_map(|player| Some((player, context.get_player_info(&player.id())?)));

        let table = table(
            [
                table::column(text("Player"), |(player, info): RowType| {
                    button(text(info.name()).size(12))
                        .style(button::text)
                        .on_press(MatchPreviewMsg::ClickPlayer(player.id()))
                }),
                table::column(text("Stats"), |(player, _): RowType| {
                    let stats = player.stats();
                    let str_wr = stats.wr().map_or_else(
                        || "--% WR".to_owned(),
                        |wr| format!("{}% WR", (wr * 100.0).round()),
                    );
                    text(format!("{} Elo, {str_wr}", stats.elo().round()))
                }),
                table::column(text("Expected"), |(player, _): RowType| {
                    text(format!(
                        "{}% (+{}/-{})",
                        (player.expected() * 100f64).round(),
                        player.elo_win().round(),
                        player.elo_loss().round()
                    ))
                }),
                table::column(
                    button(MD_LINK_VARIANT_PLUS).on_press(MatchPreviewMsg::OpenMatchLinks),
                    |(_, info): RowType| {
                        button(MD_LINK_VARIANT).on_press_maybe(
                            info.moxfield_goldfish_link().map(MatchPreviewMsg::OpenLink),
                        )
                    },
                ),
            ],
            players,
        )
        .width(Length::Fill);

        let winner_options = self
            .matchup
            .players()
            .iter()
            .filter_map(|player| context.get_registered_player(player.id()))
            .collect_vec();

        let selected_winner = self
            .winner
            .as_ref()
            .and_then(|id| context.get_registered_player(*id));

        let winner_selector = row![
            text("Winner: ").size(27),
            pick_list(winner_options, selected_winner, |picked| {
                MatchPreviewMsg::SelectWinner(picked.id())
            })
            .width(Length::Fill),
            button("Submit")
                .on_press_maybe(self.winner.is_some().then_some(MatchPreviewMsg::Submit))
        ]
        .spacing(10)
        .align_y(Alignment::Center);

        container(column![table, winner_selector]).into()
    }
}
