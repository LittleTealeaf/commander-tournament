// use iced::keyboard::{Event, Key, key::Named};

use iced::{Event, keyboard, window};

use crate::{models::tournament::TournamentMessage, update::Message};

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
        let _ = key;
        let _ = modifiers;
        None
    }

    fn from_window_event(event: window::Event) -> Option<Self> {
        match event {
            window::Event::CloseRequested => Some(Self::RequestQuit),
            window::Event::FileDropped(path_buf) => {
                Some(Self::Tourn(TournamentMessage::RequestOpen(path_buf)))
            }
            _ => None,
        }
    }
}
