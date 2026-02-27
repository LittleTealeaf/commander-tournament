use std::path::PathBuf;

use edh_tourn::Tournament;
use iced::Task;

use crate::{
    logic::Message,
    traits::HandleMessage,
    view::{Scene, home::HomeState},
};

mod logic;
mod traits;
mod view;

#[derive(Default)]
pub struct App {
    tournament: Tournament,
    error: Option<String>,
    file: Option<PathBuf>,
    home: HomeState,
    scenes: Vec<Scene>,
}

pub fn main() -> iced::Result {
    fn updater(app: &mut App, message: Message) -> Task<Message> {
        match app.update(message) {
            Ok(task) => task,
            Err(res) => {
                let msg = res.to_string();
                app.error = Some(msg);
                Task::none()
            }
        }
    }
    iced::run(updater, App::app_view)
}
//
// #![allow(dead_code)]
// mod message;
// #[cfg(test)]
// mod tests;
// mod traits;
// mod view;
//
// use core::cell::RefCell;
// use std::{path::PathBuf, rc::Rc};
//
// use appconfig::AppConfigManager;
// use edh_tourn::Tournament;
// use iced::Task;
//
// use crate::{
//     message::Message,
//     traits::{HandleMessage, View},
//     view::{Screen, home::AppHome},
// };
//
// pub fn main() -> anyhow::Result<()> {
//     launch()?;
//     Ok(())
// }
//
// pub fn launch() -> iced::Result {
//     fn updater(app: &mut App, message: Message) -> Task<Message> {
//         match app.update(message) {
//             Ok(task) => task,
//             Err(res) => {
//                 let msg = res.to_string();
//                 app.error = Some(msg);
//                 Task::none()
//             }
//         }
//     }
//     iced::run(updater, App::view)
// }
//
//
// #[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug, Default)]
// struct AppConfig {
//     last_opened_file: Option<PathBuf>,
// }
//
// impl Default for App {
//     fn default() -> Self {
//         let mut tournament = Tournament::default();
//         let _ = tournament.ingest_tsv_games(include_str!("../../data.tsv"));
//         Self {
//             error: None,
//             tournament,
//             screen: None,
//             home: AppHome::default(),
//             file: None,
//             config: AppConfigManager::new(
//                 Rc::from(RefCell::from(AppConfig::default())),
//                 "TournamentManager",
//                 "Tealeaf",
//             )
//             .with_auto_saving(true),
//         }
//     }
// }
