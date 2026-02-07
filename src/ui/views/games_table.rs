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
                let items = r.players.iter().cloned().collect::<Vec<_>>();
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

    container(scrollable(table_widget))
        .padding(Padding::new(10f32))
        .into()
}
