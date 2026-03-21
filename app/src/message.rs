pub mod file;
pub mod tournament;
use iced::Task;

use crate::{
    App,
    message::file::TournFileMessage,
    settings::AppSettings,
    traits::{ComponentUpdate, Effect, HandleMessage},
    views::{self, View, error, home, player_details},
};

#[derive(Debug, Clone, derive_more::From)]
pub enum Message {
    OnBoot,
    SettingsLoaded(Option<AppSettings>),
    Tournament(tournament::Action),
    TournFile(TournFileMessage),
    Error(String),
    ViewHome(home::Message),
    ViewError(views::error::Message),
    ViewPlayer(views::player_details::Message),
}

impl App {
    pub fn handle_update(&mut self, message: Message) -> Task<Message> {
        match self.update(message, ()) {
            Ok(effect) => effect.task,
            Err(error) => {
                self.views
                    .push(View::Error(error::State::new(error.to_string())));
                Task::none()
            }
        }
    }
}

impl ComponentUpdate for App {
    fn update(
        &mut self,
        message: Self::Message,
        (): Self::Context<'_>,
    ) -> anyhow::Result<crate::traits::Effect<Self::Message, Self::OutMessage>> {
        match message {
            Message::SettingsLoaded(maybe_settings) => {
                let Some(settings) = maybe_settings else {
                    return Effect::ok();
                };
                let message = settings
                    .last_opened()
                    .as_ref()
                    .map(|path| TournFileMessage::LoadTournament(path.clone()));

                self.settings = Some(settings);

                message.map_or_else(Effect::ok, |message| self.handle_message(message, ()))
            }
            Message::OnBoot => Effect::task(Task::perform(
                async { AppSettings::load().await.ok() },
                Message::SettingsLoaded,
            )),
            Message::Tournament(action) => self.handle_message(action, ()),
            Message::ViewHome(message) => self.handle_message(message, ()),
            Message::ViewError(error) => self.handle_message(error, ()),
            Message::ViewPlayer(message) => self.handle_message(message, ()),
            Message::TournFile(message) => self.handle_message(message, ()),
            Message::Error(error) => Err(anyhow::anyhow!("{error}")),
        }
    }
}

impl HandleMessage<home::Message> for App {
    fn handle_message(
        &mut self,
        message: home::Message,
        (): Self::Context<'_>,
    ) -> anyhow::Result<Effect<Self::Message, Self::OutMessage>> {
        self.home
            .handle_message(message, &self.tournament)?
            .map(|message| match message {
                home::OutMessage::OpenPlayerDetails(id) => {
                    let player = id.and_then(|id| self.tournament.get_registered_player(id));
                    self.views
                        .push(View::PlayerDetails(player_details::State::new(player)));
                    Effect::ok()
                }
                home::OutMessage::RegisterRecord(game_record) => {
                    self.handle_message(tournament::Action::Record(game_record), ())
                }
            })
    }
}

macro_rules! handle_view {
    ($self: ident, $variant:ident, $state:ident, $msg:expr, $ctx:expr, $mapper:expr) => {
        if let Some(View::$variant($state)) = $self.views.last_mut() {
            $state.handle_message($msg, $ctx)?.map($mapper)
        } else {
            Effect::ok()
        }
    };
}

impl HandleMessage<views::error::Message> for App {
    fn handle_message(
        &mut self,
        message: views::error::Message,
        (): Self::Context<'_>,
    ) -> anyhow::Result<Effect<Self::Message, Self::OutMessage>> {
        handle_view!(self, Error, state, message, (), |message| match message {
            views::error::OutMessage::Close => {
                self.views.pop();
                Effect::ok()
            }
        })
    }
}

impl HandleMessage<views::player_details::Message> for App {
    fn handle_message(
        &mut self,
        message: views::player_details::Message,
        (): Self::Context<'_>,
    ) -> anyhow::Result<Effect<Self::Message, Self::OutMessage>> {
        handle_view!(
            self,
            PlayerDetails,
            state,
            message,
            &self.tournament,
            |message| match message {
                views::player_details::OutMessage::SaveAndClose(maybe_id, info) => {
                    let effect = self.handle_message(
                        match maybe_id {
                            Some(id) => tournament::Action::SetPlayerInfo(id, info),
                            None => tournament::Action::Register(info),
                        },
                        (),
                    )?;
                    self.views.pop();
                    Ok(effect)
                }
                views::player_details::OutMessage::Close => {
                    self.views.pop();
                    Effect::ok()
                }
            }
        )
    }
}
