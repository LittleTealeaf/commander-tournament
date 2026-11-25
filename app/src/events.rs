use iced::{
    Event,
    keyboard::{self, Key, key::Named},
    window,
};

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

        match key {
            Key::Named(Named::Save) => Some(Self::Tourn(TournamentMessage::SAVE)),
            Key::Character(c) if modifiers.command() && !modifiers.alt() && c.eq_ignore_ascii_case("s") => {
                if modifiers.shift() {
                    Some(Self::Tourn(TournamentMessage::SAVE_AS))
                } else {
                    Some(Self::Tourn(TournamentMessage::SAVE))
                }
            }
            _ => None,
        }
    }

    fn from_window_event(event: window::Event) -> Option<Self> {
        match event {
            window::Event::CloseRequested => Some(Self::Quit),
            window::Event::FileDropped(path_buf) => {
                Some(Self::Tourn(TournamentMessage::RequestOpen(path_buf)))
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use iced::{
        Event,
        keyboard::{self, Key, Location, Modifiers, key::Named},
    };

    use crate::{
        models::{file::FileModelMsg, tournament::TournamentMessage},
        update::Message,
    };

    fn key_pressed(key: Key, modifiers: Modifiers) -> Event {
        Event::Keyboard(keyboard::Event::KeyPressed {
            key: key.clone(),
            modified_key: key,
            physical_key: keyboard::key::Physical::Unidentified(keyboard::key::NativeCode::Unidentified),
            location: Location::Standard,
            modifiers,
            text: None,
            repeat: false,
        })
    }

    #[test]
    fn ctrl_s_triggers_save() {
        let event = key_pressed(Key::Character("s".into()), Modifiers::COMMAND);
        assert!(
            matches!(
                Message::from_event(event),
                Some(Message::Tourn(TournamentMessage::File(FileModelMsg::Save)))
            ),
            "Ctrl+S should produce TournamentMessage::SAVE"
        );
    }

    #[test]
    fn ctrl_uppercase_s_triggers_save() {
        let event = key_pressed(Key::Character("S".into()), Modifiers::COMMAND);
        assert!(
            matches!(
                Message::from_event(event),
                Some(Message::Tourn(TournamentMessage::File(FileModelMsg::Save)))
            ),
            "Ctrl+S with Caps Lock should produce TournamentMessage::SAVE"
        );
    }

    #[test]
    fn ctrl_shift_s_triggers_save_as() {
        let event = key_pressed(Key::Character("s".into()), Modifiers::COMMAND | Modifiers::SHIFT);
        assert!(
            matches!(
                Message::from_event(event),
                Some(Message::Tourn(TournamentMessage::File(FileModelMsg::SaveAs)))
            ),
            "Ctrl+Shift+S should produce TournamentMessage::SAVE_AS"
        );
    }

    #[test]
    fn named_save_key_triggers_save() {
        let event = key_pressed(Key::Named(Named::Save), Modifiers::empty());
        assert!(
            matches!(
                Message::from_event(event),
                Some(Message::Tourn(TournamentMessage::File(FileModelMsg::Save)))
            ),
            "Named::Save key should produce TournamentMessage::SAVE"
        );
    }

    #[test]
    fn plain_s_does_not_trigger_save() {
        let event = key_pressed(Key::Character("s".into()), Modifiers::empty());
        assert!(
            Message::from_event(event).is_none(),
            "Plain 's' without command modifier should be ignored"
        );
    }

    #[test]
    fn ctrl_alt_s_does_not_trigger_save() {
        let event = key_pressed(Key::Character("s".into()), Modifiers::COMMAND | Modifiers::ALT);
        assert!(
            Message::from_event(event).is_none(),
            "Ctrl+Alt+S should be ignored"
        );
    }
}
