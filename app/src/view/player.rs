mod game_history;
mod info;
mod stats;

use core::fmt::{Display, Formatter};
use std::borrow::ToOwned;

use edh_tourn::{
    Tournament,
    error::TournamentError,
    player::{
        color::{ColorIdentity, MtgColor},
        info::PlayerInfo,
    },
};
use iced::{
    Alignment, Element, Length,
    widget::{button, column, container, row, space, text, text_editor},
};
use iced_aw::{TabBar, TabLabel};

use crate::{
    App,
    logic::Message,
    traits::{HandleMessage, View},
    view::{
        Scene,
        confirm::ConfirmPrompt,
        player::{
            game_history::view_game_history,
            info::view_info_panel,
            stats::{
                view_color_matchups, view_identity_matchups, view_player_matchups,
                view_stats_summary,
            },
        },
    },
};

#[derive(Clone, Debug)]
pub struct ViewPlayerScene {
    player: Option<u32>,
    name: Option<String>,
    edit_description: text_editor::Content,
    moxfield: String,
    info: PlayerInfo,
    stats_tab: StatsTab,
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
                    edit_description: text_editor::Content::with_text(info.description()),
                    stats_tab: StatsTab::default(),
                    info,
                }
            }
            None => Self {
                player: None,
                name: None,
                edit_description: text_editor::Content::new(),
                stats_tab: StatsTab::default(),
                moxfield: String::new(),
                info: PlayerInfo::default(),
            },
        })
    }

    pub fn title(&self) -> String {
        format!("Player: {}", self.info.name())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StatsTab {
    #[default]
    Games,
    Players,
    Identities,
    Colors,
}

impl StatsTab {
    pub const VALUES: [Self; 4] = [Self::Games, Self::Players, Self::Identities, Self::Colors];
}

impl Display for StatsTab {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            Self::Games => "Games",
            Self::Players => "Players",
            Self::Identities => "Identities",
            Self::Colors => "Colors",
        })
    }
}

#[derive(Clone, Debug)]
pub enum ViewPlayerMessage {
    Open(Option<u32>),
    SaveAndClose,
    Close,
    SetName(String),
    EditDescription(text_editor::Action),
    SetMoxfieldId(String),
    ClearColors,
    ToggleColor(MtgColor),
    ConfirmedDelete,
    SetStatsTab(StatsTab),
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
                if maybe_id.is_some() && maybe_id == scene.player {
                    return Message::done();
                }
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

                self.update(ViewPlayerMessage::Close)
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
                text.clone_into(&mut scene.moxfield);
                scene.info.set_moxfield_id(text);
                Message::done()
            }
            ViewPlayerMessage::ClearColors => {
                scene.info.set_color_identity(ColorIdentity::default());
                Message::done()
            }
            ViewPlayerMessage::ToggleColor(color) => {
                scene.info.toggle_color(color);
                Message::done()
            }
            ViewPlayerMessage::Delete => {
                let name = scene.name.clone().unwrap_or_default();
                self.scenes.push(Scene::Confirm(ConfirmPrompt::new(format!("Delete {name}?"),format!(
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
            ViewPlayerMessage::SetStatsTab(tab) => {
                scene.stats_tab = tab;
                Message::done()
            }
        }
    }
}

impl View<ViewPlayerScene> for App {
    fn view<'a>(&'a self, scene: &'a ViewPlayerScene) -> Element<'a, Message> {
        let title_text = scene
            .name
            .as_ref()
            .map_or_else(|| "Create New Player".to_owned(), ToOwned::to_owned);

        let title = text(title_text).width(Length::Fill).center().size(50);

        let info_panel = view_info_panel(scene).max_width(700);

        let deck_progress = scene.player.map(|id| {
            column![
                view_stats_summary(self.tournament().get_player_or_default_stats(id)),
                StatsTab::VALUES
                    .into_iter()
                    .fold(
                        TabBar::new(|tab| ViewPlayerMessage::SetStatsTab(tab).into()),
                        |tab_bar, tab| { tab_bar.push(tab, TabLabel::Text(format!("{tab}"))) }
                    )
                    .set_active_tab(&scene.stats_tab),
                match scene.stats_tab {
                    StatsTab::Games => view_game_history(self.tournament(), id),
                    StatsTab::Players => view_player_matchups(self.tournament(), id),
                    StatsTab::Identities => view_identity_matchups(self.tournament(), id),
                    StatsTab::Colors => view_color_matchups(self.tournament(), id),
                }
                .unwrap_or_else(|| {
                    container(
                        text("No Stats Available")
                            .size(25)
                            .width(Length::Fill)
                            .align_x(Alignment::Center),
                    )
                })
                .width(Length::Fill)
                .height(Length::Fill)
            ]
            .spacing(30)
        });

        let bottom_row = row![
            scene.player.is_some().then_some(
                button("Delete")
                    .style(button::danger)
                    .on_press(ViewPlayerMessage::Delete.into())
            ),
            space().width(Length::Fill),
            button("Close").on_press(ViewPlayerMessage::Close.into()),
            button("Save").on_press(ViewPlayerMessage::SaveAndClose.into())
        ]
        .spacing(20);

        container(
            column![
                title,
                row![info_panel, deck_progress]
                    .height(Length::Fill)
                    .spacing(40),
                bottom_row
            ]
            .spacing(20)
            .width(Length::Fill)
            .height(Length::Fill),
        )
        .height(Length::Fill)
        .into()
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
