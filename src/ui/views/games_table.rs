use iced::{
    Alignment, Element, Length, Padding,
    widget::{button, container, pick_list, row, scrollable, space, table, text},
};

use crate::ui::{Message, TournamentApp};

pub fn games_table(app: &TournamentApp) -> Element<'_, Message> {
    #[derive(Clone)]
    struct GameRow<'a> {
        index: usize,
        players: &'a [String; 4],
        winner: &'a String,
        app: &'a TournamentApp,
    }

    let table_widget = table(
        [
            table::column("#", |r: GameRow<'_>| text(format!("{}", r.index)).size(12)),
            table::column("Players", |r: GameRow<'_>| {
                let mut player_row = row![];
                for player_name in r.players.iter() {
                    let has_moxfield = r.app.tournament.player_details(player_name)
                        .and_then(|d| d.moxfield_goldfish_link())
                        .is_some();

                    let player_text = text(player_name).size(12);
                    player_row = player_row.push(player_text);

                    if has_moxfield {
                        let player_clone = player_name.clone();
                        player_row = player_row.push(
                            button(text("🔗").size(12))
                                .on_press(Message::ShowPlayerInfo(player_clone))
                                .padding(2)
                                .width(Length::Fixed(24.0))
                                .height(Length::Fixed(24.0))
                        );
                    }

                    player_row = player_row.push(space().width(8));
                }
                player_row.spacing(4).align_y(Alignment::Center)
            }),
            table::column("Winner", |r: GameRow<'_>| {
                let items = r.players.to_vec();
                let idx = r.index;
                pick_list(items, Some(r.winner.clone()), move |s: String| {
                    Message::ChangeGameWinner(idx, s)
                })
            }),
            table::column("", |r: GameRow<'_>| {
                let idx = r.index;
                button(text("Delete"))
                    .on_press_with(move || Message::DeleteGame(idx))
                    .padding(4)
                    .width(Length::Fixed(70.0))
            }),
        ],
        app.tournament
            .games()
            .iter()
            .enumerate()
            .map(|(i, g)| GameRow {
                index: i,
                players: &g.players,
                winner: &g.winner,
                app,
            }),
    );
    // Ensure the table expands to fill available width by wrapping it in a container
    let table_widget = container(table_widget).width(Length::Fill);
    let content = iced::widget::column![
        crate::ui::views::toolbar::toolbar(),
        iced::widget::space().height(5),
        container(scrollable(table_widget).width(Length::Fill))
            .padding(Padding::new(10f32))
            .width(Length::Fill)
            .height(Length::Fill),
        iced::widget::row![
            iced::widget::space().width(Length::Fill),
            button(text("Close"))
                .on_press(Message::ShowGames(false))
                .width(Length::Fixed(120.0)),
        ]
        .spacing(10)
        .width(Length::Fill),
    ]
    .spacing(8)
    .width(Length::Fill)
    .height(Length::Fill);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
