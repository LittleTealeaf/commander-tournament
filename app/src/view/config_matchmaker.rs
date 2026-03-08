use anyhow::anyhow;
use edh_tourn::config::TournamentConfig;
use iced::{
    Alignment, Length, color,
    widget::{button, column, container, row, rule, space, text, text_input},
};

use crate::{
    App,
    logic::Message,
    traits::{HandleMessage, View},
    view::Scene,
};

pub struct ConfigValue {
    value: Option<f64>,
    string: String,
}

impl ConfigValue {
    #[must_use]
    fn new(value: f64) -> Self {
        Self {
            string: format!("{value:.2}"),
            value: Some(value),
        }
    }

    fn update(&mut self, string: String) {
        self.value = string.parse::<f64>().ok();
        self.string = string;
    }
}

pub struct ViewMatchmakerConfig {
    config: TournamentConfig,
    least_played: ConfigValue,
    nemesis: ConfigValue,
    lost_with: ConfigValue,
    elo_neighbor: ConfigValue,
    wr_neighbor: ConfigValue,
    expected_neighbor: ConfigValue,
}

impl ViewMatchmakerConfig {
    #[must_use]
    pub fn new(config: TournamentConfig) -> Self {
        Self {
            least_played: ConfigValue::new(config.match_weight_least_played),
            nemesis: ConfigValue::new(config.match_weight_nemesis),
            lost_with: ConfigValue::new(config.match_weight_lost_with),
            elo_neighbor: ConfigValue::new(config.match_weight_elo_neighbor),
            wr_neighbor: ConfigValue::new(config.match_weight_wr_neighbor),
            expected_neighbor: ConfigValue::new(config.match_weight_expected_neighbor),
            config,
        }
    }

    #[must_use]
    pub fn get_updated_config(&self) -> Option<TournamentConfig> {
        Some(TournamentConfig {
            match_weight_least_played: self.least_played.value?,
            match_weight_expected_neighbor: self.expected_neighbor.value?,
            match_weight_wr_neighbor: self.wr_neighbor.value?,
            match_weight_elo_neighbor: self.elo_neighbor.value?,
            match_weight_lost_with: self.lost_with.value?,
            match_weight_nemesis: self.nemesis.value?,
            ..self.config
        })
    }

    #[must_use]
    pub const fn all_valid(&self) -> bool {
        self.least_played.value.is_some()
            && self.nemesis.value.is_some()
            && self.lost_with.value.is_some()
            && self.elo_neighbor.value.is_some()
            && self.wr_neighbor.value.is_some()
            && self.expected_neighbor.value.is_some()
    }
}

#[derive(Clone)]
pub enum MessageMatchmakerConfig {
    Open,
    Save,
    Close,
    SetLeastPlayed(String),
    SetNemesis(String),
    SetLostWith(String),
    SetEloNeighbor(String),
    SetWRNeighbor(String),
    SetExpectedNeighbor(String),
}

impl From<MessageMatchmakerConfig> for Message {
    fn from(value: MessageMatchmakerConfig) -> Self {
        Self::ViewMatchmakerConfig(value)
    }
}

impl HandleMessage<MessageMatchmakerConfig> for App {
    fn update(
        &mut self,
        msg: MessageMatchmakerConfig,
    ) -> anyhow::Result<iced::Task<crate::logic::Message>> {
        let Some(Scene::MatchmakerConfig(scene)) = self.scenes.last_mut() else {
            if matches!(msg, MessageMatchmakerConfig::Open) {
                self.scenes
                    .push(Scene::MatchmakerConfig(ViewMatchmakerConfig::new(
                        self.tournament().config().clone(),
                    )));
            }
            return Message::done();
        };

        match msg {
            MessageMatchmakerConfig::Open => {
                self.scenes
                    .push(Scene::MatchmakerConfig(ViewMatchmakerConfig::new(
                        self.tournament().config().clone(),
                    )));
                Message::done()
            }
            MessageMatchmakerConfig::Save => {
                let config = scene
                    .get_updated_config()
                    .ok_or_else(|| anyhow!("One or more invalid values"))?;
                self.tournament_mut().set_config(config)?;
                self.scenes.pop();
                Message::done()
            }
            MessageMatchmakerConfig::Close => {
                self.scenes.pop();
                Message::done()
            }
            MessageMatchmakerConfig::SetLeastPlayed(txt) => {
                scene.least_played.update(txt);
                Message::done()
            }
            MessageMatchmakerConfig::SetNemesis(txt) => {
                scene.nemesis.update(txt);
                Message::done()
            }
            MessageMatchmakerConfig::SetLostWith(txt) => {
                scene.lost_with.update(txt);
                Message::done()
            }
            MessageMatchmakerConfig::SetEloNeighbor(txt) => {
                scene.elo_neighbor.update(txt);
                Message::done()
            }
            MessageMatchmakerConfig::SetWRNeighbor(txt) => {
                scene.wr_neighbor.update(txt);
                Message::done()
            }
            MessageMatchmakerConfig::SetExpectedNeighbor(txt) => {
                scene.expected_neighbor.update(txt);
                Message::done()
            }
        }
    }
}

impl View<ViewMatchmakerConfig> for App {
    fn view<'a>(
        &'a self,
        scene: &'a ViewMatchmakerConfig,
    ) -> iced::Element<'a, crate::logic::Message> {
        fn config_row<'a>(
            name: &'a str,
            value: &ConfigValue,
            on_submit: impl Fn(String) -> MessageMatchmakerConfig + 'a,
        ) -> iced::Element<'a, crate::logic::Message> {
            row![
                text(name).color_maybe(value.value.is_none().then_some(color!(0x0088_0808))),
                text_input("Config Value", value.string.as_str())
                    .on_input(move |string| on_submit(string).into())
            ]
            .spacing(15)
            .align_y(Alignment::Center)
            .into()
        }

        container(
            container(
                column![
                    text("Matchmaker Configuration")
                        .align_x(Alignment::Center)
                        .size(25),
                    space().height(20),
                    rule::horizontal(2),
                    config_row(
                        "Weight Least Played",
                        &scene.least_played,
                        MessageMatchmakerConfig::SetLeastPlayed
                    ),
                    config_row(
                        "Weight Nemesis",
                        &scene.nemesis,
                        MessageMatchmakerConfig::SetNemesis
                    ),
                    config_row(
                        "Weight Lost With",
                        &scene.lost_with,
                        MessageMatchmakerConfig::SetLostWith
                    ),
                    config_row(
                        "Weight Elo Neighbor",
                        &scene.elo_neighbor,
                        MessageMatchmakerConfig::SetEloNeighbor
                    ),
                    config_row(
                        "Weight WR Neighbor",
                        &scene.wr_neighbor,
                        MessageMatchmakerConfig::SetWRNeighbor
                    ),
                    config_row(
                        "Weight Expected Neighbor",
                        &scene.expected_neighbor,
                        MessageMatchmakerConfig::SetExpectedNeighbor
                    ),
                    row![
                        button("Cancel").on_press(MessageMatchmakerConfig::Close.into()),
                        space().width(Length::Fill),
                        button("Save").on_press_maybe(
                            scene
                                .all_valid()
                                .then_some(MessageMatchmakerConfig::Save.into())
                        )
                    ]
                    .width(Length::Fill)
                    .align_y(Alignment::Center)
                ]
                .spacing(15),
            )
            .padding(20)
            .width(Length::Shrink),
        )
        .align_y(Alignment::Start)
        .align_x(Alignment::Center)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}
