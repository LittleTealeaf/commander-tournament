pub mod file;

use edh_tourn::Tournament;
use iced::Task;
use opener::open_browser;

use crate::{App, logic::file::FileMessage, traits::HandleMessage};

#[derive(Clone, Default)]
pub enum Message {
    #[default]
    None,
    OpenLink(String),
    Error(Option<String>),
    File(FileMessage),
    LoadTournament(Box<Tournament>),
}

impl Message {
    #[allow(clippy::unnecessary_wraps)]
    fn done() -> anyhow::Result<Task<Self>> {
        Ok(Task::none())
    }

    fn handle_error_fn<T, E: ToString, M: Into<Self>>(
        on_ok: impl Fn(T) -> M,
    ) -> impl Fn(Result<T, E>) -> Self {
        move |result: Result<T, E>| match result {
            Ok(value) => on_ok(value).into(),
            Err(error) => Self::Error(Some(error.to_string())),
        }
    }

    fn handle_option_fn<T, M: Into<Self>>(on_some: impl Fn(T) -> M) -> impl Fn(Option<T>) -> Self {
        move |option: Option<T>| option.map_or(Self::None, |value| on_some(value).into())
    }
}

impl HandleMessage<Message> for App {
    fn update(&mut self, msg: Message) -> anyhow::Result<iced::Task<Message>> {
        match msg {
            Message::OpenLink(link) => {
                open_browser(link)?;
                Message::done()
            }
            Message::None => Message::done(),
            Message::Error(error) => {
                self.error = error;
                Message::done()
            }
            Message::File(file_message) => self.update(file_message),
            Message::LoadTournament(tournament) => {
                self.tournament = *tournament;
                Message::done()
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

    mod handle_error_fn {
        use crate::logic::Message;

        #[test]
        fn error_returns_error() {
            let error_msg = String::from("Error");
            let res: Result<(), String> = Err(error_msg);
            let func = Message::handle_error_fn(Message::from);
            let output = func(res);
            assert!(matches!(output, Message::Error(Some(_))));
        }
    }
}
