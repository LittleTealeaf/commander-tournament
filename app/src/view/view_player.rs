use edh_tourn::{
    Tournament,
    error::TournamentError,
    game::GameRecord,
    info::{MtgColor, PlayerInfo},
    stats::PlayerStats,
};
use iced::{
    Alignment, Length, Task,
    widget::{button, column, container, row, text, text_input},
};

use crate::{
    App,
    logic::Message,
    traits::{HandleMessage, View},
    view::Scene,
};

#[derive(Clone, Debug)]
pub struct ViewPlayerScene {
    player: Option<u32>,
    name: Option<String>,
    moxfield: String,
    info: PlayerInfo,
    stats: Option<PlayerStats>,
    record: Vec<GameRecord>,
}

impl From<ViewPlayerScene> for Scene {
    fn from(value: ViewPlayerScene) -> Self {
        Self::Player(value)
    }
}

impl ViewPlayerScene {
    fn new(tournament: &Tournament, maybe_id: Option<u32>) -> anyhow::Result<Self> {
        Ok(match maybe_id {
            Some(id) => {
                let info = tournament
                    .get_player_info(&id)
                    .ok_or(TournamentError::InvalidPlayerId(id))?
                    .clone();

                Self {
                    player: Some(id),
                    moxfield: info.moxfield_id().cloned().unwrap_or_default(),
                    name: Some(info.name().to_owned()),
                    stats: Some(
                        tournament
                            .get_player_stats(id)
                            .cloned()
                            .unwrap_or_else(|| tournament.create_default_stats()),
                    ),
                    info,
                    record: tournament
                        .get_player_games(id)?
                        .copied()
                        .collect::<Vec<_>>(),
                }
            }
            None => Self {
                player: None,
                name: None,
                moxfield: String::new(),
                info: PlayerInfo::default(),
                stats: None,
                record: Vec::new(),
            },
        })
    }
}

impl View for ViewPlayerScene {
    fn view(&self) -> iced::Element<'_, Message> {
        let title = self.name.as_ref().map_or_else(
            || String::from("Create New Deck"),
            |name| format!("Edit: {name}"),
        );

        let colors_row = row(MtgColor::COLORS.into_iter().map(|color| {
            let style = if self.info.has_color(&color) {
                button::primary
            } else {
                button::secondary
            };

            button(color.letter())
                .on_press(ViewPlayerMessage::ToggleColor(color).into())
                .style(style)
                .into()
        }));

        let info_page = column![
            text_input("", self.info.name())
                .on_input(|text| ViewPlayerMessage::SetName(text).into()),
            text_input("Description", self.info.description())
                .on_input(|text| ViewPlayerMessage::SetDescription(text).into()),
            text_input("Moxfield ID", &self.moxfield)
                .on_input(|text| ViewPlayerMessage::SetMoxfieldId(text).into()),
            colors_row,
        ];

        let submit_row = row![
            button("Save").on_press(ViewPlayerMessage::SaveAndClose.into()),
            button("Close").on_press(ViewPlayerMessage::Close.into()),
        ];

        container(column![text(title), info_page, submit_row])
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
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
        let Some(Scene::Player(scene)) = self.scenes.last_mut() else {
            if let ViewPlayerMessage::Open(maybe_id) = msg {
                self.scenes.push(Scene::Player(ViewPlayerScene::new(
                    &self.tournament,
                    maybe_id,
                )?));
            }
            return Ok(Task::none());
        };

        match msg {
            ViewPlayerMessage::Open(maybe_id) => {
                self.scenes.push(Scene::Player(ViewPlayerScene::new(
                    &self.tournament,
                    maybe_id,
                )?));
                Ok(Task::none())
            }
            ViewPlayerMessage::SaveAndClose => {
                if let Some(id) = scene.player {
                    self.tournament.set_player_info(id, scene.info.clone())?;
                } else {
                    self.tournament
                        .register_player_with_info(scene.info.clone())?;
                }

                self.scenes.pop();

                Ok(Task::none())
            }
            ViewPlayerMessage::Close => {
                self.scenes.pop();
                Ok(Task::none())
            }
            ViewPlayerMessage::SetName(name) => {
                scene.info.set_name(name);
                Ok(Task::none())
            }
            ViewPlayerMessage::SetDescription(description) => {
                scene.info.set_description(description);
                Ok(Task::none())
            }
            ViewPlayerMessage::SetMoxfieldId(text) => {
                scene.moxfield = text;
                Ok(Task::none())
            }
            ViewPlayerMessage::ToggleColor(color) => {
                scene.info.toggle_color(color);
                Ok(Task::none())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use edh_tourn::Tournament;
    use itertools::Itertools;

    use crate::view::view_player::ViewPlayerScene;

    #[test]
    fn new_creates_default_values() {
        let t = Tournament::sample_game();
        let scene = ViewPlayerScene::new(&t, None).unwrap();
        assert!(scene.info.name().is_empty());
        assert!(scene.info.description().is_empty());
        assert!(scene.info.moxfield_link().is_none());
        assert!(scene.record.is_empty());
    }

    #[test]
    fn new_fails_when_invalid_id() {
        let t = Tournament::new();
        assert!(!t.players().keys().contains(&100));
        ViewPlayerScene::new(&t, Some(100)).unwrap_err();
    }

    #[test]
    fn new_grabs_player_data() {
        let t = Tournament::sample_game();

        for (id, info) in t.players().clone() {
            let stats = t
                .get_player_stats(id)
                .cloned()
                .unwrap_or_else(|| t.create_default_stats());

            let games = t.get_player_games(id).unwrap().copied().collect::<Vec<_>>();

            let scene = ViewPlayerScene::new(&t, Some(id)).unwrap();

            assert_eq!(Some(id), scene.player);
            assert_eq!(games, scene.record);
            assert_eq!(info, scene.info);
            assert_eq!(Some(stats), scene.stats);
        }
    }
}
