use iced::{
    Alignment, Element, Length, Padding,
    widget::{button, column, row, space, table, text, text_input},
};

use crate::{
    tournament::PlayerStats,
    ui::{TournamentApp, message::Message},
};

pub fn view(app: &TournamentApp) -> Element<'_, Message> {
    if let Some(name) = &app.new_player_name {
        return row![
            text_input("Player Name...", name)
                .on_input(|s| { Message::NewPlayerSetName(Some(s)) })
                .on_submit(Message::NewPlayerSubmit),
            button("Submit").on_press(Message::NewPlayerSubmit),
            button("Cancel").on_press(Message::NewPlayerSetName(None)),
            space().width(Length::Fill),
        ]
        .align_y(Alignment::Center)
        .into();
    }

    column![
        row![button("Load"), button("Save"), button("New Player")]
            .width(Length::Fill)
        row![leaderboard(app)]
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

    table(
        [
            table::column("Deck", |p: Player<'_>| text(p.name)),
            table::column("Elo", |p: Player<'_>| text(p.stats.elo)),
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
        ],
        app.tournament
            .players()
            .map(|(name, stats)| Player { name, stats }),
    )
    .into()
}
