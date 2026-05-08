// use iced::keyboard::{Event, Key, key::Named};

use iced::{
    Event,
    keyboard::{self, Key, key::Named},
    window,
};

use crate::{app::Message, core::file::FileAction};

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

        match key {
            Key::Named(named) => match named {
                Named::Save => Some(Self::TournFile(FileAction::Save)),
                Named::Open => Some(Self::TournFile(FileAction::Open)),
                Named::New => Some(Self::TournFile(FileAction::RequestNew)),
                Named::Escape => Some(Self::CloseView),
                _ => None,
            },
            Key::Character(c) => {
                if modifiers.command() {
                    match c.as_ref() {
                        "s" if modifiers.shift() => Some(Self::TournFile(FileAction::SaveAs)),
                        "s" => Some(Self::TournFile(FileAction::Save)),
                        "o" => Some(Self::TournFile(FileAction::Open)),
                        _ => None,
                    }
                } else {
                    None
                }
            }
            Key::Unidentified => None,
        }
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
