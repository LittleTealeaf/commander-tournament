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
    modals::ModalMsg,
    traits::ComponentUpdate,
    views::play::PlayMode,
};

#[derive(Debug, Clone, derive_more::From)]
pub enum Message {
    Nothing,
    OpenPlay(PlayMode),
    AppState(AppStateMsg),
    AppStateLoaded(Option<AppState>),
    Tournament(TournamentAction),
    TournFile(FileAction),
    OpenPlayerDetails(Option<PlayerId>),
    OpenPlayConfig,
    OpenGameConfig,
    CloseView,
    #[from(ignore)]
    Error(String),
    Home(HomeMsg),
    View(ViewMsg),
    #[from(ignore)]
    OpenLink(String),
    CloseModal,
    Modal(ModalMsg),
    QuitRequested,
    QuitConfirm(bool),
}

impl App {
    fn process_effect(&mut self, effect: Effect<Message, ()>) -> anyhow::Result<Task<Message>> {
        match effect {
            Effect::OnError(effect, on_error) => {
                let result = self.process_effect(*effect);
                match result {
                    Ok(task) => Ok(task),
                    Err(error) => {
                        eprintln!("Gracefully caught error: {error:#}");
                        on_error.map_or_else(
                            || Ok(Task::none()),
                            |message| self.process_message(message),
                        )
                    }
                }
            }
            Effect::Msg(message) => self.process_message(message),
            Effect::Out(()) | Effect::Done => Ok(Task::none()),
            Effect::Task(task) => Ok(task),
            Effect::Batch(effects) => {
                let mut errors = Vec::new();
                let mut tasks = Vec::new();
                for effect in effects {
                    match self.process_effect(effect) {
                        Ok(task) => tasks.push(task),
                        Err(error) => errors.push(error),
                    }
                }

                if errors.is_empty() {
                    Ok(Task::batch(tasks))
                } else {
                    Err(anyhow::anyhow!("Multiple errors occurred: {errors:?}"))
                }
            }
            Effect::Sequence(effects) => {
                let mut task = Task::none();
                for effect in effects {
                    task = task.chain(self.process_effect(effect)?);
                }
                Ok(task)
            }
            Effect::Modal(modal) => {
                self.modals.push(modal);
                Ok(Task::none())
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
