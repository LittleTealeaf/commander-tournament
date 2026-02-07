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
            MatchupType::LossWith => app.tournament.rank_loss_with(&p).ok()?.collect(),
        };

        let filtered: Vec<String> = players
            .into_iter()
            .filter(|name| name != &p)
            .unique()
            .collect();

        if filtered.len() >= 3 {
            Some([
                p,
                filtered[0].clone(),
                filtered[1].clone(),
                filtered[2].clone(),
            ])
        } else {
            None
        }
    });

    let game_vec: Vec<_> = game.as_ref().map(|g| g.to_vec()).unwrap_or_default();

    // Build a 3-card display showing top 3 match recommendations
    let player_display: Element<Message> = if game_vec.is_empty() {
        text("No recommendation").into()
    } else {
        // Build cards for top 3 opponents (indices 1..3 in game_vec)
        let card_items: Vec<(usize, String)> = (1..4)
            .filter_map(|i| game_vec.get(i).map(|name| (i, name.clone())))
            .collect();

        // helper to build a card element: left (name/elo) | middle (score/rivalry) | right (button)
        let focus = game_vec[0].clone();
        let make_card = |_idx: usize, name: String| {
            let stats_opt = app.tournament.players().get(&name);
            let elo_text = stats_opt
                .map(|s| format!("Elo: {:.0}", s.elo()))
                .unwrap_or_default();

            // Left column: name and elo
            let left_col = column![text(name.clone()).size(22), text(elo_text).size(12),]
                .spacing(4)
                .width(Length::Fill);

            // Compute score and rivalry
            let score_text = {
                let focus_stats = app.tournament.players().get(&focus);
                if let (Some(f), Some(o)) = (focus_stats, stats_opt) {
                    let diff = f.elo() - o.elo();
                    format!("Elo Δ: {:+.0}", diff)
                } else {
                    String::from("Elo Δ: -")
                }
            };

            let rivalry_text = {
                // head-to-head when both present in same game
                let mut games_together = 0usize;
                let mut focus_wins = 0usize;
                for g in app.tournament.games().iter() {
                    if g.players.contains(&focus) && g.players.contains(&name) {
                        games_together += 1;
                        if g.winner == focus {
                            focus_wins += 1;
                        }
                    }
                }
                if games_together == 0 {
                    String::from("H2H: N/A")
                } else {
                    let pct = (focus_wins as f64) / (games_together as f64) * 100.0;
                    format!("H2H: {}/{} ({:.1}%)", focus_wins, games_together, pct)
                }
            };

            // Middle column: score / rivalry score
            let middle_col = column![text(score_text).size(12), text(rivalry_text).size(12),]
                .spacing(4)
                .width(Length::Shrink);

            // Right column: add to game button
            let add_button = button("Add to Game")
                .on_press(Message::AddPlayerToNextSlot(name.clone()))
                .width(Length::Fixed(120.0));

            // Create the inner card content
            let card_content = row![
                left_col,
                space().width(16),
                middle_col,
                space().width(16),
                add_button,
            ]
            .align_y(iced::Alignment::Center)
            .spacing(8)
            .width(Length::Fill);

            // Wrap in a container with padding to create card distinction
            container(container(card_content).padding(14).width(Length::Fill))
                .padding(2)
                .width(Length::Fill)
                .into()
        };

        // Render as a vertical column of wide cards for better resizing
        let cards: Vec<_> = card_items
            .iter()
            .enumerate()
            .map(|(i, (_rank, name))| make_card(i, name.clone()))
            .collect();
        column(cards).spacing(6).width(Length::Fill).into()
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
        space().height(3),
        text("Algorithm:").size(12),
        pick_list(
            MatchupType::all().to_vec(),
            Some(app.matchup_type),
            Message::SetMatchupType
        )
        .width(Length::Fill)
        .text_size(12),
        space().height(6),
        player_display,
        space().height(8),
        button("Load Top 3 Matches into Game Input")
            .on_press_maybe(game.map(Message::SelectPlayers))
            .width(Length::Fill),
    ]
    .spacing(4)
    .width(Length::Fill);

    container(inner)
        .padding(Padding::new(15f32))
        .width(Length::Fill)
        .into()
}
