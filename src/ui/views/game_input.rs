use iced::{
    Element, Length, Padding,
    widget::{button, column, container, pick_list, row, space, text},
};
use itertools::Itertools;

use crate::ui::{Message, TournamentApp};

pub fn game_input(app: &TournamentApp) -> Element<'_, Message> {
    let player_inputs = column((0..4).map(|i| {
        let mut player_row = row!(
            pick_list(
                app.tournament
                    .players()
                    .keys()
                    .cloned()
                    .sorted()
                    .collect::<Vec<_>>(),
                app.selected_players[i].clone(),
                move |choice| Message::SelectPlayer(i, Some(choice)),
            )
            .width(Length::Fill),
        );
        if let Some(gm) = &app.selected_match {
            let percent = format!("{:.1}%", gm.0[i].expected() * 100.0);
            let elo = format!("{:.0}", gm.0[i].stats().elo());
            let wr_text = gm.0[i]
                .stats()
                .wr()
                .map(|w| format!("{:.1}%", w * 100.0))
                .unwrap_or_else(|| "-".to_string());

            player_row = player_row
                .push(space().width(10))
                .push(
                    column![
                        text(percent).size(12),
                        text(format!("{} / {}", elo, wr_text)).size(12),
                    ]
                    .spacing(2)
                    .width(Length::Fixed(120.0)),
                );
        }
        player_row.spacing(10).into()
    }))
    .spacing(8);

    let winner_row = row![
        text("Winner:").width(60),
        pick_list(
            app.selected_players
                .iter()
                .flatten()
                .cloned()
                .collect::<Vec<_>>(),
            app.selected_winner.clone(),
            Message::SelectWinner
        )
        .width(Length::Fill),
    ]
    .spacing(10)
    .align_y(iced::Alignment::Center);

    let submit_button = button("Submit Game").on_press_maybe(
        (app.selected_match.is_some() && app.selected_winner.is_some())
            .then_some(Message::SubmitGame)
    );

    let inner = column![
        text("Players:").size(14),
        player_inputs,
        space().height(10),
        winner_row,
        space().height(10),
        submit_button.width(Length::Fill),
    ]
    .spacing(5)
    .width(Length::Fill);

    container(inner)
        .padding(Padding::new(15f32))
        .width(Length::Fill)
        .into()
}
