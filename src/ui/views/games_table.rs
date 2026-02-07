use iced::{Element, Length, Padding, widget::{button, container, scrollable, table, text, pick_list}};
use itertools::Itertools;

use crate::ui::{Message, TournamentApp};

pub fn games_table(app: &TournamentApp) -> Element<'_, Message> {
    #[derive(Clone)]
    struct GameRow<'a> {
        index: usize,
        players: &'a [String; 4],
        winner: &'a String,
    }

    let table_widget = table(
        [
            table::column("#", |r: GameRow<'_>| text(format!("{}", r.index)).size(12)),
            table::column("Players", |r: GameRow<'_>| text(r.players.iter().join(", ")).size(12)),
            table::column("Winner", |r: GameRow<'_>| {
                let items = r.players.to_vec();
                let idx = r.index;
                pick_list(items, Some(r.winner.clone()), move |s: String| Message::ChangeGameWinner(idx, s)).width(Length::Fixed(160.0))
            }),
            table::column("", |r: GameRow<'_>| {
                let idx = r.index;
                button(text("Delete")).on_press_with(move || Message::DeleteGame(idx)).padding(4).width(Length::Fixed(70.0))
            }),
        ],
        app.tournament
            .games()
            .iter()
            .enumerate()
            .map(|(i, g)| GameRow { index: i, players: &g.players, winner: &g.winner }),
    );
    let content = iced::widget::column![
        crate::ui::views::toolbar::toolbar(),
        iced::widget::space().height(5),
        container(scrollable(table_widget)).padding(Padding::new(10f32)).width(Length::Fill).height(Length::Fill),
        iced::widget::row![
            iced::widget::space().width(Length::Fill),
            button(text("Close")).on_press(Message::ShowGames(false)).width(Length::Fixed(120.0)),
        ]
        .spacing(10)
        .width(Length::Fill),
    ]
    .spacing(8)
    .width(Length::Fill)
    .height(Length::Fill);

    container(content).width(Length::Fill).height(Length::Fill).into()
}
