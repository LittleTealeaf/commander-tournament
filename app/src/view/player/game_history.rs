use edh_tourn::{Tournament, game::record::GameRecord};
use iced::{
    Length, Padding, font,
    widget::{Container, button, column, container, scrollable, table, text},
};
use itertools::Itertools;

use crate::{fonts::default_font, logic::Message, view::player::ViewPlayerMessage};

pub fn view_game_history(tournament: &Tournament, id: u32) -> Option<Container<'_, Message>> {
    let games = tournament
        .get_player_games(id)
        .ok()?
        .collect_vec()
        .into_iter()
        .rev();

    Some(container(
        scrollable(
            table(
                [
                    table::column("Games", |game: &GameRecord| {
                        column(game.players().iter().map(|player| {
                            let elo = player.stats().elo().round();
                            button(
                                text(tournament.get_player_name(&player.id()).map_or_else(
                                    || format!("({elo}) {}", player.id()),
                                    |name| format!("({elo}) {name}"),
                                ))
                                .font_maybe(
                                    (player.id() == game.winner()).then_some(font::Font {
                                        weight: font::Weight::Bold,
                                        ..default_font()
                                    }),
                                ),
                            )
                            .padding(Padding::new(0.0))
                            .style(button::text)
                            .on_press(ViewPlayerMessage::Open(Some(player.id())).into())
                            .into()
                        }))
                    }),
                    table::column("Elo", |game: &GameRecord| {
                        let elo_change = game.get_player_elo_change(id).unwrap_or_default();
                        let elo_change_str = if elo_change >= 0f64 {
                            format!("+{}", elo_change.round())
                        } else {
                            format!("{}", elo_change.round())
                        };

                        let old_elo = game.get_player(id).map_or_else(
                            || tournament.default_stats().elo(),
                            |player| player.stats().elo(),
                        );

                        let new_elo = (old_elo + elo_change).round();

                        column![
                            text(format!("{new_elo}")).size(20),
                            text(elo_change_str).size(15)
                        ]
                        .spacing(5)
                        .padding(5)
                    }),
                ],
                games,
            )
            .width(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill),
    ))
}
