use iced::{
    Element, Length, Padding,
    widget::{button, column, container, pick_list, row, space, text},
};
use itertools::Itertools;

use crate::ui::{Message, TournamentApp};

pub fn game_input(app: &TournamentApp) -> Element<'_, Message> {
    // Build a two-by-two grid of player selectors (Player 1..4)
    let player_choices = app
        .tournament
        .players()
        .keys()
        .cloned()
        .sorted()
        .collect::<Vec<_>>();

    let make_player_card = |i: usize| {
        let (percent, elo_change) = app
            .selected_match
            .as_ref()
            .map(|gm| (
                format!("{:.1}%", gm.0[i].expected() * 100.0),
                format!("+{:.1} / -{:.1}", gm.0[i].elo_win(), gm.0[i].elo_loss()),
            ))
            .unwrap_or_default();



        let stats_line = app
            .selected_match
            .as_ref()
            .map(|gm| {
                let elo = format!("{:.0}", gm.0[i].stats().elo());
                let wr_text = gm.0[i]
                    .stats()
                    .wr()
                    .map(|w| format!("{:.1}%", w * 100.0))
                    .unwrap_or_else(|| "-".to_string());
                format!("{} ({})", elo, wr_text)
            })
            .unwrap_or_default();

        let player_name = app.selected_players[i].clone();

        let has_moxfield_link = player_name
            .as_ref()
            .and_then(|name| app.tournament.player_details(name))
            .and_then(|details| details.moxfield_goldfish_link())
            .is_some();

        let moxfield_button = if has_moxfield_link {
            let name = player_name.clone().unwrap_or_default();
            button(text("🔗").size(14))
                .on_press(Message::ShowPlayerInfo(name))
                .padding(4)
                .width(iced::Length::Fixed(28.0))
        } else {
            button(text("🔗").size(14))
                .padding(4)
                .width(iced::Length::Fixed(28.0))
        };

        let picker_row = row![
            pick_list(
                player_choices.clone(),
                app.selected_players[i].clone(),
                move |choice| Message::SelectPlayer(i, Some(choice)),
            )
            .width(Length::Fill),
            space().width(4),
            moxfield_button
        ]
        .align_y(iced::Alignment::Center)
        .spacing(4);

        column![
            row![
                text(format!("Player {}", i + 1)).size(12),
                space().width(Length::Fill),
                text(percent).size(12),
                space().width(5),
                text(elo_change).size(12)
            ]
            .align_y(iced::Alignment::Center)
            .width(Length::Fill),
            picker_row.width(Length::Fill),
            text(stats_line).size(12).width(Length::Fill),
        ]
        .spacing(6)
        .width(Length::Fill)
        .into()
    };

    let left_col = column([
        make_player_card(0),
        space().height(8).into(),
        make_player_card(2),
    ])
    .spacing(12);

    let right_col = column([
        make_player_card(1),
        space().height(8).into(),
        make_player_card(3),
    ])
    .spacing(12);

    let player_inputs = row![
        left_col.width(Length::Fill),
        space().width(10),
        right_col.width(Length::Fill)
    ]
    .spacing(10)
    .width(Length::Fill);

    let winner_row = row![
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
        space().width(8),
        button("Submit Result")
            .on_press_maybe(
                (app.selected_match.is_some() && app.selected_winner.is_some())
                    .then_some(Message::SubmitGame)
            )
            .width(Length::Fixed(140.0)),
        space().width(8),
        button("Clear")
            .on_press(Message::ClearGame)
            .width(Length::Fixed(80.0)),
    ]
    .spacing(8)
    .align_y(iced::Alignment::Center)
    .width(Length::Fill);

    let inner = column![
        row![text("Record Game").size(16), space().width(8)],
        space().height(6),
        player_inputs,
        space().height(12),
        winner_row,
    ]
    .spacing(8)
    .width(Length::Fill);

    container(inner)
        .padding(Padding::new(15f32))
        .width(Length::Fill)
        .into()
}
