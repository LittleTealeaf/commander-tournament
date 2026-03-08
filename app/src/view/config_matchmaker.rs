use core::fmt::Display;

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

    fn set_value(&mut self, string: String) {
        self.value = string.parse::<f64>().ok();
        self.string = string;
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum MatchmakerConfigOption {
    LeastPlayed,
    Nemesis,
    LostWith,
    EloNeighbor,
    WRNeighbor,
    ExpectedNeighbor,
}

impl Display for MatchmakerConfigOption {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::LeastPlayed => write!(f, "Weight Least Played"),
            Self::Nemesis => write!(f, "Weight Nemesis"),
            Self::LostWith => write!(f, "Weight Lost With"),
            Self::EloNeighbor => write!(f, "Weight Elo Neighbor"),
            Self::WRNeighbor => write!(f, "Weight WR Neighbor"),
            Self::ExpectedNeighbor => write!(f, "Weight Expected Neighbor"),
        }
    }
}

impl MatchmakerConfigOption {
    const VALUES: [Self; 6] = [
        Self::LeastPlayed,
        Self::Nemesis,
        Self::LostWith,
        Self::EloNeighbor,
        Self::WRNeighbor,
        Self::ExpectedNeighbor,
    ];
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
    const fn get_config_value(&self, option: MatchmakerConfigOption) -> &ConfigValue {
        match option {
            MatchmakerConfigOption::LeastPlayed => &self.least_played,
            MatchmakerConfigOption::Nemesis => &self.nemesis,
            MatchmakerConfigOption::LostWith => &self.lost_with,
            MatchmakerConfigOption::EloNeighbor => &self.elo_neighbor,
            MatchmakerConfigOption::WRNeighbor => &self.wr_neighbor,
            MatchmakerConfigOption::ExpectedNeighbor => &self.expected_neighbor,
        }
    }

    #[must_use]
    const fn get_config_value_mut(&mut self, option: MatchmakerConfigOption) -> &mut ConfigValue {
        match option {
            MatchmakerConfigOption::LeastPlayed => &mut self.least_played,
            MatchmakerConfigOption::Nemesis => &mut self.nemesis,
            MatchmakerConfigOption::LostWith => &mut self.lost_with,
            MatchmakerConfigOption::EloNeighbor => &mut self.elo_neighbor,
            MatchmakerConfigOption::WRNeighbor => &mut self.wr_neighbor,
            MatchmakerConfigOption::ExpectedNeighbor => &mut self.expected_neighbor,
        }
    }

    #[must_use]
    pub fn get_updated_config(&self) -> Option<TournamentConfig> {
        Some(TournamentConfig {
            match_weight_least_played: self.least_played.value?,
            match_weight_nemesis: self.nemesis.value?,
            match_weight_lost_with: self.lost_with.value?,
            match_weight_elo_neighbor: self.elo_neighbor.value?,
            match_weight_wr_neighbor: self.wr_neighbor.value?,
            match_weight_expected_neighbor: self.expected_neighbor.value?,
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
    SetConfigValue(MatchmakerConfigOption, String),
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
            MessageMatchmakerConfig::Open => Message::done(),
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
            MessageMatchmakerConfig::SetConfigValue(option, value) => {
                let config = scene.get_config_value_mut(option);
                config.set_value(value);
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
        const INVALID_INPUT_COLOR: iced::Color = color!(0x0088_0808);
        container(
            container(
                column![
                    text("Matchmaker Configuration")
                        .align_x(Alignment::Center)
                        .size(25),
                    space().height(20),
                    rule::horizontal(2),
                    column(MatchmakerConfigOption::VALUES.map(|option| {
                        let value = scene.get_config_value(option);
                        row![
                            text(format!("{option}"))
                                .color_maybe(value.value.is_none().then_some(INVALID_INPUT_COLOR)),
                            text_input("Config Value", value.string.as_str()).on_input(
                                move |string| {
                                    MessageMatchmakerConfig::SetConfigValue(option, string).into()
                                }
                            )
                        ]
                        .spacing(15)
                        .align_y(Alignment::Center)
                        .into()
                    })),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_value_sets_string() {
        let config = ConfigValue::new(5.0);
        assert!(!config.string.is_empty());
    }

    #[test]
    fn invalid_value_doesnt_parse() {
        const VAL: &str = "AOIWEJOWIJEF";
        let mut config = ConfigValue::new(5.0);
        config.set_value(VAL.to_owned());
        assert_eq!(VAL, config.string);
        assert!(config.value.is_none());
        config.set_value("5.0".to_owned());
        assert!(config.value.is_some());
    }
}
