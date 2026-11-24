mod drafts;

use commander_tournament_core::config::{game::GameConfig, matchmaker::MatchmakerConfig};
use iced::{
    Element, Length,
    alignment::{Horizontal, Vertical},
    widget::{button, checkbox, column, container, row, rule, space, text, text_input},
};
use iced_tea::{Component, Model, Signal};
use nerd_font_symbols::md;

use crate::{
    components::empty_card,
    fonts::FONT_BOLD,
    models::tournament::TournamentModel,
    style::{self, COLORS},
};

use self::drafts::{GameConfigDraft, MatchmakerConfigDraft};

#[derive(Debug, Default)]
pub struct SettingsState {
    game_draft: Option<GameConfigDraft>,
    mm_draft: Option<MatchmakerConfigDraft>,
    status_message: Option<(String, bool)>,
}

#[derive(Debug, Clone)]
pub enum SettingsMsg {
    InitialEloChanged(String),
    LogisticScaleChanged(String),
    InitialKChanged(String),
    BaseKChanged(String),
    CalibrationGamesChanged(String),
    ResetGameDefaults,
    SaveGameConfig,

    EloRangeChanged(String),
    MinPoolSizeChanged(String),
    RepeatPenaltyChanged(String),
    StrictCalibrateToggled(bool),
    MaxPreconChanged(String),
    MaxUnrankedChanged(String),
    ResetMatchmakerDefaults,
    SaveMatchmakerConfig,

    ReloadRatings,
    SetStatus(String),
}

#[derive(Debug, Clone)]
pub enum SettingsOut {
    SaveGameConfig(GameConfig),
    SaveMatchmakerConfig(MatchmakerConfig),
    ReloadRatings,
}

impl SettingsState {
    fn ensure_game_draft(&mut self, context: &TournamentModel) -> &mut GameConfigDraft {
        self.game_draft.get_or_insert_with(|| {
            context.tournament().map_or_else(
                || GameConfigDraft::from(&GameConfig::default()),
                |t| GameConfigDraft::from(t.game_config()),
            )
        })
    }

    fn ensure_mm_draft(&mut self, context: &TournamentModel) -> &mut MatchmakerConfigDraft {
        self.mm_draft.get_or_insert_with(|| {
            context.tournament().map_or_else(
                || MatchmakerConfigDraft::from(&MatchmakerConfig::default()),
                |t| MatchmakerConfigDraft::from(t.matchmaker_config()),
            )
        })
    }
}

impl Model for SettingsState {
    type Message = SettingsMsg;
    type OutMessage = SettingsOut;
    type Context<'a>
        = &'a TournamentModel
    where
        Self: 'a;

    fn update<'a>(
        &'a mut self,
        message: Self::Message,
        context: Self::Context<'a>,
    ) -> anyhow::Result<Signal<Self::Message, Self::OutMessage>> {
        match message {
            SettingsMsg::InitialEloChanged(v) => {
                self.ensure_game_draft(context).initial_elo = v;
                self.status_message = None;
                Signal::done()
            }
            SettingsMsg::LogisticScaleChanged(v) => {
                self.ensure_game_draft(context).logistic_scale = v;
                self.status_message = None;
                Signal::done()
            }
            SettingsMsg::InitialKChanged(v) => {
                self.ensure_game_draft(context).initial_k = v;
                self.status_message = None;
                Signal::done()
            }
            SettingsMsg::BaseKChanged(v) => {
                self.ensure_game_draft(context).base_k = v;
                self.status_message = None;
                Signal::done()
            }
            SettingsMsg::CalibrationGamesChanged(v) => {
                self.ensure_game_draft(context).calibration_games = v;
                self.status_message = None;
                Signal::done()
            }
            SettingsMsg::ResetGameDefaults => {
                self.game_draft = Some(GameConfigDraft::from(&GameConfig::default()));
                self.status_message = None;
                Signal::done()
            }
            SettingsMsg::SaveGameConfig => {
                let draft = self.ensure_game_draft(context).clone();
                match draft.parse() {
                    Ok(cfg) => {
                        self.game_draft = Some(GameConfigDraft::from(&cfg));
                        self.status_message = None;
                        Ok(
                            Signal::out(SettingsOut::SaveGameConfig(cfg)).chain(Signal::delayed(
                                SettingsMsg::SetStatus(
                                    "Rating engine settings saved & ratings replayed.".to_owned(),
                                ),
                            )),
                        )
                    }
                    Err(err) => {
                        self.status_message = Some((err, true));
                        Signal::done()
                    }
                }
            }
            SettingsMsg::EloRangeChanged(v) => {
                self.ensure_mm_draft(context).elo_range = v;
                self.status_message = None;
                Signal::done()
            }
            SettingsMsg::MinPoolSizeChanged(v) => {
                self.ensure_mm_draft(context).min_pool_size = v;
                self.status_message = None;
                Signal::done()
            }
            SettingsMsg::RepeatPenaltyChanged(v) => {
                self.ensure_mm_draft(context).repeat_penalty = v;
                self.status_message = None;
                Signal::done()
            }
            SettingsMsg::StrictCalibrateToggled(v) => {
                self.ensure_mm_draft(context).strict_calibrate = v;
                self.status_message = None;
                Signal::done()
            }
            SettingsMsg::MaxPreconChanged(v) => {
                self.ensure_mm_draft(context).max_precon = v;
                self.status_message = None;
                Signal::done()
            }
            SettingsMsg::MaxUnrankedChanged(v) => {
                self.ensure_mm_draft(context).max_unranked = v;
                self.status_message = None;
                Signal::done()
            }
            SettingsMsg::ResetMatchmakerDefaults => {
                self.mm_draft = Some(MatchmakerConfigDraft::from(&MatchmakerConfig::default()));
                self.status_message = None;
                Signal::done()
            }
            SettingsMsg::SaveMatchmakerConfig => {
                let draft = self.ensure_mm_draft(context).clone();
                match draft.parse() {
                    Ok(cfg) => {
                        self.mm_draft = Some(MatchmakerConfigDraft::from(&cfg));
                        self.status_message = None;
                        Ok(
                            Signal::out(SettingsOut::SaveMatchmakerConfig(cfg)).chain(Signal::delayed(
                                SettingsMsg::SetStatus("Matchmaker settings saved.".to_owned()),
                            )),
                        )
                    }
                    Err(err) => {
                        self.status_message = Some((err, true));
                        Signal::done()
                    }
                }
            }
            SettingsMsg::ReloadRatings => {
                self.status_message = None;
                Ok(
                    Signal::out(SettingsOut::ReloadRatings).chain(Signal::delayed(SettingsMsg::SetStatus(
                        "Recalculated all ratings from match history.".to_owned(),
                    ))),
                )
            }
            SettingsMsg::SetStatus(msg) => {
                self.status_message = Some((msg, false));
                Signal::done()
            }
        }
    }
}

impl Component for SettingsState {
    fn render<'a>(&'a self, context: Self::Context<'a>) -> Element<'a, Self::Message> {
        let Some(tournament) = context.tournament() else {
            return empty_card(
                md::MD_FOLDER_OPEN_OUTLINE,
                "No tournament file open",
                "Open or create a .ron file to configure rating engine and matchmaker settings.",
                None,
            );
        };

        let game_draft = self
            .game_draft
            .clone()
            .unwrap_or_else(|| GameConfigDraft::from(tournament.game_config()));
        let mm_draft = self
            .mm_draft
            .clone()
            .unwrap_or_else(|| MatchmakerConfigDraft::from(tournament.matchmaker_config()));

        let game_card = render_game_config_card(&game_draft);
        let mm_card = render_matchmaker_config_card(&mm_draft);
        let maintenance_card = render_maintenance_card();

        let mut stack = column![].spacing(16).max_width(760);

        if let Some((msg, is_err)) = &self.status_message {
            stack = stack.push(
                container(
                    text(msg)
                        .size(13)
                        .color(if *is_err { COLORS.red } else { COLORS.green }),
                )
                .width(Length::Fill)
                .padding([10, 14])
                .style(style::card),
            );
        }

        container(stack.push(game_card).push(mm_card).push(maintenance_card))
            .width(Length::Fill)
            .align_x(Horizontal::Center)
            .into()
    }
}

fn render_game_config_card<'a>(draft: &GameConfigDraft) -> Element<'a, SettingsMsg> {
    container(
        column![
            text("Plackett-Luce Rating Engine")
                .font(FONT_BOLD)
                .size(16)
                .color(COLORS.text),
            rule::horizontal(1),
            row![
                labeled_input("Initial Elo", &draft.initial_elo, SettingsMsg::InitialEloChanged),
                labeled_input(
                    "Logistic Scale",
                    &draft.logistic_scale,
                    SettingsMsg::LogisticScaleChanged,
                ),
            ]
            .spacing(12)
            .width(Length::Fill),
            row![
                labeled_input("Initial K-Factor", &draft.initial_k, SettingsMsg::InitialKChanged),
                labeled_input("Base K-Factor", &draft.base_k, SettingsMsg::BaseKChanged),
                labeled_input(
                    "Calibration Games",
                    &draft.calibration_games,
                    SettingsMsg::CalibrationGamesChanged,
                ),
            ]
            .spacing(12)
            .width(Length::Fill),
            row![
                space::horizontal(),
                button(text("Reset Defaults").size(13))
                    .padding([8, 12])
                    .style(style::secondary_button)
                    .on_press(SettingsMsg::ResetGameDefaults),
                button(text("Save Rating Config").font(FONT_BOLD).size(13))
                    .padding([8, 14])
                    .style(style::primary_button)
                    .on_press(SettingsMsg::SaveGameConfig),
            ]
            .spacing(8)
            .align_y(Vertical::Center),
        ]
        .spacing(12)
        .width(Length::Fill),
    )
    .width(Length::Fill)
    .padding(16)
    .style(style::card)
    .into()
}

fn render_matchmaker_config_card<'a>(draft: &MatchmakerConfigDraft) -> Element<'a, SettingsMsg> {
    container(
        column![
            text("Matchmaker Parameters")
                .font(FONT_BOLD)
                .size(16)
                .color(COLORS.text),
            rule::horizontal(1),
            row![
                labeled_input("Elo Range", &draft.elo_range, SettingsMsg::EloRangeChanged),
                labeled_input(
                    "Min Pool Size",
                    &draft.min_pool_size,
                    SettingsMsg::MinPoolSizeChanged,
                ),
                labeled_input(
                    "Repeat Penalty",
                    &draft.repeat_penalty,
                    SettingsMsg::RepeatPenaltyChanged,
                ),
            ]
            .spacing(12)
            .width(Length::Fill),
            row![
                labeled_input(
                    "Max Precons per Pod",
                    &draft.max_precon,
                    SettingsMsg::MaxPreconChanged,
                ),
                labeled_input(
                    "Max Unranked per Pod",
                    &draft.max_unranked,
                    SettingsMsg::MaxUnrankedChanged,
                ),
            ]
            .spacing(12)
            .width(Length::Fill),
            checkbox(draft.strict_calibrate)
                .label("Strict calibration (pair unranked decks strictly with nearest Elo neighbors)")
                .text_size(13)
                .on_toggle(SettingsMsg::StrictCalibrateToggled),
            row![
                space::horizontal(),
                button(text("Reset Defaults").size(13))
                    .padding([8, 12])
                    .style(style::secondary_button)
                    .on_press(SettingsMsg::ResetMatchmakerDefaults),
                button(text("Save Matchmaker Config").font(FONT_BOLD).size(13))
                    .padding([8, 14])
                    .style(style::primary_button)
                    .on_press(SettingsMsg::SaveMatchmakerConfig),
            ]
            .spacing(8)
            .align_y(Vertical::Center),
        ]
        .spacing(12)
        .width(Length::Fill),
    )
    .width(Length::Fill)
    .padding(16)
    .style(style::card)
    .into()
}

fn render_maintenance_card<'a>() -> Element<'a, SettingsMsg> {
    container(
        row![
            column![
                text("Recalculate Ratings").font(FONT_BOLD).size(15).color(COLORS.text),
                text("Replays all recorded games chronologically from Game 1 using the current rating engine parameters.")
                    .size(12)
                    .color(COLORS.subtext0),
            ]
            .spacing(4)
            .width(Length::Fill),
            button(
                row![
                    text(md::MD_REFRESH).size(14),
                    text("Recalculate All Ratings").size(13),
                ]
                .spacing(6)
                .align_y(Vertical::Center),
            )
            .padding([8, 14])
            .style(style::secondary_button)
            .on_press(SettingsMsg::ReloadRatings),
        ]
        .spacing(16)
        .align_y(Vertical::Center)
        .width(Length::Fill),
    )
    .width(Length::Fill)
    .padding(16)
    .style(style::card)
    .into()
}

fn labeled_input<'a, F>(label: &'static str, value: &str, on_input: F) -> Element<'a, SettingsMsg>
where
    F: Fn(String) -> SettingsMsg + 'a,
{
    column![
        text(label).size(12).color(COLORS.subtext1),
        text_input("", value)
            .on_input(on_input)
            .size(13)
            .padding([8, 12])
            .style(style::search_input),
    ]
    .spacing(4)
    .width(Length::FillPortion(1))
    .into()
}
