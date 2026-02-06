use crate::{tournament::ScoreConfig, ui::TournamentApp};

#[derive(Debug, Clone)]
pub enum Message {
    NewPlayerSubmit,
    NewPlayerSetName(Option<String>),
    SelectPlayer(usize, Option<String>),
    SelectWinner(String),
    SelectMatchPlayer(String),
    SubmitGame,
    SetScoreConfig(ScoreConfig),
    CloseError,
}

#[derive(thiserror::Error, Debug)]
enum AppError {
    #[error("Missing Player: {0}")]
    MissingPlayer(usize),
}

pub fn update(app: &mut TournamentApp, message: Message) -> anyhow::Result<()> {
    match message {
        Message::NewPlayerSubmit => {
            if let Some(name) = &app.new_player_name {
                app.tournament.register_player(name.clone());
                return update(app, Message::NewPlayerSetName(None));
            }
        }
        Message::NewPlayerSetName(name) => app.new_player_name = name,
        Message::CloseError => {
            app.error = None;
        }
        Message::SelectPlayer(index, value) => {
            if index < 4 {
                if let Some(player) = value {
                    if app.tournament.has_registered_player(&player) {
                        app.selected_players[index] = Some(player)
                    }
                } else {
                    app.selected_players[index] = None;
                }
                app.selected_match =
                    if let [Some(p1), Some(p2), Some(p3), Some(p4)] = &app.selected_players {
                        Some(app.tournament.create_game([p1, p2, p3, p4]))
                    } else {
                        None
                    };
            }
        }
        Message::SelectWinner(winner) => app.selected_winner = Some(winner),
        Message::SelectMatchPlayer(_) => todo!(),
        Message::SubmitGame => {
            if let (Some(game), Some(winner)) = (&app.selected_match, &app.selected_winner) {
                app.tournament.submit_game(game.clone(), winner)?;
                app.selected_players = Default::default();
                app.selected_winner = Default::default();
                app.selected_match = None;
            }
        }
        Message::SetScoreConfig(config) => {
            app.tournament.set_score_config(config)?;
        }
    }
    Ok(())
}
