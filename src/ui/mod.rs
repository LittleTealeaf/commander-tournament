mod errors;
mod message;
mod state;
pub mod views;

pub use message::Message;
pub use state::TournamentApp;

use std::fs::{self, File};

use ron::ser::PrettyConfig;

use crate::tournament::MatchmakerConfig;
use crate::tournament::ScoreConfig;
use crate::tournament::Tournament;
use anyhow::anyhow;

fn f64_to_string(v: f64) -> String {
    format!("{}", v)
}

fn parse_f64(s: &str) -> Result<f64, anyhow::Error> {
    s.trim()
        .parse::<f64>()
        .map_err(|e| anyhow!("Failed to parse '{}' as number: {}", s, e))
}

pub fn launch() -> iced::Result {
    fn updater(app: &mut TournamentApp, message: Message) {
        let result = update(app, message);
        if let Err(res) = result {
            let msg = res.to_string();
            app.error = Some(msg);
        }
    }
    iced::run(updater, views::view)
}

pub fn update(app: &mut TournamentApp, message: Message) -> anyhow::Result<()> {
    match message {
        Message::ShowIngest(val) => {
            app.show_ingest = val;
            if !val {
                app.ingest_text.clear();
            }
        }
        Message::ShowExport(val) => {
            app.show_export = val;
            if val {
                // build TSV export from games
                let mut buf = String::new();
                for g in app.tournament.games().iter() {
                    buf.push_str(&format!(
                        "{}\t{}\t{}\t{}\t{}\n",
                        g.players[0], g.players[1], g.players[2], g.players[3], g.winner
                    ));
                }
                app.export_text = buf;
            } else {
                app.export_text.clear();
            }
        }
        Message::UpdateExport(text) => {
            app.export_text = text;
        }
        Message::UpdateIngest(text) => {
            app.ingest_text = text;
        }
        Message::SubmitIngest => {
            for line in app.ingest_text.lines() {
                let parts = line.split('\t').collect::<Vec<_>>();
                if parts.len() < 5 {
                    continue;
                }

                let players = [parts[0], parts[1], parts[2], parts[3]];
                let winner = parts[4].to_string();
                let game = app.tournament.create_game(players);
                app.tournament.submit_game(game, winner)?;
            }
            app.show_ingest = false;
            app.ingest_text.clear();
        }
        Message::Reload => {
            app.tournament.reload()?;
        }
        Message::SelectPlayers(players) => {
            for (i, player) in players.into_iter().enumerate() {
                update(app, Message::SelectPlayer(i, Some(player)))?;
            }
        }
        Message::AddPlayerToNextSlot(player) => {
            // Find the first empty slot and add the player there
            if let Some(empty_index) = app.selected_players.iter().position(|p| p.is_none()) {
                update(app, Message::SelectPlayer(empty_index, Some(player)))?;
            }
        }
        Message::New => {
            app.tournament = Tournament::new();
        }
        Message::ShowConfig(val) => {
            app.show_config = val;
            if val {
                // Pre-fill editable strings with current config numeric values
                let sc = app.tournament.get_score_config();
                app.score_starting_elo = f64_to_string(sc.starting_elo);
                app.score_game_points = f64_to_string(sc.game_points);
                app.score_elo_pow = f64_to_string(sc.elo_pow);
                app.score_wr_pow = f64_to_string(sc.wr_pow);
                app.score_elo_weight = f64_to_string(sc.elo_weight);
                app.score_wr_weight = f64_to_string(sc.wr_weight);

                let mc = app.tournament.get_match_config();
                app.match_weight_least_played = f64_to_string(mc.weight_least_played);
                app.match_weight_nemesis = f64_to_string(mc.weight_nemesis);
                app.match_weight_neighbor = f64_to_string(mc.weight_neighbor);
                app.match_weight_wr_neighbor = f64_to_string(mc.weight_wr_neighbor);
                app.match_weight_lost_with = f64_to_string(mc.weight_lost_with);
            }
        }
        Message::UpdateScoreStartingElo(s) => app.score_starting_elo = s,
        Message::UpdateScoreGamePoints(s) => app.score_game_points = s,
        Message::UpdateScoreEloPow(s) => app.score_elo_pow = s,
        Message::UpdateScoreWrPow(s) => app.score_wr_pow = s,
        Message::UpdateScoreEloWeight(s) => app.score_elo_weight = s,
        Message::UpdateScoreWrWeight(s) => app.score_wr_weight = s,

        Message::UpdateMatchWeightLeastPlayed(s) => app.match_weight_least_played = s,
        Message::UpdateMatchWeightNemesis(s) => app.match_weight_nemesis = s,
        Message::UpdateMatchWeightNeighbor(s) => app.match_weight_neighbor = s,
        Message::UpdateMatchWeightWrNeighbor(s) => app.match_weight_wr_neighbor = s,
        Message::UpdateMatchWeightLostWith(s) => app.match_weight_lost_with = s,

        Message::SaveConfig => {
            // Parse score config fields
            let starting_elo = parse_f64(&app.score_starting_elo)?;
            let game_points = parse_f64(&app.score_game_points)?;
            let elo_pow = parse_f64(&app.score_elo_pow)?;
            let wr_pow = parse_f64(&app.score_wr_pow)?;
            let elo_weight = parse_f64(&app.score_elo_weight)?;
            let wr_weight = parse_f64(&app.score_wr_weight)?;

            let score_cfg = ScoreConfig {
                starting_elo,
                game_points,
                elo_pow,
                wr_pow,
                elo_weight,
                wr_weight,
            };
            app.tournament.set_score_config(score_cfg)?;

            // Parse match config fields
            let weight_least_played = parse_f64(&app.match_weight_least_played)?;
            let weight_nemesis = parse_f64(&app.match_weight_nemesis)?;
            let weight_neighbor = parse_f64(&app.match_weight_neighbor)?;
            let weight_wr_neighbor = parse_f64(&app.match_weight_wr_neighbor)?;
            let weight_lost_with = parse_f64(&app.match_weight_lost_with)?;

            let match_cfg = MatchmakerConfig {
                weight_least_played,
                weight_nemesis,
                weight_neighbor,
                weight_wr_neighbor,
                weight_lost_with,
            };
            app.tournament.set_match_config(match_cfg)?;
            app.show_config = false;
        }
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
        Message::SelectMatchPlayer(player) => app.match_player = Some(player),
        Message::SubmitGame => {
            if let (Some(game), Some(winner)) = (&app.selected_match, &app.selected_winner) {
                app.tournament.submit_game(game.clone(), winner)?;
                app.selected_players = Default::default();
                app.selected_winner = Default::default();
                app.selected_match = None;
            }
        }
        Message::ClearGame => {
            app.selected_players = Default::default();
            app.selected_winner = Default::default();
            app.selected_match = None;
        }
        Message::SetMatchupType(matchup_type) => {
            app.matchup_type = matchup_type;
        }
        Message::Load(path) => {
            let file = File::open(path)?;
            app.tournament = ron::de::from_reader(file)?;
            app.tournament.reload()?;
            app.selected_match = Default::default();
            app.selected_winner = Default::default();
            app.selected_players = Default::default();
            app.match_player = None;
        }
        Message::Save(path) => {
            fs::write(
                path,
                ron::ser::to_string_pretty(&app.tournament, PrettyConfig::default())?,
            )?;
        }
        Message::ShowGames(val) => {
            app.show_games = val;
            if !val {
                app.selected_game_index = None;
            }
        }
        Message::OpenChangeWinnerModal(index) => {
            if index == usize::MAX {
                app.selected_game_index = None;
            } else {
                app.selected_game_index = Some(index);
            }
        }
        Message::ChangeGameWinner(index, winner) => {
            app.tournament.set_game_winner(index, winner)?;
            app.selected_game_index = None;
        }
        Message::DeleteGame(index) => {
            app.tournament.delete_game(index)?;
            app.selected_game_index = None;
        }
    }
    Ok(())
}
