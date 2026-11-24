pub mod app;
pub mod core;
pub mod fonts;
pub mod icons;
pub mod models;
pub mod services;

use iced::{Program, Subscription, Task, event, window};
pub use iced_tea::{App as IcedTeaApp, Component, HandleMessage, Model, Signal};

use crate::{
    app::Message,
    core::state::AppState,
    fonts::{FONT_BYTES, FONT_NORMAL},
    models::tournament::TournamentModel,
};

#[derive(Debug, Default)]
pub struct ComTourApp {
    state: Option<AppState>,
    tourn: TournamentModel,
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
            .theme(iced::theme::Theme::CatppuccinMocha);

        FONT_BYTES
            .into_iter()
            .fold(app, iced::Application::font)
            .default_font(FONT_NORMAL)
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        todo!()
    }

    fn update(&mut self, message: Self::Message) -> anyhow::Result<Signal<Self::Message, ()>> {
        <Self as Model>::update(self, message, ())
    }

    fn on_error(&mut self, error: &anyhow::Error) {
        let _ = rfd::AsyncMessageDialog::new()
            .set_level(rfd::MessageLevel::Error)
            .set_title("Error Raised")
            .set_description(error.to_string())
            .set_buttons(rfd::MessageButtons::Ok)
            .show();
    }
}
