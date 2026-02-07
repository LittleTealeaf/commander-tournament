use iced::{
    Alignment, Element, Length, Padding,
    widget::{button, column, container, row, space, text, text_input, scrollable},
};

use crate::ui::{Message, TournamentApp};

pub fn error_modal(app: &TournamentApp) -> Element<'_, Message> {
    if let Some(msg) = &app.error {
        let content = column![
            text("⚠ Error").size(18),
            space().height(10),
            text(msg),
            space().height(15),
            button("Close").on_press(Message::CloseError).width(Length::Fill),
        ]
        .spacing(5)
        .width(Length::Fixed(400.0));

        return container(
            container(content)
                .padding(Padding::new(20f32))
                .width(Length::Fixed(400.0))
        )
        .align_y(Alignment::Center)
        .align_x(Alignment::Center)
        .width(Length::Fill)
        .height(Length::Fill)
        .into();
    }
    container(text("")).into()
}

pub fn player_name_modal(app: &TournamentApp) -> Element<'_, Message> {
    if let Some((old, new)) = &app.change_player_name {
        let title = match old {
            Some(old) => format!("Renaming {old}"),
            None => String::from("New Deck"),
        };

        let content = column![
            text(title).size(18),
            space().height(10),
            text_input("Enter deck name...", new)
                .on_input(|s| Message::SetChangePlayerName(Some((old.clone(), s))))
                .on_submit(Message::ChangePlayerSubmit)
                .padding(10)
                .width(Length::Fill),
            space().height(15),
            row![
                button("Submit")
                    .on_press(Message::ChangePlayerSubmit)
                    .width(Length::Fill),
                space().width(10),
                button("Cancel")
                    .on_press(Message::SetChangePlayerName(None))
                    .width(Length::Fill),
                space().width(10),
                button("Delete")
                    .on_press_maybe(
                        old.as_ref()
                            .map(|old| { Message::DeletePlayer(old.clone()) })
                    )
                    .width(Length::Fill),
            ]
            .spacing(5)
        ]
        .spacing(5)
        .width(Length::Fixed(400.0));

        return container(
            container(content)
                .padding(Padding::new(20f32))
                .width(Length::Fixed(400.0))
        )
        .align_y(Alignment::Center)
        .align_x(Alignment::Center)
        .width(Length::Fill)
        .height(Length::Fill)
        .into();
    }
    container(text("")).into()
}

pub fn config_modal(app: &TournamentApp) -> Element<'_, Message> {
    if app.show_config {
        let content = column![
            text("Config").size(18),
            space().height(10),
            text("ScoreConfig").size(14),
            row![
                column![text("starting_elo"), text_input("", &app.score_starting_elo).on_input(|s| Message::UpdateScoreStartingElo(s)).width(Length::Fill)],
                space().width(10),
                column![text("game_points"), text_input("", &app.score_game_points).on_input(|s| Message::UpdateScoreGamePoints(s)).width(Length::Fill)],
            ]
            .spacing(8),
            row![
                column![text("elo_pow"), text_input("", &app.score_elo_pow).on_input(|s| Message::UpdateScoreEloPow(s)).width(Length::Fill)],
                space().width(10),
                column![text("wr_pow"), text_input("", &app.score_wr_pow).on_input(|s| Message::UpdateScoreWrPow(s)).width(Length::Fill)],
            ]
            .spacing(8),
            row![
                column![text("elo_weight"), text_input("", &app.score_elo_weight).on_input(|s| Message::UpdateScoreEloWeight(s)).width(Length::Fill)],
                space().width(10),
                column![text("wr_weight"), text_input("", &app.score_wr_weight).on_input(|s| Message::UpdateScoreWrWeight(s)).width(Length::Fill)],
            ]
            .spacing(8),
            space().height(8),
            text("MatchmakerConfig").size(14),
            row![
                column![text("weight_least_played"), text_input("", &app.match_weight_least_played).on_input(|s| Message::UpdateMatchWeightLeastPlayed(s)).width(Length::Fill)],
                space().width(10),
                column![text("weight_nemesis"), text_input("", &app.match_weight_nemesis).on_input(|s| Message::UpdateMatchWeightNemesis(s)).width(Length::Fill)],
            ]
            .spacing(8),
            row![
                column![text("weight_neighbor"), text_input("", &app.match_weight_neighbor).on_input(|s| Message::UpdateMatchWeightNeighbor(s)).width(Length::Fill)],
                space().width(10),
                column![text("weight_wr_neighbor"), text_input("", &app.match_weight_wr_neighbor).on_input(|s| Message::UpdateMatchWeightWrNeighbor(s)).width(Length::Fill)],
            ]
            .spacing(8),
            column![text("weight_lost_with"), text_input("", &app.match_weight_lost_with).on_input(|s| Message::UpdateMatchWeightLostWith(s)).width(Length::Fill)],
            space().height(15),
            row![
                button("Save").on_press(Message::SaveConfig).width(Length::Fill),
                space().width(10),
                button("Cancel").on_press(Message::ShowConfig(false)).width(Length::Fill)
            ]
        ]
        .spacing(5)
        .width(Length::Fixed(640.0));

        return container(
            container(content)
                .padding(Padding::new(20f32))
                .width(Length::Fixed(640.0))
        )
        .align_y(Alignment::Center)
        .align_x(Alignment::Center)
        .width(Length::Fill)
        .height(Length::Fill)
        .into();
    }
    container(text("")).into()
}

pub fn ingest_modal(app: &TournamentApp) -> Element<'_, Message> {
    if app.show_ingest {
        let content = column![
            text("Import Game Data").size(18),
            space().height(10),
            text("Paste tab-separated values (Player1\\tPlayer2\\tPlayer3\\tPlayer4\\tWinner):").size(11),
            space().height(8),
            scrollable(
                text_input("Player1\tPlayer2\tPlayer3\tPlayer4\tWinner\n...", &app.ingest_text)
                    .on_input(Message::UpdateIngest)
                    .padding(10)
                    .width(Length::Fill)
            )
            .height(Length::Fixed(300.0))
            .width(Length::Fill),
            space().height(15),
            row![
                button("Import")
                    .on_press(Message::SubmitIngest)
                    .width(Length::Fill),
                space().width(10),
                button("Cancel")
                    .on_press(Message::ShowIngest(false))
                    .width(Length::Fill),
            ]
            .spacing(5)
        ]
        .spacing(5)
        .width(Length::Fixed(700.0));

        return container(
            container(content)
                .padding(Padding::new(20f32))
                .width(Length::Fixed(700.0))
        )
        .align_y(Alignment::Center)
        .align_x(Alignment::Center)
        .width(Length::Fill)
        .height(Length::Fill)
        .into();
    }
    container(text("")).into()
}
