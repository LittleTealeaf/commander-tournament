mod view;

use std::fs::{self, File};

use ron::ser::PrettyConfig;

use crate::tournament::{GameMatch, ScoreConfig, Tournament};

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

#[derive(Debug, Clone)]
pub enum Message {
    Ingest,
    SetChangePlayerName(Option<(Option<String>, String)>),
    ChangePlayerSubmit,
    ShowConfig(bool),
    SelectPlayer(usize, Option<String>),
    SelectPlayers([String; 4]),
    SelectWinner(String),
    SelectMatchPlayer(String),
    DeletePlayer(String),
    SubmitGame,
    Reload,
    SetScoreConfig(ScoreConfig),
    CloseError,
    Load(String),
    Save(String),
    New,
}

#[derive(thiserror::Error, Debug)]
enum AppError {
    #[error("Missing Player: {0}")]
    MissingPlayer(usize),
}

pub fn update(app: &mut TournamentApp, message: Message) -> anyhow::Result<()> {
    match message {
        Message::Reload => {
            app.tournament.reload()?;
        }
        Message::SelectPlayers(players) => {
            for (i, player) in players.into_iter().enumerate() {
                update(app, Message::SelectPlayer(i, Some(player)))?;
            }
        }
        Message::New => {
            app.tournament = Tournament::new();
        }
        Message::Ingest => {
            let data = std::fs::read_to_string("data.tsv")?;
            for line in data.lines() {
                let parts = line.split('\t').collect::<Vec<_>>();
                if parts.len() < 5 {
                    continue;
                }

                let players = [parts[0], parts[1], parts[2], parts[3]];
                let winner = parts[4].to_string();
                let game = app.tournament.create_game(players);
                app.tournament.submit_game(game, winner)?;
            }
        }
        Message::ShowConfig(val) => app.show_config = val,
        Message::ChangePlayerSubmit => {
            if let Some((prev, name)) = &app.change_player_name {
                if let Some(prev) = prev {
                    if !prev.eq(name) {
                        app.tournament
                            .rename_player(prev.to_string(), name.to_string())?;
                    }
                } else {
                    app.tournament.register_player(name.to_string());
                }
                app.change_player_name = None;
            }
        }
        Message::SetChangePlayerName(value) => app.change_player_name = value,
        Message::CloseError => {
            app.error = None;
        }
        Message::DeletePlayer(player) => {
            app.tournament.remove_player(player)?;
            app.change_player_name = None;
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
        Message::Load(path) => {
            let file = File::open(path)?;
            app.tournament = ron::de::from_reader(file)?;
            app.tournament.reload()?;
            app.selected_match = Default::default();
            app.selected_winner = Default::default();
            app.selected_players = Default::default();
        }
        Message::Save(path) => {
            fs::write(
                path,
                ron::ser::to_string_pretty(&app.tournament, PrettyConfig::default())?,
            )?;
        }
    }
    Ok(())
}
