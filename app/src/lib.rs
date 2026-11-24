pub mod components;
pub mod events;
pub mod fonts;
pub mod icons;
pub mod models;
pub mod screens;
pub mod services;
pub mod style;
pub mod update;
pub mod view;

use iced::{
    Background, Border, Color, Element, Length, Program, Subscription, Task,
    alignment::{Horizontal, Vertical},
    event,
    widget::{button, column, container, mouse_area, opaque, row, space, stack, text},
    window,
};
pub use iced_tea::{App as IcedTeaApp, Component, HandleMessage, Model, Signal};
use nerd_font_symbols::md;

use crate::{
    fonts::{FONT_BOLD, FONT_BYTES, FONT_NORMAL},
    models::{state::AppState, tournament::TournamentModel},
    style::COLORS,
    update::Message,
    view::AppView,
};

#[derive(Debug, Default)]
pub struct ComTourApp {
    state: Option<AppState>,
    tourn: TournamentModel,
    view: AppView,
    error_dialog: Option<String>,
}

impl ComTourApp {
    #[must_use]
    pub fn title(&self) -> String {
        const APP_TITLE: &str = "Commander Tournament";
        APP_TITLE.to_owned()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        event::listen_with(|event, _status, _window| -> Option<Message> { Message::from_event(event) })
    }
}

impl iced_tea::App for ComTourApp {
    type Message = Message;

    fn boot() -> (Self, Option<Task<Self::Message>>) {
        (
            Self::default(),
            Some(Task::future(async {
                Message::AppStateLoaded(AppState::load().await.ok())
            })),
        )
    }

    fn config<P>(
        app: iced::Application<P>,
    ) -> iced::Application<impl Program<State = Self, Message = Self::Message, Theme = iced::Theme>>
    where
        P: Program<State = Self, Message = Self::Message, Theme = iced::Theme>,
    {
        let settings = window::Settings {
            exit_on_close_request: false,
            ..Default::default()
        };

        let app = app
            .title("Commander Tournament")
            .window(settings)
            .subscription(Self::subscription)
            .style(style::app_style);

        FONT_BYTES
            .into_iter()
            .fold(app, iced::Application::font)
            .default_font(FONT_NORMAL)
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let base = self.view.view_into(&self.tourn);

        let Some(error_msg) = &self.error_dialog else {
            return base;
        };

        stack![base, error_modal(error_msg)].into()
    }

    fn update(&mut self, message: Self::Message) -> anyhow::Result<Signal<Self::Message, ()>> {
        <Self as Model>::update(self, message, ())
    }

    fn on_error(&mut self, error: &anyhow::Error) {
        log::error!("Error raised: {error:#}");
        self.error_dialog = Some(format!("{error:#}"));
    }
}

fn error_modal(error_msg: &str) -> Element<'_, Message> {
    let dialog = opaque(
        container(
            column![
                row![
                    text(md::MD_ALERT_CIRCLE_OUTLINE).size(20).color(COLORS.red),
                    text("Error Raised").font(FONT_BOLD).size(16).color(COLORS.text),
                ]
                .spacing(10)
                .align_y(Vertical::Center),
                text(error_msg).size(13).color(COLORS.subtext1),
                row![
                    space::horizontal(),
                    button(text("OK").font(FONT_BOLD).size(13))
                        .padding([8, 20])
                        .style(style::primary_button)
                        .on_press(Message::DismissError),
                ]
                .width(Length::Fill),
            ]
            .spacing(14)
            .width(Length::Fill),
        )
        .max_width(480)
        .padding(20)
        .style(|_theme| container::Style {
            background: Some(Background::Color(COLORS.surface0.into())),
            text_color: Some(COLORS.text.into()),
            border: Border {
                color: COLORS.red.into(),
                width: 1.5,
                radius: 10.0.into(),
            },
            ..Default::default()
        }),
    );

    opaque(
        mouse_area(
            container(dialog)
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(Horizontal::Center)
                .align_y(Vertical::Center)
                .padding(24)
                .style(|_theme| {
                    let mut backdrop: Color = COLORS.crust.into();
                    backdrop.a = 0.75;
                    container::Style {
                        background: Some(Background::Color(backdrop)),
                        ..Default::default()
                    }
                }),
        )
        .on_press(Message::DismissError),
    )
}
