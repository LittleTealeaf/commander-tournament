use iced::{
    Element, Length, Padding,
    widget::{button, column, container, pick_list, row, space, text},
};
use itertools::Itertools;

use crate::ui::{Message, TournamentApp, state::MatchupType};

pub fn game_matchups(app: &TournamentApp) -> Element<'_, Message> {
    let player = app.match_player.clone();
    let game = player.and_then(|p| {
        // Collect ranked players, then remove the focus player and deduplicate while keeping order
        let players: Vec<String> = match app.matchup_type {
            MatchupType::Combined => app.tournament.rank_combined(&p).ok()?.collect(),
            MatchupType::LeastPlayed => app.tournament.rank_least_played(&p).ok()?.collect(),
            MatchupType::Nemesis => app.tournament.rank_nemesis(&p).ok()?.collect(),
            MatchupType::WinrateNeighbors => app.tournament.rank_wr_neighbors(&p).ok()?.collect(),
            MatchupType::Neighbors => app.tournament.rank_neighbors(&p).ok()?.collect(),
        };

        let filtered: Vec<String> = players
            .into_iter()
            .filter(|name| name != &p)
            .unique()
            .collect();

        if filtered.len() >= 3 {
            Some([p, filtered[0].clone(), filtered[1].clone(), filtered[2].clone()])
        } else {
            None
        }
    });

    let game_vec: Vec<_> = game.as_ref().map(|g| g.to_vec()).unwrap_or_default();

    let player_display: Element<Message> = if game_vec.is_empty() {
        text("No recommendation").into()
    } else {
        column(
            game_vec
                .iter()
                .enumerate()
                .map(|(i, p)| {
                    let label = match i {
                        0 => "P1:",
                        1 => "P2:",
                        2 => "P3:",
                        3 => "P4:",
                        _ => "P?",
                    };
                    {
                        let stats_opt = app.tournament.players().get(p);
                        if let Some(stats) = stats_opt {
                            let wr_text = stats
                                .wr()
                                .map(|w| format!("{:.1}%", w * 100.0))
                                .unwrap_or_else(|| "-".to_string());
                            row![
                                text(label).width(30),
                                text(p.clone()),
                                space().width(8),
                                text(format!("{:.0} / {}", stats.elo(), wr_text)),
                            ]
                        } else {
                            row![text(label).width(30), text(p.clone())]
                        }
                    }
                    .spacing(10)
                    .into()
                })
                .collect::<Vec<_>>()
        )
        .spacing(6)
        .into()
    };

    let inner = column![
        text("Matchup Recommendation:").size(14),
        pick_list(
            app.tournament
                .players()
                .keys()
                .cloned()
                .sorted()
                .collect::<Vec<_>>(),
            app.match_player.clone(),
            Message::SelectMatchPlayer
        )
        .width(Length::Fill),
        space().height(8),
        text("Algorithm:").size(12),
        pick_list(
            MatchupType::all().to_vec(),
            Some(app.matchup_type),
            Message::SetMatchupType
        )
        .width(Length::Fill)
        .text_size(12),
        space().height(8),
        player_display,
        space().height(10),
        button("Load Matchup")
            .on_press_maybe(game.map(Message::SelectPlayers))
            .width(Length::Fill),
    ]
    .spacing(8)
    .width(Length::Fill);

    container(inner)
        .padding(Padding::new(15f32))
        .width(Length::Fill)
        .into()
}
