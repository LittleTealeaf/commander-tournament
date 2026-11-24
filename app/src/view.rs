use commander_tournament_core::{
    config::{game::GameConfig, matchmaker::MatchmakerConfig},
    game::record::GameRecord,
    player::{PlayerId, info::PlayerInfo},
    tournament::Tournament,
};
use iced::{
    Length,
    alignment::{Horizontal, Vertical},
    widget::{button, column, container, row, rule, scrollable, space, text},
};
use iced_tea::{Component, Model, Signal};
use nerd_font_symbols::md;
use strum::VariantArray;

use crate::{
    components::pill,
    fonts::FONT_BOLD,
    models::tournament::TournamentModel,
    screens::{
        ScreenKind,
        decks::{DecksMsg, DecksOut, DecksState},
        history::{MatchHistoryMsg, MatchHistoryOut, MatchHistoryState},
        import::{ImportMsg, ImportOut, ImportState},
        overview::{OverviewMsg, OverviewOut, OverviewState},
        play::{PlayMsg, PlayOut, PlayState},
        settings::{SettingsMsg, SettingsOut, SettingsState},
    },
    style::{self, COLORS},
};

#[derive(Debug, Default)]
pub struct AppView {
    sidebar_collapsed: bool,
    screen: ScreenKind,
    import_open: bool,
    overview: OverviewState,
    play: PlayState,
    decks: DecksState,
    history: MatchHistoryState,
    settings: SettingsState,
    import: ImportState,
}

#[derive(Debug, Clone, derive_more::From)]
pub enum AppViewMsg {
    SetScreen(ScreenKind),
    ToggleSidebar,
    NewFile,
    OpenFile,
    SaveFile,
    SaveFileAs,
    OpenImport,
    CloseImport,
    Overview(OverviewMsg),
    Play(PlayMsg),
    Decks(DecksMsg),
    History(MatchHistoryMsg),
    Settings(SettingsMsg),
    Import(ImportMsg),
}

#[derive(Debug, Clone)]
pub enum AppViewOut {
    NewFile,
    OpenFile,
    SaveFile,
    SaveFileAs,
    ImportGames { source: Box<Tournament>, prepend: bool },
    RecordGame(Box<GameRecord>),
    RegisterDeck(PlayerInfo),
    UpdateDeck(PlayerId, PlayerInfo),
    DeleteDeck(PlayerId),
    MergeDecks { target: PlayerId, other: PlayerId },
    DeleteGame(usize),
    SaveGameConfig(GameConfig),
    SaveMatchmakerConfig(MatchmakerConfig),
    ReloadRatings,
}

impl Model for AppView {
    type Message = AppViewMsg;

    type OutMessage = AppViewOut;

    type Context<'a>
        = &'a TournamentModel
    where
        Self: 'a;

    fn update<'a>(
        &'a mut self,
        message: Self::Message,
        context: Self::Context<'a>,
    ) -> anyhow::Result<iced_tea::Signal<Self::Message, Self::OutMessage>> {
        match message {
            AppViewMsg::ToggleSidebar => {
                self.sidebar_collapsed = !self.sidebar_collapsed;
                Signal::done()
            }
            AppViewMsg::SetScreen(screen_kind) => {
                self.screen = screen_kind;
                self.import_open = false;
                Signal::done()
            }
            AppViewMsg::NewFile => Signal::out(AppViewOut::NewFile).ok(),
            AppViewMsg::OpenFile => Signal::out(AppViewOut::OpenFile).ok(),
            AppViewMsg::SaveFile => Signal::out(AppViewOut::SaveFile).ok(),
            AppViewMsg::SaveFileAs => Signal::out(AppViewOut::SaveFileAs).ok(),
            AppViewMsg::OpenImport => {
                self.import_open = true;
                Signal::done()
            }
            AppViewMsg::CloseImport => {
                self.import_open = false;
                Signal::done()
            }
            AppViewMsg::Overview(msg) => self.overview.map_update(msg, context, |out| {
                Ok(match out {
                    OverviewOut::InspectDeck(id) => Signal::msg(AppViewMsg::SetScreen(ScreenKind::Decks))
                        .chain(Signal::msg(AppViewMsg::Decks(DecksMsg::OpenDetail(id)))),
                    OverviewOut::GoToPlay => Signal::msg(AppViewMsg::SetScreen(ScreenKind::Play)),
                })
            }),
            AppViewMsg::Play(msg) => self.play.map_update(msg, context, |out| {
                Ok(match out {
                    PlayOut::RecordGame(record) => Signal::out(AppViewOut::RecordGame(record)),
                    PlayOut::InspectDeck(id) => Signal::msg(AppViewMsg::SetScreen(ScreenKind::Decks))
                        .chain(Signal::msg(AppViewMsg::Decks(DecksMsg::OpenDetail(id)))),
                })
            }),
            AppViewMsg::Decks(msg) => self.decks.map_update(msg, context, |out| {
                Ok(match out {
                    DecksOut::RegisterDeck(info) => Signal::out(AppViewOut::RegisterDeck(info)),
                    DecksOut::UpdateDeck(id, info) => Signal::out(AppViewOut::UpdateDeck(id, info)),
                    DecksOut::DeleteDeck(id) => Signal::out(AppViewOut::DeleteDeck(id)),
                    DecksOut::MergeDecks { target, other } => {
                        Signal::out(AppViewOut::MergeDecks { target, other })
                    }
                    DecksOut::PracticeDeck(id) => Signal::msg(AppViewMsg::SetScreen(ScreenKind::Play))
                        .chain(Signal::msg(AppViewMsg::Play(PlayMsg::PracticeDeck(id)))),
                })
            }),
            AppViewMsg::History(msg) => self.history.map_update(msg, context, |out| {
                Ok(match out {
                    MatchHistoryOut::DeleteGame(gid) => Signal::out(AppViewOut::DeleteGame(gid)),
                    MatchHistoryOut::OpenImport => Signal::msg(AppViewMsg::OpenImport),
                })
            }),
            AppViewMsg::Settings(msg) => self.settings.map_update(msg, context, |out| {
                Ok(match out {
                    SettingsOut::SaveGameConfig(cfg) => Signal::out(AppViewOut::SaveGameConfig(cfg)),
                    SettingsOut::SaveMatchmakerConfig(cfg) => {
                        Signal::out(AppViewOut::SaveMatchmakerConfig(cfg))
                    }
                    SettingsOut::ReloadRatings => Signal::out(AppViewOut::ReloadRatings),
                })
            }),
            AppViewMsg::Import(msg) => self.import.map_update(msg, context, |out| {
                Ok(match out {
                    ImportOut::Close => Signal::msg(AppViewMsg::CloseImport),
                    ImportOut::Apply { source, prepend } => {
                        Signal::out(AppViewOut::ImportGames { source, prepend })
                            .chain(Signal::msg(AppViewMsg::SetScreen(ScreenKind::MatchHistory)))
                    }
                })
            }),
        }
    }
}

impl Component for AppView {
    fn render<'a>(&'a self, context: Self::Context<'a>) -> iced::Element<'a, Self::Message> {
        let screen_view: iced::Element<'a, AppViewMsg> = if self.import_open {
            self.import.view_into(context)
        } else {
            match self.screen {
                ScreenKind::Overview => self.overview.view_into(context),
                ScreenKind::Play => self.play.view_into(context),
                ScreenKind::Decks => self.decks.view_into(context),
                ScreenKind::MatchHistory => self.history.view_into(context),
                ScreenKind::Settings => self.settings.view_into(context),
            }
        };

        let main_column = column![
            topbar(self.screen, self.import_open, context),
            scrollable(container(screen_view).width(Length::Fill).padding(20))
                .width(Length::Fill)
                .height(Length::Fill),
        ]
        .width(Length::Fill)
        .height(Length::Fill);

        row![sidebar(self.screen, self.sidebar_collapsed), main_column]
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

fn sidebar(active: ScreenKind, collapsed: bool) -> iced::Element<'static, AppViewMsg> {
    let header: iced::Element<'static, AppViewMsg> = if collapsed {
        container(
            button(text(md::MD_MENU).size(18))
                .padding([6, 8])
                .style(style::icon_button)
                .on_press(AppViewMsg::ToggleSidebar),
        )
        .width(Length::Fill)
        .align_x(Horizontal::Center)
        .into()
    } else {
        row![
            text("Goldfish Lab").font(FONT_BOLD).size(18),
            space::horizontal(),
            button(text(md::MD_DOCK_LEFT).size(18))
                .padding([6, 8])
                .style(style::icon_button)
                .on_press(AppViewMsg::ToggleSidebar),
        ]
        .align_y(Vertical::Center)
        .into()
    };

    let col = column![header, rule::horizontal(1)].spacing(8);

    let col = ScreenKind::VARIANTS.iter().copied().fold(col, |col, screen| {
        let is_active = screen == active;

        let content: iced::Element<'static, AppViewMsg> = if collapsed {
            container(text(screen.icon()).size(20))
                .width(Length::Fill)
                .align_x(Horizontal::Center)
                .into()
        } else {
            row![text(screen.icon()).size(16), text(screen.to_string()).size(13)]
                .spacing(10)
                .align_y(Vertical::Center)
                .into()
        };

        let btn = button(content)
            .width(Length::Fill)
            .padding(if collapsed { [10, 0] } else { [10, 12] })
            .style(style::nav_button(is_active))
            .on_press(AppViewMsg::SetScreen(screen));

        col.push(btn)
    });

    container(col)
        .width(if collapsed { 64.0 } else { 210.0 })
        .height(Length::Fill)
        .padding(if collapsed { 12 } else { 16 })
        .style(style::sidebar_container)
        .into()
}

fn topbar(
    active: ScreenKind,
    import_open: bool,
    context: &TournamentModel,
) -> iced::Element<'static, AppViewMsg> {
    let (status_label, status_accent) = if context.tournament().is_none() {
        ("No File", COLORS.overlay1)
    } else if context.modified() {
        ("Unsaved", COLORS.yellow)
    } else {
        ("Saved", COLORS.green)
    };

    let status_badge = pill(status_label, status_accent);
    let title = if import_open {
        "Import Games".to_owned()
    } else {
        active.to_string()
    };

    let left = row![text(title).font(FONT_BOLD).size(22), status_badge]
        .spacing(12)
        .align_y(Vertical::Center);

    let has_file = context.tournament().is_some();

    let mut save_btn = button(text("Save").size(13))
        .padding([8, 14])
        .style(style::secondary_button);
    let mut save_as_btn = button(text("Save As").size(13))
        .padding([8, 14])
        .style(style::secondary_button);

    if has_file {
        save_btn = save_btn.on_press(AppViewMsg::SaveFile);
        save_as_btn = save_as_btn.on_press(AppViewMsg::SaveFileAs);
    }

    let right = row![
        button(text("New").size(13))
            .padding([8, 14])
            .style(style::secondary_button)
            .on_press(AppViewMsg::NewFile),
        button(text("Open").size(13))
            .padding([8, 14])
            .style(style::secondary_button)
            .on_press(AppViewMsg::OpenFile),
        save_btn,
        save_as_btn,
        button(text("Import Games").font(FONT_BOLD).size(13))
            .padding([8, 14])
            .style(style::primary_button)
            .on_press(AppViewMsg::OpenImport),
    ]
    .spacing(8)
    .align_y(Vertical::Center);

    container(row![left, space::horizontal(), right].align_y(Vertical::Center))
        .width(Length::Fill)
        .padding([12, 20])
        .style(style::topbar_container)
        .into()
}
