use edh_tourn::{
    Tournament,
    error::TournamentError,
    info::{MtgColor, PlayerInfo},
    stats::PlayerStats,
};
use iced::{
    Element, Length, Task,
    widget::{button, column, row, space, table, text, text_input},
};

use crate::{
    App,
    logic::Message,
    traits::{HandleMessage, ViewWithApp},
    view::Scene,
};

#[derive(Clone, Debug)]
pub struct ViewPlayerScene {
    player: Option<u32>,
    name: Option<String>,
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
                    info,
                }
            }
            None => Self {
                player: None,
                name: None,
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

impl ViewWithApp for ViewPlayerScene {
    fn view<'a>(&'a self, app: &'a App) -> iced::Element<'a, Message> {
        let tournament = &app.tournament;

        let stats_panel = self.player.map(|id| {
            let stats = tournament.get_player_or_default_stats(id);

            let stats_view = stats_display(stats);
            let games_table = games_table(id, tournament);

            column![stats_view, games_table]
        });

        let info_panel = info_display(
            self.info.name(),
            self.info.description(),
            &self.moxfield,
            self.info.colors(),
        );

        let mut row = row![info_panel];
        if let Some(stats_panel) = stats_panel {
            row = row.push(stats_panel);
        }

        row.into()
    }
}

fn colors_bar(colors: &[MtgColor]) -> Element<'_, Message> {
    row(MtgColor::COLORS.into_iter().map(|color| {
        let style = if colors.contains(&color) {
            button::primary
        } else {
            button::secondary
        };

        button(color.letter())
            .on_press(ViewPlayerMessage::ToggleColor(color).into())
            .style(style)
            .into()
    }))
    .into()
}

fn info_display<'a>(
    name: &'a str,
    description: &'a str,
    moxfield: &'a str,
    colors: &'a [MtgColor],
) -> Element<'a, Message> {
    column![
        text_input("", name).on_input(|text| ViewPlayerMessage::SetName(text).into()),
        text_input("Description", description)
            .on_input(|text| ViewPlayerMessage::SetDescription(text).into()),
        text_input("Moxfield ID", moxfield)
            .on_input(|text| ViewPlayerMessage::SetMoxfieldId(text).into()),
        colors_bar(colors)
    ]
    .into()
}

fn stats_display(stats: &PlayerStats) -> Element<'_, Message> {
    row![
        text(format!("Elo: {}", stats.elo())).size(15),
        space().width(Length::Fill)
    ]
    .width(Length::Fill)
    .into()
}

fn games_table(id: u32, tournament: &Tournament) -> Element<'_, Message> {
    #[derive(Clone)]
    struct GameRow {
        players: [String; 4],
        winner: String,
        elo_change: f64,
    }

    let unknown_string = String::from("??????");
    let rows = tournament
        .get_player_games(id)
        .into_iter()
        .flatten()
        .map(|record| GameRow {
            players: (*record.players()).map(|player| {
                tournament
                    .get_player_info(&player)
                    .map_or_else(|| &unknown_string, PlayerInfo::name)
                    .to_owned()
            }),
            winner: tournament
                .get_player_info(&record.winner())
                .map_or_else(|| &unknown_string, PlayerInfo::name)
                .to_owned(),
            elo_change: record.get_player_elo_change(&id).unwrap_or_default(),
        });

    table(
        [
            table::column("Players", |row: GameRow| {
                text(format!(
                    "{}\n{}\n{}\n{}",
                    row.players[0], row.players[1], row.players[2], row.players[3]
                ))
            }),
            table::column("Winner", |row: GameRow| text(row.winner)),
            table::column("Elo Change", |row: GameRow| text(row.elo_change)),
        ],
        rows,
    )
    .into()
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
