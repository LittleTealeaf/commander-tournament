use iced::{
    Length,
    alignment::{Horizontal, Vertical},
    widget::{button, column, container, row, rule, scrollable, space, text},
};
use iced_tea::{Component, Model, Signal};
use nerd_font_symbols::md;
use strum::VariantArray;

use crate::{fonts::FONT_BOLD, models::tournament::TournamentModel, screens::ScreenKind, style};

const COLORS: catppuccin::FlavorColors = catppuccin::PALETTE.mocha.colors;

#[derive(Debug, Default)]
pub struct AppView {
    sidebar_collapsed: bool,
    screen: ScreenKind,
}

#[derive(Debug, Clone)]
pub enum AppViewMsg {
    SetScreen(ScreenKind),
    ToggleSidebar,
    NewFile,
    OpenFile,
    SaveFile,
    OpenImport,
}

#[derive(Debug, Clone)]
pub enum AppViewOut {
    NewFile,
    OpenFile,
    SaveFile,
    OpenImport,
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
        _context: Self::Context<'a>,
    ) -> anyhow::Result<iced_tea::Signal<Self::Message, Self::OutMessage>> {
        match message {
            AppViewMsg::ToggleSidebar => {
                self.sidebar_collapsed = !self.sidebar_collapsed;
                Signal::done()
            }
            AppViewMsg::SetScreen(screen_kind) => {
                self.screen = screen_kind;
                Signal::done()
            }
            AppViewMsg::NewFile => Signal::out(AppViewOut::NewFile).ok(),
            AppViewMsg::OpenFile => Signal::out(AppViewOut::OpenFile).ok(),
            AppViewMsg::SaveFile => Signal::out(AppViewOut::SaveFile).ok(),
            AppViewMsg::OpenImport => Signal::out(AppViewOut::OpenImport).ok(),
        }
    }
}

impl Component for AppView {
    fn render<'a>(&'a self, context: Self::Context<'a>) -> iced::Element<'a, Self::Message> {
        let main_column = column![
            topbar(self.screen, context),
            scrollable(
                container(
                    text(format!("{} view coming soon...", self.screen))
                        .size(14)
                        .color(COLORS.subtext1),
                )
                .width(Length::Fill)
                .padding(20),
            )
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

fn topbar(active: ScreenKind, context: &TournamentModel) -> iced::Element<'static, AppViewMsg> {
    let (status_label, status_accent) = if context.tournament().is_none() {
        ("No File", COLORS.overlay1)
    } else if context.modified() {
        ("Unsaved", COLORS.yellow)
    } else {
        ("Saved", COLORS.green)
    };

    let status_badge = container(text(status_label).size(11))
        .padding([3, 10])
        .style(style::pill_badge(status_accent));

    let left = row![text(active.to_string()).font(FONT_BOLD).size(22), status_badge,]
        .spacing(12)
        .align_y(Vertical::Center);

    let right = row![
        button(text("New").size(13))
            .padding([8, 14])
            .style(style::secondary_button)
            .on_press(AppViewMsg::NewFile),
        button(text("Open").size(13))
            .padding([8, 14])
            .style(style::secondary_button)
            .on_press(AppViewMsg::OpenFile),
        button(text("Save").size(13))
            .padding([8, 14])
            .style(style::secondary_button)
            .on_press(AppViewMsg::SaveFile),
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
