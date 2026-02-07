use iced::{
    Alignment, Element, Length, Padding,
    widget::{
        button, column, container, pick_list, row, rule, scrollable, space, table, text, text_input,
    },
};
use itertools::Itertools;

use crate::{
    tournament::PlayerStats,
    ui::{Message, TournamentApp},
};

pub fn view(app: &TournamentApp) -> Element<'_, Message> {
    if let Some(msg) = &app.error {
        return container(row![
            text(format!("Error: {}", msg)),
            button("Close").on_press(Message::CloseError)
        ])
        .align_y(Alignment::Center)
        .align_x(Alignment::Center)
        .into();
    }

    column![
        row![
            button("Config").on_press(Message::ShowConfig(true)),
            button("New Player")
                .on_press(Message::ShowPlayerInfo(String::new())),
            button("Reload").on_press(Message::Reload),
            space().width(Length::Fill),
            button("Ingest").on_press(Message::Ingest),
            button("New").on_press(Message::New),
            button("Load").on_press(Message::Load(String::from("game.ron"))),
            button("Save").on_press(Message::Save(String::from("game.ron"))),
        ]
        .width(Length::Fill)
        .spacing(5),
        row![
            leaderboard(app),
            rule::vertical(1),
            column![game_input(app), rule::horizontal(1), game_matchups(app)]
        ]
    ]
    .padding(Padding::new(10f32))
    .into()
}

fn leaderboard(app: &TournamentApp) -> Element<'_, Message> {
    #[derive(Clone)]
    struct Player<'a> {
        name: &'a String,
        stats: &'a PlayerStats,
    }

    scrollable(table(
        [
            table::column("Deck", |p: Player<'_>| text(p.name)),
            table::column("Elo", |p: Player<'_>| text(format!("{:.0}", p.stats.elo()))),
            table::column("Games", |p: Player<'_>| text(p.stats.games())),
            table::column("Wins", |p: Player<'_>| text(p.stats.wins())),
            table::column("Winrate", |p: Player<'_>| {
                text(
                    p.stats
                        .wr()
                        .map(|wr| format!("{:.1}%", wr * 100.0))
                        .unwrap_or_default(),
                )
            }),
            table::column("Options", |p: Player<'_>| {
                row![
                    button("E").on_press_with(|| Message::SetChangePlayerName(Some((
                        Some(p.name.clone()),
                        p.name.clone()
                    ))))
                ]
            }),
        ],
        app.tournament
            .players()
            .iter()
            .map(|(name, stats)| Player { name, stats })
            .sorted_by(|a, b| a.stats.elo().total_cmp(&b.stats.elo()))
            .rev(),
    ))
    .into()
}

fn game_input(app: &TournamentApp) -> Element<'_, Message> {
    container(row![column![
        text("Players:"),
        column((0..4).map(|i| {
            let mut row = row!(pick_list(
                app.tournament
                    .players()
                    .keys()
                    .cloned()
                    .sorted()
                    .collect::<Vec<_>>(),
                app.selected_players[i].clone(),
                move |choice| Message::SelectPlayer(i, Some(choice)),
            ));
            if let Some(gm) = &app.selected_match {
                row = row
                    .push(space().width(5))
                    .push(text(format!("{:.1}%", gm.0[i].expected() * 100.0)));
            }

            row.into()
        })),
        text("Winner:"),
        row![
            pick_list(
                app.selected_players
                    .iter()
                    .flatten()
                    .cloned()
                    .collect::<Vec<_>>(),
                app.selected_winner.clone(),
                Message::SelectWinner
            ),
            button("Submit Game").on_press_maybe(
                (app.selected_match.is_some() && app.selected_winner.is_some())
                    .then_some(Message::SubmitGame)
            )
        ]
    ]])
    .into()
}

fn game_matchups(app: &TournamentApp) -> Element<'_, Message> {
    let player = app.match_player.clone();
    let game = player.and_then(|p| {
        let mut iter = app.tournament.rank_combined(&p).ok()?;
        let p2 = iter.next()?.clone();
        let p3 = iter.next()?.clone();
        let p4 = iter.next()?.clone();
        drop(iter);

        Some([p, p2, p3, p4])
    });

    let game_vec: Vec<_> = game.as_ref().map(|g| g.to_vec()).unwrap_or_default();

    let player_names = game_vec
        .iter()
        .cloned()
        .map(|p| text(p).into())
        .collect::<Vec<Element<Message>>>();

    container(column![
        pick_list(
            app.tournament
                .players()
                .keys()
                .cloned()
                .sorted()
                .collect::<Vec<_>>(),
            app.match_player.clone(),
            Message::SelectMatchPlayer
        ),
        row![
            text("Combined"),
            row(player_names),
            button("Load").on_press_maybe(game.map(Message::SelectPlayers))
        ]
    ])
    .into()
}
