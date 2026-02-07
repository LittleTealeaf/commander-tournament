pub mod game_input;
pub mod game_matchups;
pub mod leaderboard;
pub mod modal;
pub mod toolbar;

use iced::{
    Element, Length,
    widget::{column, row, rule, space},
};

use crate::ui::{Message, TournamentApp};

pub fn view(app: &TournamentApp) -> Element<'_, Message> {
    // Show modal dialogs if active
    if app.error.is_some() {
        return modal::error_modal(app);
    }

    if app.show_ingest {
        return modal::ingest_modal(app);
    }

    if app.show_config {
        return modal::config_modal(app);
    }

    if app.change_player_name.is_some() {
        return modal::player_name_modal(app);
    }

    // Main layout with improved spacing and organization
    let main_content = row![
        leaderboard::leaderboard(app),
        rule::vertical(2),
        column![
            game_input::game_input(app),
            rule::horizontal(2),
            game_matchups::game_matchups(app)
        ]
        .width(Length::Fill)
        .spacing(0)
    ]
    .spacing(0)
    .width(Length::Fill)
    .height(Length::Fill);

    column![toolbar::toolbar(), space().height(5), main_content,]
        .spacing(0)
        .height(Length::Fill)
        .into()
}
