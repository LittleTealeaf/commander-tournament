use edh_tourn::player::PlayerId;
use iced::Task;

use crate::{
    App,
    app::ViewMsg,
    core::{
        file::FileAction,
        state::{AppState, AppStateMsg},
        tournament::TournamentAction,
    },
    effect::Effect,
    home::HomeMsg,
    traits::ComponentUpdate,
};

#[derive(Debug, Clone, derive_more::From)]
pub enum Message {
    OnBoot,
    Nothing,
    AppState(AppStateMsg),
    AppStateLoaded(Option<AppState>),
    Tournament(TournamentAction),
    TournFile(FileAction),
    OpenPlayerDetails(Option<PlayerId>),
    CloseView,
    #[from(ignore)]
    Error(String),
    Home(HomeMsg),
    View(ViewMsg),
    #[from(ignore)]
    OpenLink(String),
}

impl App {
    fn process_effect(&mut self, effect: Effect<Message, ()>) -> anyhow::Result<Task<Message>> {
        match effect {
            Effect::Msg(message) => self.process_message(message),
            Effect::Out(()) | Effect::Done => Ok(Task::none()),
            Effect::Global(message) => {
                let effect = self.update(message, ())?;
                self.process_effect(effect)
            }
            Effect::Task(task) => Ok(task),
            Effect::Batch(effects) => Ok(Task::batch(effects.into_iter().map(|effect| {
                self.process_effect(effect)
                    .unwrap_or_else(|err| Task::done(Message::Error(err.to_string())))
            }))),
            Effect::Sequence(effects) => {
                let mut task = Task::none();
                for effect in effects {
                    task = task.chain(self.process_effect(effect)?);
                }
                Ok(task)
            }
        }
    }

    fn process_message(&mut self, message: Message) -> anyhow::Result<Task<Message>> {
        self.update(message, ())
            .and_then(|effect| self.process_effect(effect))
    }

    pub fn handle_update(&mut self, message: Message) -> Task<Message> {
        self.process_message(message).unwrap_or_else(|error| {
            eprintln!("Error: {error}");
            self.error(format!("{error}"));
            Task::none()
        })
    }
}
