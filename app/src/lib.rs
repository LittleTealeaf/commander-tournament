pub mod app;
pub mod components;
pub mod core;
pub mod fonts;
pub mod home;
pub mod icons;
pub mod popup;
pub mod services;
pub mod views;

use std::path::PathBuf;

use edh_tourn::tournament::Tournament;
use iced::{Program, Subscription, Task, event, window};
pub use iced_tea::{App as IcedTeaApp, Component, HandleMessage, Model, Signal};

use crate::{
    app::{Message, View},
    core::{file::FileAction, state::AppState},
    fonts::{FONT_BYTES, FONT_NORMAL},
};

#[derive(Debug, Default)]
pub struct ComTourApp {
    tournament: Tournament,
    modified: bool,
    is_saving: bool,
    menu: app::Menu,
    home: home::Home,
    file: Option<PathBuf>,
    views: Vec<View>,
    state: Option<AppState>,
    close_requested: bool,
    // If Some, then show a confirmation dialog
    // that performs the given action on confirmation
    overwrite_requested: Option<FileAction>,
    error: Option<String>,
}

impl ComTourApp {
    #[must_use]
    pub fn boot() -> (Self, Option<Task<Message>>) {
        (
            Self::default(),
            Some(Task::future(async {
                Message::AppStateLoaded(AppState::load().await.ok())
            })),
        )
    }

    #[must_use]
    pub const fn tournament(&self) -> &Tournament {
        &self.tournament
    }

    #[must_use]
    pub fn title(&self) -> String {
        const APP_TITLE: &str = "Commander Tournament";
        if self.modified {
            format!("* {APP_TITLE}")
        } else {
            APP_TITLE.to_owned()
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        event::listen_with(|event, _status, _window| -> Option<Message> { Message::from_event(event) })
    }
}

impl iced_tea::App for ComTourApp {
    type Message = Message;

    fn boot() -> (Self, Option<Task<Self::Message>>) {
        Self::boot()
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
            .theme(iced::theme::Theme::CatppuccinMocha);

        FONT_BYTES
            .into_iter()
            .fold(app, iced::Application::font)
            .default_font(FONT_NORMAL)
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        self.handle_view()
    }

    fn update(&mut self, message: Self::Message) -> anyhow::Result<Signal<Self::Message, ()>> {
        <Self as Model>::update(self, message, ())
    }

    fn on_error(&mut self, error: &anyhow::Error) {
        log::error!("Application Error: {error:#}");
        self.error = Some(error.to_string());
    }
}
