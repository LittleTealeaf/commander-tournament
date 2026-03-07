use std::borrow::ToOwned;

use edh_tourn::{
    Tournament, error::TournamentError, player::color::MtgColor, player::info::PlayerInfo,
};
use iced::{
    Element, Length,
    alignment::Horizontal,
    widget::{button, column, container, row, space, text, text_editor, text_input},
};

use crate::{
    App,
    logic::Message,
    traits::{HandleMessage, View},
    view::{Scene, confirm::ConfirmPrompt},
};

#[derive(Clone, Debug)]
pub struct ViewPlayerScene {
    player: Option<u32>,
    name: Option<String>,
    edit_description: text_editor::Content,
    moxfield: String,
    info: PlayerInfo,
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
                    edit_description: text_editor::Content::new(),
                    info,
                }
            }
            None => Self {
                player: None,
                name: None,
                edit_description: text_editor::Content::new(),
                moxfield: String::new(),
                info: PlayerInfo::default(),
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
    EditDescription(text_editor::Action),
    SetMoxfieldId(String),
    ToggleColor(MtgColor),
    ConfirmedDelete,
    Delete,
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
            return Message::done();
        };

        match msg {
            ViewPlayerMessage::Open(maybe_id) => {
                self.scenes.push(Scene::Player(ViewPlayerScene::new(
                    &self.tournament,
                    maybe_id,
                )?));
                Message::done()
            }
            ViewPlayerMessage::SaveAndClose => {
                scene.info.set_description(scene.edit_description.text());
                if !scene.moxfield.is_empty() {
                    scene.info.set_moxfield_id(scene.moxfield.clone());
                }
                if let Some(id) = scene.player {
                    self.tournament.set_player_info(id, scene.info.clone())?;
                } else {
                    self.tournament
                        .register_player_with_info(scene.info.clone())?;
                }

                self.scenes.pop();

                Message::done()
            }
            ViewPlayerMessage::Close => {
                self.scenes.pop();
                Message::done()
            }
            ViewPlayerMessage::SetName(name) => {
                scene.info.set_name(name);
                Message::done()
            }
            ViewPlayerMessage::EditDescription(action) => {
                scene.edit_description.perform(action);
                Message::done()
            }
            ViewPlayerMessage::SetMoxfieldId(text) => {
                scene.moxfield = text;
                Message::done()
            }
            ViewPlayerMessage::ToggleColor(color) => {
                scene.info.toggle_color(color);
                Message::done()
            }
            ViewPlayerMessage::Delete => {
                let name = scene.name.clone().unwrap_or_default();
                self.scenes.push(Scene::Confirm(ConfirmPrompt::new(format!(
                    "Are you sure you want to delete {name}, including any games they participated in?"
                ),
                    ViewPlayerMessage::ConfirmedDelete.into())));
                Message::done()
            }
            ViewPlayerMessage::ConfirmedDelete => {
                if let Some(id) = &scene.player {
                    self.tournament.unregister_player(*id)?;
                }
                self.scenes.pop();
                Message::done()
            }
        }
    }
}

impl View<ViewPlayerScene> for App {
    fn view<'a>(&'a self, scene: &'a ViewPlayerScene) -> Element<'a, Message> {
        let menu_bar = row![
            space().width(Length::Fill),
            button(text("Cancel")).on_press(ViewPlayerMessage::Close.into()),
            button(text("Save")).on_press(ViewPlayerMessage::SaveAndClose.into()),
        ]
        .spacing(20);

        let title_text = scene
            .name
            .as_ref()
            .map_or_else(|| "Create New Player".to_owned(), ToOwned::to_owned);

        let title = text(title_text).width(Length::Fill).center().size(50);

        let info_panel = {
            let edit_name = text_input("Player Name...", scene.info.name())
                .on_input(|text| ViewPlayerMessage::SetName(text).into());

            let edit_description = text_editor(&scene.edit_description)
                .placeholder("Description...")
                .on_action(|action| ViewPlayerMessage::EditDescription(action).into());

            // let edit_moxfieldid = text_input("Moxfield ID", &scene.moxfield);

            let deck_colors = row(MtgColor::COLORS.map(|color| {
                let style = if scene.info.color_identity().has_color(color) {
                    button::primary
                } else {
                    button::secondary
                };

                button(color.letter())
                    .on_press(ViewPlayerMessage::ToggleColor(color).into())
                    .style(style)
                    .into()
            }))
            .spacing(5);

            column![row![edit_name, deck_colors].spacing(20), edit_description]
                .max_width(700)
                .spacing(20)
        };

        // let deck_progress = scene.player.map(|id| {
        //     let stats = self.tournament.get_player_or_default_stats(id);
        //
        //     let view_stats = row![text(format!("Elo: {}", stats.elo()))];
        //
        //     column![view_stats]
        // });
        //
        // let content = match deck_progress {
        //     Some(view) => container(row![info_panel, view]),
        //     None => container(info_panel),
        // };

        let content = container(info_panel).align_x(Horizontal::Center);

        let bottom_panel = { row![space().width(Length::Fill)] };

        container(column![menu_bar, title, content, bottom_panel].width(Length::Fill)).into()
    }
}

#[cfg(test)]
mod tests {
    use edh_tourn::Tournament;
    use itertools::Itertools;

    use crate::view::player::ViewPlayerScene;

    #[test]
    fn new_creates_default_values() {
        let t = Tournament::sample_game();
        let scene = ViewPlayerScene::new(&t, None).unwrap();
        assert!(scene.info.name().is_empty());
        assert!(scene.info.description().is_empty());
        assert!(scene.info.moxfield_link().is_none());
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
            let scene = ViewPlayerScene::new(&t, Some(id)).unwrap();
            assert_eq!(Some(id), scene.player);
            assert_eq!(info, scene.info);
        }
    }
}
