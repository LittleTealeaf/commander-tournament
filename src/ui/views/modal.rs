use iced::{
    Alignment, Element, Length, Padding,
    widget::{button, column, container, row, scrollable, space, text, text_input},
};

use crate::ui::{Message, TournamentApp};

pub fn error_modal(app: &TournamentApp) -> Element<'_, Message> {
    if let Some(msg) = &app.error {
        let content = column![
            text("⚠ Error").size(18),
            space().height(10),
            text(msg),
            space().height(15),
            button("Close")
                .on_press(Message::CloseError)
                .width(Length::Fill),
        ]
        .spacing(5)
        .width(Length::Fixed(400.0));

        return container(
            container(content)
                .padding(Padding::new(20f32))
                .width(Length::Fixed(400.0)),
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
                column![
                    text("starting_elo"),
                    text_input("", &app.score_starting_elo)
                        .on_input(Message::UpdateScoreStartingElo)
                        .width(Length::Fill)
                ],
                space().width(10),
                column![
                    text("game_points"),
                    text_input("", &app.score_game_points)
                        .on_input(Message::UpdateScoreGamePoints)
                        .width(Length::Fill)
                ],
            ]
            .spacing(8),
            row![
                column![
                    text("elo_pow"),
                    text_input("", &app.score_elo_pow)
                        .on_input(Message::UpdateScoreEloPow)
                        .width(Length::Fill)
                ],
                space().width(10),
                column![
                    text("wr_pow"),
                    text_input("", &app.score_wr_pow)
                        .on_input(Message::UpdateScoreWrPow)
                        .width(Length::Fill)
                ],
            ]
            .spacing(8),
            row![
                column![
                    text("elo_weight"),
                    text_input("", &app.score_elo_weight)
                        .on_input(Message::UpdateScoreEloWeight)
                        .width(Length::Fill)
                ],
                space().width(10),
                column![
                    text("wr_weight"),
                    text_input("", &app.score_wr_weight)
                        .on_input(Message::UpdateScoreWrWeight)
                        .width(Length::Fill)
                ],
            ]
            .spacing(8),
            space().height(8),
            text("MatchmakerConfig").size(14),
            row![
                column![
                    text("weight_least_played"),
                    text_input("", &app.match_weight_least_played)
                        .on_input(Message::UpdateMatchWeightLeastPlayed)
                        .width(Length::Fill)
                ],
                space().width(10),
                column![
                    text("weight_nemesis"),
                    text_input("", &app.match_weight_nemesis)
                        .on_input(Message::UpdateMatchWeightNemesis)
                        .width(Length::Fill)
                ],
            ]
            .spacing(8),
            row![
                column![
                    text("weight_neighbor"),
                    text_input("", &app.match_weight_neighbor)
                        .on_input(Message::UpdateMatchWeightNeighbor)
                        .width(Length::Fill)
                ],
                space().width(10),
                column![
                    text("weight_wr_neighbor"),
                    text_input("", &app.match_weight_wr_neighbor)
                        .on_input(Message::UpdateMatchWeightWrNeighbor)
                        .width(Length::Fill)
                ],
            ]
            .spacing(8),
            column![
                text("weight_lost_with"),
                text_input("", &app.match_weight_lost_with)
                    .on_input(Message::UpdateMatchWeightLostWith)
                    .width(Length::Fill)
            ],
            space().height(15),
            row![
                button("Save")
                    .on_press(Message::SaveConfig)
                    .width(Length::Fill),
                space().width(10),
                button("Cancel")
                    .on_press(Message::ShowConfig(false))
                    .width(Length::Fill)
            ]
        ]
        .spacing(5)
        .width(Length::Fixed(640.0));

        return container(
            container(content)
                .padding(Padding::new(20f32))
                .width(Length::Fixed(640.0)),
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
            text("Paste tab-separated values (Player1\\tPlayer2\\tPlayer3\\tPlayer4\\tWinner):")
                .size(11),
            space().height(8),
            scrollable(
                text_input(
                    "Player1\tPlayer2\tPlayer3\tPlayer4\tWinner\n...",
                    &app.ingest_text
                )
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
                .width(Length::Fixed(700.0)),
        )
        .align_y(Alignment::Center)
        .align_x(Alignment::Center)
        .width(Length::Fill)
        .height(Length::Fill)
        .into();
    }
    container(text("")).into()
}

pub fn export_modal(app: &TournamentApp) -> Element<'_, Message> {
    if app.show_export {
        let content = column![
            text("Export Game Data (TSV)").size(18),
            space().height(10),
            text("Copy tab-separated values (Player1\tPlayer2\tPlayer3\tPlayer4\tWinner):")
                .size(11),
            space().height(8),
            scrollable(
                text_input(
                    "Player1\tPlayer2\tPlayer3\tPlayer4\tWinner\n...",
                    &app.export_text
                )
                .on_input(Message::UpdateExport)
                .padding(10)
                .width(Length::Fill)
            )
            .height(Length::Fixed(300.0))
            .width(Length::Fill),
            space().height(15),
            row![
                button("Close")
                    .on_press(Message::ShowExport(false))
                    .width(Length::Fill),
            ]
            .spacing(5)
        ]
        .spacing(5)
        .width(Length::Fixed(700.0));

        return container(
            container(content)
                .padding(Padding::new(20f32))
                .width(Length::Fixed(700.0)),
        )
        .align_y(Alignment::Center)
        .align_x(Alignment::Center)
        .width(Length::Fill)
        .height(Length::Fill)
        .into();
    }
    container(text("")).into()
}

pub fn game_winner_modal(app: &TournamentApp) -> Element<'_, Message> {
    if let Some(index) = app.selected_game_index
        && let Some(game) = app.tournament.games().get(index)
    {
        let mut buttons = column![];
        for player in game.players.iter() {
            let p = player.clone();
            buttons = buttons.push(
                button(text(p.clone()))
                    .on_press(Message::ChangeGameWinner(index, p))
                    .width(Length::Fill),
            );
        }

        let content = column![
            text(format!("Change winner for game #{}", index)).size(18),
            space().height(10),
            buttons,
            space().height(15),
            row![button("Cancel").on_press(Message::OpenChangeWinnerModal(usize::MAX)),].spacing(5)
        ]
        .spacing(5)
        .width(Length::Fixed(400.0));

        return container(
            container(content)
                .padding(Padding::new(20f32))
                .width(Length::Fixed(400.0)),
        )
        .align_y(Alignment::Center)
        .align_x(Alignment::Center)
        .width(Length::Fill)
        .height(Length::Fill)
        .into();
    }
    container(text("")).into()
}
pub fn player_info_modal(app: &TournamentApp) -> Element<'_, Message> {
    if let Some(player) = &app.show_player_info {
        let is_new = player.is_empty();
        let title = if is_new {
            "New Deck".to_string()
        } else {
            format!("Edit Deck: {}", player)
        };

        let colors_row = {
            use crate::tournament::MtgColor;
            let colors = [
                (MtgColor::White, "W"),
                (MtgColor::Blue, "U"),
                (MtgColor::Black, "B"),
                (MtgColor::Red, "R"),
                (MtgColor::Green, "G"),
            ];

            let mut row_content = row![];
            for (color, label) in colors.iter() {
                let is_selected = app.player_info_colors.contains(color);
                let label_text = if is_selected {
                    format!("{} ✓", label)
                } else {
                    label.to_string()
                };

                let btn = button(text(label_text))
                    .on_press(Message::TogglePlayerColor(*color))
                    .width(Length::Fixed(40.0));

                row_content = row_content.push(btn);
            }
            row_content.spacing(5)
        };

        let content = column![
            text(title).size(18),
            space().height(10),
            text("Deck Name:"),
            text_input("Enter deck name...", &app.player_info_name)
                .on_input(Message::SetPlayerName)
                .padding(5)
                .width(Length::Fill),
            space().height(5),
            text("Description:"),
            text_input("Enter deck description...", &app.player_info_description)
                .on_input(Message::SetPlayerDescription)
                .padding(5)
                .width(Length::Fill),
            space().height(5),
            text("Moxfield Link:"),
            text_input("https://moxfield.com/decks/...", &app.player_info_moxfield_link)
                .on_input(Message::SetPlayerMoxfieldLink)
                .padding(5)
                .width(Length::Fill),
            space().height(5),
            text("Colors:"),
            colors_row,
            space().height(15),
            row![
                button("Save").on_press(Message::SavePlayerDetails).width(Length::Fill),
                button("Cancel").on_press(Message::ClosePlayerInfo).width(Length::Fill),
            ]
            .spacing(5)
        ]
        .spacing(5)
        .width(Length::Fixed(500.0))
        .padding(20);

        return container(
            container(content)
                .padding(Padding::new(20f32))
                .width(Length::Fixed(500.0)),
        )
        .align_y(Alignment::Center)
        .align_x(Alignment::Center)
        .width(Length::Fill)
        .height(Length::Fill)
        .into();
    }
    container(text("")).into()
}
