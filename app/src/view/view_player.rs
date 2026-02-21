use edh_tourn::{
    Tournament,
    game::GameRecord,
    info::{MtgColor, PlayerInfo},
    stats::PlayerStats,
};
use iced::Task;

use crate::{App, logic::Message, traits::HandleMessage, view::Scene};

#[derive(Clone)]
pub struct ViewPlayerScene {
    player: Option<u32>,
    info: PlayerInfo,
    stats: Option<PlayerStats>,
    record: Vec<GameRecord>,
}

impl ViewPlayerScene {
    fn new(tournament: &Tournament, maybe_id: Option<u32>) -> anyhow::Result<Self> {
        Ok(match maybe_id {
            Some(id) => Self {
                player: Some(id),
                stats: Some(
                    tournament
                        .get_player_stats(id)
                        .cloned()
                        .unwrap_or_else(|| tournament.create_default_stats()),
                ),
                info: tournament.get_player_info(id)?,
                record: tournament
                    .get_player_games(id)?
                    .copied()
                    .collect::<Vec<_>>(),
            },
            None => Self {
                player: None,
                info: PlayerInfo::default(),
                stats: None,
                record: Vec::new(),
            },
        })
    }
}

#[derive(Clone)]
pub enum ViewPlayerMessage {
    Open(Option<u32>),
    SaveAndClose,
    Close,
    SetName(String),
    SetDescription(String),
    SetMoxfieldId(String),
    ToggleColor(MtgColor),
}

impl From<ViewPlayerMessage> for Message {
    fn from(value: ViewPlayerMessage) -> Self {
        Self::ViewPlayer(value)
    }
}

impl HandleMessage<ViewPlayerMessage> for App {
    fn update(
        &mut self,
        msg: ViewPlayerMessage,
    ) -> anyhow::Result<iced::Task<crate::logic::Message>> {
        let Scene::Player(scene) = &mut self.scene else {
            if let ViewPlayerMessage::Open(maybe_id) = msg {
                self.scene = Scene::Player(ViewPlayerScene::new(&self.tournament, maybe_id)?);
            }
            return Ok(Task::none());
        };

        match msg {
            ViewPlayerMessage::Open(_) => Ok(Task::none()),
            ViewPlayerMessage::SaveAndClose => {
                let id = match scene.player {
                    Some(id) => id,
                    None => self
                        .tournament
                        .register_player(scene.info.name().to_owned())?,
                };

                self.tournament.set_player_info(id, scene.info.clone())?;
                self.scene = Scene::Home;

                Ok(Task::none())
            }
            ViewPlayerMessage::Close => todo!(),
            ViewPlayerMessage::SetName(name) => {
                scene.info.set_name(name);
                Ok(Task::none())
            }
            ViewPlayerMessage::SetDescription(description) => {
                scene.info.set_description(description);
                Ok(Task::none())
            }
            ViewPlayerMessage::SetMoxfieldId(text) => {
                if text.is_empty() {
                    scene.info.set_moxfield_id(None);
                } else {
                    scene.info.set_moxfield_id(Some(text));
                }
                Ok(Task::none())
            }
            ViewPlayerMessage::ToggleColor(color) => {
                scene.info.toggle_color(color);
                Ok(Task::none())
            }
        }
    }
}
