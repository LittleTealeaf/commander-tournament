mod message;
mod view;

use crate::{
    tournament::{GameMatch, Tournament},
    ui::message::{Message, update},
};

pub fn launch() -> iced::Result {
    fn updater(app: &mut TournamentApp, message: Message) {
        let result = update(app, message);
        if let Err(res) = result {
            let msg = res.to_string();
            app.error = Some(msg);
        }
    }
    iced::run(updater, view::view)
}

#[derive(Default)]
struct TournamentApp {
    tournament: Tournament,
    selected_players: [Option<String>; 4],
    selected_match: Option<GameMatch>,
    selected_winner: Option<String>,
    match_player: Option<String>,
    change_player_name: Option<(Option<String>, String)>,
    show_config: bool,
    error: Option<String>,
}
