use iced::{
    Alignment, Element, Length, Padding,
    application::IntoBoot,
    widget::{
        button, column, container, pick_list, row, rule, scrollable, space, table, text, text_input,
    },
};
use itertools::Itertools;

use crate::{
    tournament::PlayerStats,
    ui::{TournamentApp, message::Message},
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

    if let Some((old, new)) = &app.change_player_name {
        return container(column![
            text(match old {
                Some(old) => format!("Renaming {old}"),
                None => String::from("New Deck"),
            }),
            text_input("Name...", new)
                .on_input(|s| Message::SetChangePlayerName(Some((old.clone(), s))))
                .on_submit(Message::ChangePlayerSubmit),
            row![
                button("Submit").on_press(Message::ChangePlayerSubmit),
                space().width(5),
                button("Cancel").on_press(Message::SetChangePlayerName(None)),
                space().width(20),
                button("Delete").on_press_maybe(
                    old.as_ref()
                        .map(|old| { Message::DeletePlayer(old.clone()) })
                )
            ]
        ])
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .into();
    }

    column![
        row![
            button("Config").on_press(Message::ShowConfig(true)),
            button("New Player")
                .on_press(Message::SetChangePlayerName(Some((None, String::new())))),
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
            column![game_input(app), rule::horizontal(1)]
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
            table::column("Elo", |p: Player<'_>| text(format!("{:.0}", p.stats.elo))),
            table::column("Games", |p: Player<'_>| text(p.stats.games)),
            table::column("Wins", |p: Player<'_>| text(p.stats.wins)),
            table::column("Winrate", |p: Player<'_>| {
                text({
                    if p.stats.games == 0 {
                        String::new()
                    } else {
                        let wr = (p.stats.wins as f32) / (p.stats.games as f32);
                        format!("{:.1}%", wr * 100.0)
                    }
                })
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
            .sorted_by(|a, b| a.stats.elo.total_cmp(&b.stats.elo))
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
                    .push(text(format!("{:.1}%", gm.0[i].expected * 100.0)));
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


fn matches(app: &TournamentApp) -> Element<'_, Message> {


    container(
        column![

        


        ]
    ).into()
}
