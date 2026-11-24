mod pod;

use commander_tournament_core::{
    game::{matchup::Matchup, next_mode::NextPlayerMode, record::GameRecord},
    player::{PlayerId, info::PlayerInfo},
    tournament::Tournament,
};
use iced::{
    Element, Length,
    alignment::Vertical,
    widget::{button, column, container, row, space, text},
};
use iced_tea::{Component, Model, Signal};
use nerd_font_symbols::md;
use strum::VariantArray;

use crate::{
    components::{DeckPicker, DeckPickerMsg, empty_card, pill},
    fonts::{FONT_BOLD, FONT_MEDIUM},
    models::tournament::TournamentModel,
    style::{self, COLORS},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, strum::VariantArray, derive_more::Display)]
pub enum PodMode {
    #[default]
    #[display("Auto-Next Balanced Pod")]
    AutoNext,
    #[display("Target Deck Practice")]
    TargetDeck,
    #[display("Manual Pod")]
    Manual,
}

#[derive(Debug)]
pub struct PlayState {
    mode: PodMode,
    next_mode: NextPlayerMode,
    target_picker: DeckPicker,
    manual_pickers: [DeckPicker; 4],
    current_matchup: Option<Matchup>,
    selected_winner: Option<usize>,
    error: Option<String>,
}

impl Default for PlayState {
    fn default() -> Self {
        Self {
            mode: PodMode::AutoNext,
            next_mode: NextPlayerMode::Default,
            target_picker: DeckPicker::active_only("Search target deck for Seat 1...")
                .with_width(Length::Fixed(320.0)),
            manual_pickers: core::array::from_fn(|_| DeckPicker::active_only("Search deck...")),
            current_matchup: None,
            selected_winner: None,
            error: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum PlayMsg {
    SetMode(PodMode),
    SetNextMode(NextPlayerMode),
    TargetPicker(DeckPickerMsg),
    PracticeDeck(PlayerId),
    ManualPicker(usize, DeckPickerMsg),
    GeneratePod,
    SelectWinner(usize),
    ClearWinner,
    ConfirmResult { generate_next: bool },
    ClearPod,
    OpenMoxfield(String),
    OpenAllGoldfish,
    InspectDeck(PlayerId),
}

#[derive(Debug, Clone)]
pub enum PlayOut {
    RecordGame(Box<GameRecord>),
    InspectDeck(PlayerId),
}

impl PlayState {
    fn generate_pod(&mut self, tournament: &Tournament) {
        self.error = None;
        self.selected_winner = None;

        let result = match self.mode {
            PodMode::AutoNext => {
                if let Some(next_id) = self.next_mode.next_player(tournament) {
                    tournament.matchmaker().create_match(next_id)
                } else {
                    self.error = Some("No eligible active decks available for auto-next.".to_owned());
                    return;
                }
            }
            PodMode::TargetDeck => {
                if let Some(target_id) = self.target_picker.selected() {
                    tournament.matchmaker().create_match(target_id)
                } else {
                    self.error = Some("Select a target deck first.".to_owned());
                    return;
                }
            }
            PodMode::Manual => {
                let seats = self.manual_pickers.each_ref().map(DeckPicker::selected);
                let [Some(a), Some(b), Some(c), Some(d)] = seats else {
                    self.error = Some("Select a deck for all 4 seats.".to_owned());
                    return;
                };
                tournament.create_match([a, b, c, d])
            }
        };

        match result {
            Ok(matchup) => {
                self.current_matchup = Some(matchup);
            }
            Err(err) => {
                self.error = Some(err.to_string());
            }
        }
    }
}

impl Model for PlayState {
    type Message = PlayMsg;
    type OutMessage = PlayOut;
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
            PlayMsg::SetMode(mode) => {
                self.mode = mode;
                self.error = None;
                Signal::done()
            }
            PlayMsg::SetNextMode(next_mode) => {
                self.next_mode = next_mode;
                self.error = None;
                Signal::done()
            }
            PlayMsg::TargetPicker(msg) => {
                self.error = None;
                if let Some(tournament) = context.tournament() {
                    let _ = self.target_picker.update(msg, tournament)?;
                }
                Signal::done()
            }
            PlayMsg::PracticeDeck(id) => {
                self.mode = PodMode::TargetDeck;
                self.target_picker.set_selected(Some(id));
                if let Some(tournament) = context.tournament() {
                    self.target_picker.sync(tournament);
                    self.generate_pod(tournament);
                }
                Signal::done()
            }
            PlayMsg::ManualPicker(idx, msg) => {
                self.error = None;
                if let (Some(picker), Some(tournament)) =
                    (self.manual_pickers.get_mut(idx), context.tournament())
                {
                    let _ = picker.update(msg, tournament)?;
                }
                Signal::done()
            }
            PlayMsg::GeneratePod => {
                if let Some(tournament) = context.tournament() {
                    self.generate_pod(tournament);
                }
                Signal::done()
            }
            PlayMsg::SelectWinner(seat_idx) => {
                self.selected_winner = Some(seat_idx);
                Signal::done()
            }
            PlayMsg::ClearWinner => {
                self.selected_winner = None;
                Signal::done()
            }
            PlayMsg::ClearPod => {
                self.current_matchup = None;
                self.selected_winner = None;
                self.error = None;
                Signal::done()
            }
            PlayMsg::ConfirmResult { generate_next } => {
                let Some(seat_idx) = self.selected_winner else {
                    return Signal::done();
                };
                let Some(matchup) = self.current_matchup.take() else {
                    return Signal::done();
                };
                let Some(winner_id) = matchup.ids().get(seat_idx).copied() else {
                    return Signal::done();
                };

                self.selected_winner = None;
                match matchup.record(winner_id) {
                    Ok(record) => {
                        let out = Signal::out(PlayOut::RecordGame(Box::new(record)));
                        if generate_next {
                            Ok(out.chain(Signal::delayed(PlayMsg::GeneratePod)))
                        } else {
                            Ok(out)
                        }
                    }
                    Err(err) => {
                        self.error = Some(err.to_string());
                        Signal::done()
                    }
                }
            }
            PlayMsg::OpenMoxfield(url) => {
                if let Err(err) = opener::open(&url) {
                    log::warn!("Failed to open Moxfield goldfish link {url}: {err}");
                }
                Signal::done()
            }
            PlayMsg::OpenAllGoldfish => {
                let (Some(tournament), Some(matchup)) = (context.tournament(), &self.current_matchup) else {
                    return Signal::done();
                };
                let signals: Vec<Signal<PlayMsg, PlayOut>> = matchup
                    .ids()
                    .into_iter()
                    .filter_map(|id| {
                        tournament
                            .get_player_info(&id)
                            .and_then(PlayerInfo::moxfield_goldfish_link)
                    })
                    .map(|url| Signal::msg(PlayMsg::OpenMoxfield(url)))
                    .collect();

                Ok(Signal::sequence(signals))
            }
            PlayMsg::InspectDeck(id) => Signal::out(PlayOut::InspectDeck(id)).ok(),
        }
    }
}

impl Component for PlayState {
    fn render<'a>(&'a self, context: Self::Context<'a>) -> Element<'a, Self::Message> {
        let Some(tournament) = context.tournament() else {
            return empty_card(
                md::MD_FOLDER_OPEN_OUTLINE,
                "No tournament file open",
                "Create a new file or open an existing .ron file from the top bar to generate pods.",
                None,
            );
        };

        let controls = self.render_controls(tournament);

        let mut layout = column![controls].spacing(16).width(Length::Fill);

        if let Some(err) = &self.error {
            layout = layout.push(
                container(text(err).size(13).color(COLORS.red))
                    .width(Length::Fill)
                    .padding([10, 14])
                    .style(style::card),
            );
        }

        if let Some(matchup) = &self.current_matchup {
            let refreshed = tournament
                .update_match(matchup)
                .unwrap_or_else(|_| matchup.clone());
            layout = layout.push(pod::render_pod_section(
                tournament,
                &refreshed,
                self.mode,
                self.selected_winner,
            ));
        } else {
            layout = layout.push(empty_card(
                md::MD_PLAY_OUTLINE,
                "No active pod",
                "Choose a matchmaker mode above and click Generate to build a 4-deck pod.",
                None,
            ));
        }

        layout.into()
    }
}

impl PlayState {
    fn render_controls<'a>(&'a self, tournament: &'a Tournament) -> Element<'a, PlayMsg> {
        let active_count = tournament
            .players()
            .values()
            .filter(|info| !info.is_archived())
            .count();

        let mode_tabs =
            PodMode::VARIANTS
                .iter()
                .copied()
                .fold(row![].spacing(6).align_y(Vertical::Center), |r, mode| {
                    r.push(
                        button(text(mode.to_string()).font(FONT_MEDIUM).size(13))
                            .padding([8, 12])
                            .style(style::segment_button(mode == self.mode))
                            .on_press(PlayMsg::SetMode(mode)),
                    )
                });

        let pool_badge = pill(
            format!("{active_count} active decks"),
            if active_count >= 4 {
                COLORS.green
            } else {
                COLORS.yellow
            },
        );

        let top_row = row![mode_tabs, space::horizontal(), pool_badge]
            .spacing(12)
            .align_y(Vertical::Center)
            .width(Length::Fill);

        let mode_body: Element<'a, PlayMsg> = match self.mode {
            PodMode::AutoNext => {
                let next_selector = NextPlayerMode::VARIANTS.iter().copied().fold(
                    row![text("Next deck rule:").size(12).color(COLORS.subtext0)]
                        .spacing(6)
                        .align_y(Vertical::Center),
                    |r, nm| {
                        r.push(
                            button(text(nm.to_string()).size(12))
                                .padding([6, 10])
                                .style(style::segment_button(nm == self.next_mode))
                                .on_press(PlayMsg::SetNextMode(nm)),
                        )
                    },
                );

                let mut gen_btn = button(
                    row![
                        text(md::MD_SHUFFLE_VARIANT).size(15),
                        text("Generate Next Pod").font(FONT_BOLD).size(13),
                    ]
                    .spacing(8)
                    .align_y(Vertical::Center),
                )
                .padding([8, 16])
                .style(style::primary_button);

                if active_count >= 4 {
                    gen_btn = gen_btn.on_press(PlayMsg::GeneratePod);
                }

                row![next_selector, space::horizontal(), gen_btn]
                    .spacing(12)
                    .align_y(Vertical::Center)
                    .width(Length::Fill)
                    .into()
            }
            PodMode::TargetDeck => {
                let picker = self.target_picker.view(tournament).map(PlayMsg::TargetPicker);

                let mut gen_btn = button(
                    row![
                        text(md::MD_TARGET).size(15),
                        text("Generate 3 Opponents").font(FONT_BOLD).size(13),
                    ]
                    .spacing(8)
                    .align_y(Vertical::Center),
                )
                .padding([8, 16])
                .style(style::primary_button);

                if active_count >= 4 && self.target_picker.selected().is_some() {
                    gen_btn = gen_btn.on_press(PlayMsg::GeneratePod);
                }

                row![
                    text("Target Deck:").size(12).color(COLORS.subtext0),
                    picker,
                    space::horizontal(),
                    gen_btn,
                ]
                .spacing(10)
                .align_y(Vertical::Center)
                .width(Length::Fill)
                .into()
            }
            PodMode::Manual => {
                let seat_pickers = self.manual_pickers.iter().enumerate().fold(
                    row![].spacing(8).width(Length::Fill),
                    |r, (idx, picker)| {
                        r.push(
                            column![
                                text(format!("Seat {}", idx + 1)).size(11).color(COLORS.subtext0),
                                picker
                                    .view(tournament)
                                    .map(move |m| PlayMsg::ManualPicker(idx, m)),
                            ]
                            .spacing(4)
                            .width(Length::FillPortion(1)),
                        )
                    },
                );

                let all_selected = self.manual_pickers.iter().all(|p| p.selected().is_some());
                let mut build_btn = button(text("Build Pod").font(FONT_BOLD).size(13))
                    .padding([8, 16])
                    .style(style::primary_button);
                if all_selected {
                    build_btn = build_btn.on_press(PlayMsg::GeneratePod);
                }

                column![
                    seat_pickers,
                    row![space::horizontal(), build_btn].width(Length::Fill),
                ]
                .spacing(10)
                .width(Length::Fill)
                .into()
            }
        };

        container(column![top_row, mode_body].spacing(12).width(Length::Fill))
            .width(Length::Fill)
            .padding(16)
            .style(style::card)
            .into()
    }
}
