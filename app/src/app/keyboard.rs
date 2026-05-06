use iced::keyboard::{Event, Key, key::Named};

use crate::{app::Message, core::file::FileAction};

impl Message {
    #[must_use]
    pub fn from_keyboard_event(event: Event) -> Option<Self> {
        match event {
            // Handle the dedicated physical Save key if present
            Event::KeyPressed {
                key: Key::Named(named),
                ..
            } => match named {
                Named::Save => Some(Self::TournFile(FileAction::Save)),
                Named::Open => Some(Self::TournFile(FileAction::Open)),
                Named::New => Some(Self::TournFile(FileAction::RequestNew)),
                Named::Escape => Some(Self::CloseView),
                _ => None,
            },

            Event::KeyPressed {
                key: Key::Character(c),
                modifiers,
                ..
            } if modifiers.command() => match c.as_ref() {
                "s" if modifiers.shift() => Some(Self::TournFile(FileAction::SaveAs)),
                "s" => Some(Self::TournFile(FileAction::Save)),
                "o" => Some(Self::TournFile(FileAction::Open)),
                _ => None,
            },
            _ => None,
        }
    }
}
