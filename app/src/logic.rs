pub mod file;

use edh_tourn::Tournament;
use iced::Task;

use crate::{App, logic::file::FileMessage, traits::HandleMessage};

#[derive(Clone, Default)]
pub enum Message {
    #[default]
    None,
    Error(Option<String>),
    File(FileMessage),
    LoadTournament(Box<Tournament>),
}

impl Message {
    fn handle_error_fn<T, E: ToString, M: Into<Self>>(
        on_ok: impl Fn(T) -> M,
    ) -> impl Fn(Result<T, E>) -> Self {
        move |result: Result<T, E>| match result {
            Ok(value) => on_ok(value).into(),
            Err(error) => Self::Error(Some(error.to_string())),
        }
    }
}

impl HandleMessage<Message> for App {
    fn update(&mut self, msg: Message) -> anyhow::Result<iced::Task<Message>> {
        match msg {
            Message::None => Ok(Task::none()),
            Message::Error(error) => {
                self.error = error;
                Ok(Task::none())
            }
            Message::File(file_message) => self.update(file_message),
            Message::LoadTournament(tournament) => {
                self.tournament = *tournament;
                Ok(Task::none())
            }
        }
    }
}

impl From<()> for Message {
    fn from((): ()) -> Self {
        Self::None
    }
}

#[cfg(test)]
mod tests {
    use edh_tourn::Tournament;

    use crate::{App, logic::Message, traits::HandleMessage};

    #[test]
    fn error_sets_correctly() {
        let mut app = App::default();
        let _ = app
            .update(Message::Error(Some("error".to_owned())))
            .unwrap();
        assert_eq!(app.error, Some("error".to_owned()));

        let _ = app.update(Message::Error(None)).unwrap();
        assert!(app.error.is_none());
    }

    #[test]
    fn load_tournaments_from_message() {
        let mut app = App::default();
        let tourn = Tournament::sample_game();
        assert_ne!(app.tournament, tourn);
        let _ = app
            .update(Message::LoadTournament(tourn.clone().into()))
            .unwrap();
        assert_eq!(app.tournament, tourn);
    }
}
