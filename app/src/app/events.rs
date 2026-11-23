// use iced::keyboard::{Event, Key, key::Named};

use iced::{Event, keyboard, window};

use crate::{app::update::Message, core::file::FileAction};

impl Message {
    #[must_use]
    pub fn from_event(event: Event) -> Option<Self> {
        match event {
            Event::Keyboard(event) => Self::from_keyboard_event(event),
            Event::Window(event) => Self::from_window_event(event),
            _ => None,
        }
    }

    fn from_keyboard_event(event: keyboard::Event) -> Option<Self> {
        let keyboard::Event::KeyPressed { key, modifiers, .. } = event else {
            return None;
        };
        None
    }

    fn from_window_event(event: window::Event) -> Option<Self> {
        match event {
            window::Event::CloseRequested => Some(Self::QuitRequested),
            window::Event::FileDropped(path_buf) => {
                Some(Self::TournFile(FileAction::RequestOpenFile(path_buf)))
            }
            _ => None,
        }
    }
}
