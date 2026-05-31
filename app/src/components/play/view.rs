use edh_tourn::{
    game::{POD_SIZE, match_player::MatchPlayer},
    player::RegisteredPlayer,
    tournament::Tournament,
};
use iced::{
    Alignment, Background, Border, Element, Length, Theme,
    widget::{button, column, container, pick_list, row, space, table, text},
};
use itertools::Itertools;
use nerd_font_symbols::md::{MD_CARDS, MD_LINK_VARIANT, MD_LINK_VARIANT_PLUS, MD_TROPHY};

use crate::{
    components::play::{PlayComponent, PlayComponentMsg, PlayMode},
    traits::ComponentView,
};

#[derive(Clone)]
struct PlayerEntry<'a> {
    row: usize,
    player: Option<RegisteredPlayer<'a>>,
    matchup: Option<MatchPlayer>,
}

impl ComponentView for PlayComponent {
    type ViewContext<'a>
        = &'a Tournament
    where
        Self: 'a;

    fn view<'a>(&'a self, context: Self::ViewContext<'a>) -> iced::Element<'a, Self::Message> {
        container(
            column![
                self.view_section_players(context),
                self.view_section_submit(context)
            ]
            .spacing(10),
        )
        .into()
    }
}

impl PlayComponent {
    fn view_section_players<'a>(&self, tournament: &'a Tournament) -> Element<'a, PlayComponentMsg> {
        let modifyable = matches!(self.mode, PlayMode::Custom(_));

        let select_players = if modifyable {
            tournament
                .registered_players()
                .sorted_by(|a, b| {
                    a.info()
                        .name()
                        .cmp(b.info().name())
                        .then_with(|| a.id().cmp(&b.id()))
                })
                .collect()
        } else {
            Vec::new()
        };

        table(
            [
                table::column(text("Player"), |entry: PlayerEntry<'_>| {
                    row![
                        button(MD_CARDS)
                            .on_press_maybe(entry.player.map(|p| PlayComponentMsg::ClickPlayer(p.id())))
                            .style(button::text),
                        if modifyable {
                            container(
                                pick_list(select_players.clone(), entry.player, move |player| {
                                    PlayComponentMsg::SetPlayer(entry.row, player.id())
                                })
                                .width(Length::Fill),
                            )
                        } else if let Some(player) = entry.player {
                            container(text(player.info().name().to_owned()))
                                .padding(button::DEFAULT_PADDING)
                                .style(|theme: &Theme| {
                                    let palette = theme.extended_palette();
                                    container::Style {
                                        background: Some(Background::Color(palette.background.weaker.color)),
                                        border: Border {
                                            radius: 2.0.into(),
                                            width: 1.0,
                                            color: palette.background.strong.color,
                                        },
                                        ..Default::default()
                                    }
                                })
                        } else {
                            container(text("")).padding(button::DEFAULT_PADDING)
                        }
                        .width(Length::Fill)
                    ]
                })
                .width(Length::Fill),
                table::column(text("Stats"), |entry: PlayerEntry<'_>| {
                    let Some(player) = entry.matchup else {
                        return text("");
                    };

                    let stats = player.stats();

                    let str_wr = stats.wr().map_or_else(
                        || "--% WR".to_owned(),
                        |wr| format!("{}% WR", (wr * 100.0).round()),
                    );
                    text(format!("{:.0} Elo, {str_wr}", stats.elo()))
                }),
                table::column(text("Expected"), |entry: PlayerEntry<'_>| {
                    let Some(player) = entry.matchup else {
                        return text("");
                    };

                    text(format!(
                        "{}% (+{}/-{})",
                        (player.expected() * 100f64).round(),
                        player.elo_win().round(),
                        player.elo_loss().round()
                    ))
                }),
                table::column(
                    button(MD_LINK_VARIANT_PLUS)
                        .on_press_maybe(self.preview.is_some().then_some(PlayComponentMsg::OpenMatchLinks)),
                    |entry: PlayerEntry<'_>| {
                        button(MD_LINK_VARIANT).on_press_maybe(
                            entry
                                .player
                                .and_then(|player| player.info().moxfield_goldfish_link())
                                .map(PlayComponentMsg::OpenLink),
                        )
                    },
                ),
            ],
            self.player_entries(tournament),
        )
        .width(Length::Fill)
        .into()
    }

    fn player_entries<'a>(&self, tournament: &'a Tournament) -> [PlayerEntry<'a>; POD_SIZE] {
        let matchup = self.preview.as_ref().map(|preview| &preview.matchup);
        let mut row = 0;

        match &self.mode {
            PlayMode::Custom(players) => players.map(|id| {
                let entry = PlayerEntry {
                    row,
                    matchup: id.zip(matchup).and_then(|(i, m)| m.get_player(i)).cloned(),
                    player: id.and_then(|id| tournament.get_registered_player(id)),
                };
                row += 1;
                entry
            }),
            _ => matchup
                .map_or([const { None }; POD_SIZE], |m| m.players().clone().map(Some))
                .map(|player| {
                    let entry = PlayerEntry {
                        row,
                        player: player
                            .as_ref()
                            .and_then(|p| tournament.get_registered_player(p.id())),
                        matchup: player,
                    };
                    row += 1;
                    entry
                }),
        }
    }

    fn view_section_submit<'a>(&self, tournament: &'a Tournament) -> Option<Element<'a, PlayComponentMsg>> {
        let Some(preview) = &self.preview else { return None };
        let players = tournament
            .get_registered_players(preview.matchup.ids())
            .collect::<Vec<_>>();

        let winner = preview.winner.and_then(|id| tournament.get_registered_player(id));

        Some(
            row![
                container(text(MD_TROPHY).align_y(Alignment::Center)).padding(button::DEFAULT_PADDING),
                pick_list(players, winner, |player| PlayComponentMsg::SetWinner(player.id())),
                space().width(Length::Fill),
                button(text("Submit")).on_press_maybe(winner.is_some().then_some(PlayComponentMsg::Submit))
            ]
            .padding(10)
            .into(),
        )
    }
}
