use commander_tournament_core::{
    game::match_player::MatchPlayer, player::RegisteredPlayer, tournament::Tournament,
};
use iced::{
    Alignment, Background, Border, Element, Length, Theme,
    widget::{button, column, container, pick_list, row, space, table, text},
};
use itertools::Itertools;
use nerd_font_symbols::md::{MD_CARDS, MD_CLOSE, MD_LINK_VARIANT, MD_LINK_VARIANT_PLUS, MD_TROPHY};

use iced_tea::Component;

use crate::components::play::{PlayComponent, PlayComponentMsg, PlayMode};

#[derive(Clone)]
struct PlayerEntry<'a> {
    row: usize,
    player: Option<RegisteredPlayer<'a>>,
    matchup: Option<MatchPlayer>,
}

impl Component for PlayComponent {
    fn render<'a>(&'a self, context: Self::Context<'a>) -> iced::Element<'a, Self::Message> {
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

        let entries = self.player_entries(tournament);
        let entry_count = entries.len();

        let tbl = table(
            [
                table::column(text("Player"), move |entry: PlayerEntry<'_>| {
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
                    move |entry: PlayerEntry<'_>| -> Element<'_, PlayComponentMsg> {
                        let link_btn = button(MD_LINK_VARIANT).on_press_maybe(
                            entry
                                .player
                                .and_then(|player| player.info().moxfield_goldfish_link())
                                .map(PlayComponentMsg::OpenLink),
                        );
                        if modifyable && entry_count > 2 {
                            let remove_btn = button(MD_CLOSE)
                                .on_press(PlayComponentMsg::RemovePlayerSlot(entry.row))
                                .style(button::text);
                            row![link_btn, remove_btn].spacing(5).into()
                        } else {
                            link_btn.into()
                        }
                    },
                ),
            ],
            entries,
        )
        .width(Length::Fill);

        if modifyable {
            let add_button = button(text("+ Add Player Slot")).on_press(PlayComponentMsg::AddPlayerSlot);
            column![tbl, add_button].spacing(10).into()
        } else {
            tbl.into()
        }
    }

    fn player_entries<'a>(&self, tournament: &'a Tournament) -> Vec<PlayerEntry<'a>> {
        let matchup = self.preview.as_ref().map(|preview| &preview.matchup);

        match &self.mode {
            PlayMode::Custom(players) => players
                .iter()
                .enumerate()
                .map(|(row, id)| PlayerEntry {
                    row,
                    matchup: id.zip(matchup).and_then(|(i, m)| m.get_player(i)).cloned(),
                    player: id.and_then(|id| tournament.get_registered_player(id)),
                })
                .collect(),
            _ => matchup
                .map(|m| {
                    m.players()
                        .iter()
                        .enumerate()
                        .map(|(row, player)| PlayerEntry {
                            row,
                            player: tournament.get_registered_player(player.id()),
                            matchup: Some(player.clone()),
                        })
                        .collect()
                })
                .unwrap_or_default(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn player_entries_custom_mode() {
        let mut t = Tournament::new();
        let p1 = t.register_player("P1").unwrap();
        let p2 = t.register_player("P2").unwrap();

        // 2 players
        let component = PlayComponent::new(PlayMode::Custom(vec![Some(p1), Some(p2)]), &t);
        let entries = component.player_entries(&t);
        assert_eq!(entries.len(), 2);
        let mut it = entries.into_iter();
        let e0 = it.next().unwrap();
        assert_eq!(e0.row, 0);
        assert_eq!(
            e0.player
                .as_ref()
                .map(commander_tournament_core::player::RegisteredPlayer::id),
            Some(p1)
        );
        assert!(e0.matchup.is_some());
        let e1 = it.next().unwrap();
        assert_eq!(e1.row, 1);
        assert_eq!(
            e1.player
                .as_ref()
                .map(commander_tournament_core::player::RegisteredPlayer::id),
            Some(p2)
        );

        // 3 players with one None slot
        let p3 = t.register_player("P3").unwrap();
        let component = PlayComponent::new(PlayMode::Custom(vec![Some(p1), None, Some(p3)]), &t);
        let entries = component.player_entries(&t);
        assert_eq!(entries.len(), 3);
        let mut it3 = entries.into_iter();
        let e0 = it3.next().unwrap();
        assert_eq!(e0.row, 0);
        assert_eq!(
            e0.player
                .as_ref()
                .map(commander_tournament_core::player::RegisteredPlayer::id),
            Some(p1)
        );
        let e1 = it3.next().unwrap();
        assert_eq!(e1.row, 1);
        assert!(e1.player.is_none());
        assert!(e1.matchup.is_none());
        let e2 = it3.next().unwrap();
        assert_eq!(e2.row, 2);
        assert_eq!(
            e2.player
                .as_ref()
                .map(commander_tournament_core::player::RegisteredPlayer::id),
            Some(p3)
        );
    }

    #[test]
    fn player_entries_next_and_preview_variable_players() {
        let mut t = Tournament::new();
        let p1 = t.register_player("P1").unwrap();
        let _p2 = t.register_player("P2").unwrap();
        let _p3 = t.register_player("P3").unwrap();

        // Next mode with no preview
        let component = PlayComponent::new(PlayMode::next(), &t);
        let entries = component.player_entries(&t);
        assert!(entries.is_empty());

        // Player mode with pod size 2 in preview
        let matchup2 = t.matchmaker().with_pod_size(2).unwrap().create_match(p1).unwrap();
        let mut comp2 = PlayComponent::new(PlayMode::Player(p1), &t);
        comp2.preview = Some(super::super::MatchPreview {
            matchup: matchup2,
            winner: None,
        });
        let entries2 = comp2.player_entries(&t);
        let rows2: Vec<usize> = entries2.iter().map(|e| e.row).collect();
        assert_eq!(rows2, vec![0, 1]);

        // Player mode with pod size 3 in preview
        let matchup3 = t.matchmaker().with_pod_size(3).unwrap().create_match(p1).unwrap();
        let mut comp3 = PlayComponent::new(PlayMode::Player(p1), &t);
        comp3.preview = Some(super::super::MatchPreview {
            matchup: matchup3,
            winner: None,
        });
        let entries3 = comp3.player_entries(&t);
        let rows3: Vec<usize> = entries3.iter().map(|e| e.row).collect();
        assert_eq!(rows3, vec![0, 1, 2]);
    }
}
