use edh_tourn::tournament::Tournament;
use iced::Task;

use crate::{
    App,
    core::{
        file::TournamentFileMessage, message::Message, settings::AppSettings, tournament,
        view::View,
    },
    error, home, player_details,
    traits::{ComponentUpdate, Effect, HandleMessage},
};

impl ComponentUpdate for App {
    type UpdateContext<'a> = ();
    fn update(
        &mut self,
        message: Self::Message,
        (): Self::UpdateContext<'_>,
    ) -> anyhow::Result<crate::traits::Effect<Self::Message, Self::OutMessage>> {
        #[cfg(debug_assertions)]
        println!("Update: {message:?}");

        match message {
            Message::Settings(message) => self.handle_message(message, ()),
            Message::SettingsLoaded(maybe_settings) => {
                let Some(settings) = maybe_settings else {
                    return Effect::ok();
                };
                let message = settings
                    .last_opened()
                    .as_ref()
                    .map(|path| TournamentFileMessage::LoadTournament(path.clone()));

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
        (): Self::UpdateContext<'_>,
    ) -> anyhow::Result<Effect<Self::Message, Self::OutMessage>> {
        self.home
            .handle_message(message, (&self.tournament, &self.file))?
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
                home::OutMessage::MenuMessage(message) => match message {
                    home::menu::Message::New => {
                        self.tournament = Tournament::new();
                        Effect::ok()
                    }
                    home::menu::Message::Open => {
                        self.handle_message(TournamentFileMessage::Open, ())
                    }
                    home::menu::Message::Save => {
                        self.handle_message(TournamentFileMessage::Save, ())
                    }
                    home::menu::Message::SaveAs => {
                        self.handle_message(TournamentFileMessage::SaveAs, ())
                    }
                },
            })
    }
}

impl HandleMessage<crate::core::settings::Message> for App {
    fn handle_message(
        &mut self,
        message: crate::core::settings::Message,
        (): Self::UpdateContext<'_>,
    ) -> anyhow::Result<Effect<Self::Message, Self::OutMessage>> {
        if let Some(settings) = &mut self.settings {
            settings
                .handle_message(message, ())?
                .map(|error| self.handle_message(Message::Error(error), ()))
        } else {
            Effect::ok()
        }
    }
}

impl HandleMessage<error::Message> for App {
    fn handle_message(
        &mut self,
        message: error::Message,
        (): Self::UpdateContext<'_>,
    ) -> anyhow::Result<Effect<Self::Message, Self::OutMessage>> {
        if let Some(View::Error(state)) = self.views.last_mut() {
            state
                .handle_message(message, ())?
                .map(|message| match message {
                    error::Message::CloseError => {
                        self.views.pop();
                        Effect::ok()
                    }
                })
        } else {
            Effect::ok()
        }
    }
}

impl HandleMessage<player_details::Message> for App {
    fn handle_message(
        &mut self,
        message: player_details::Message,
        (): Self::UpdateContext<'_>,
    ) -> anyhow::Result<Effect<Self::Message, Self::OutMessage>> {
        if let Some(View::PlayerDetails(state)) = self.views.last_mut() {
            state
                .handle_message(message, ())?
                .map(|message| match message {
                    player_details::OutMessage::OpenPlayer(id) => {
                        self.views
                            .push(View::PlayerDetails(player_details::State::new(
                                self.tournament.get_registered_player(id),
                            )));
                        Effect::ok()
                    }
                    player_details::OutMessage::SaveAndClose(maybe_id, info) => {
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
                    player_details::OutMessage::Close => {
                        self.views.pop();
                        Effect::ok()
                    }
                    player_details::OutMessage::DeletePlayer(id) => {
                        let effect =
                            self.handle_message(tournament::Action::DeletePlayer(id), ())?;
                        self.views.pop();
                        Ok(effect)
                    }
                })
        } else {
            Effect::ok()
        }
    }
}
