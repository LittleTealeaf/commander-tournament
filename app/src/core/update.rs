use iced::Task;

use crate::{
    App,
    core::{
        file::FileAction,
        message::Message,
        settings::{AppSettings, AppSettingsMsg},
        tournament::TournamentAction,
        view::View,
    },
    error::ErrorMsg,
    home::{HomeMsg, HomeOut},
    player_details::{PlayerDetails, PlayerDetailsMsg, PlayerDetailsOut},
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
                    return Effect::done();
                };
                let message = settings
                    .last_opened()
                    .as_ref()
                    .map(|path| FileAction::OpenFile(path.clone()));

                self.settings = Some(settings);

                message.map_or_else(Effect::done, |message| self.handle_message(message, ()))
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
            Message::OpenPlayerDetails(maybe_id) => {
                self.push_view(PlayerDetails::new(
                    maybe_id.and_then(|id| self.tournament().get_registered_player(id)),
                ));
                Effect::done()
            }
        }
    }
}

impl HandleMessage<HomeMsg> for App {
    fn handle_message(
        &mut self,
        message: HomeMsg,
        (): Self::UpdateContext<'_>,
    ) -> anyhow::Result<Effect<Self::Message, Self::OutMessage>> {
        self.home
            .handle_message(message, (&self.tournament, &self.file))?
            .map(|message| match message {
                HomeOut::RegisterRecord(game_record) => {
                    self.handle_message(TournamentAction::Record(game_record), ())
                }
            })
    }
}

impl HandleMessage<AppSettingsMsg> for App {
    fn handle_message(
        &mut self,
        message: AppSettingsMsg,
        (): Self::UpdateContext<'_>,
    ) -> anyhow::Result<Effect<Self::Message, Self::OutMessage>> {
        if let Some(settings) = &mut self.settings {
            settings
                .handle_message(message, ())?
                .map(|error| self.handle_message(Message::Error(error), ()))
        } else {
            Effect::done()
        }
    }
}

impl HandleMessage<ErrorMsg> for App {
    fn handle_message(
        &mut self,
        message: ErrorMsg,
        (): Self::UpdateContext<'_>,
    ) -> anyhow::Result<Effect<Self::Message, Self::OutMessage>> {
        if let Some(View::Error(state)) = self.views.last_mut() {
            state
                .handle_message(message, ())?
                .map(|message| match message {
                    ErrorMsg::CloseError => {
                        self.views.pop();
                        Effect::done()
                    }
                })
        } else {
            Effect::done()
        }
    }
}

impl HandleMessage<PlayerDetailsMsg> for App {
    fn handle_message(
        &mut self,
        message: PlayerDetailsMsg,
        (): Self::UpdateContext<'_>,
    ) -> anyhow::Result<Effect<Self::Message, Self::OutMessage>> {
        if let Some(View::PlayerDetails(state)) = self.views.last_mut() {
            state
                .handle_message(message, ())?
                .map(|message| match message {
                    PlayerDetailsOut::OpenPlayer(id) => {
                        self.views.push(View::PlayerDetails(PlayerDetails::new(
                            self.tournament.get_registered_player(id),
                        )));
                        Effect::done()
                    }
                    PlayerDetailsOut::SaveAndClose(maybe_id, info) => {
                        let effect = self.handle_message(
                            match maybe_id {
                                Some(id) => TournamentAction::SetPlayerInfo(id, info),
                                None => TournamentAction::Register(info),
                            },
                            (),
                        )?;
                        self.views.pop();
                        Ok(effect)
                    }
                    PlayerDetailsOut::Close => {
                        self.views.pop();
                        Effect::done()
                    }
                    PlayerDetailsOut::DeletePlayer(id) => {
                        let effect = self.handle_message(TournamentAction::DeletePlayer(id), ())?;
                        self.views.pop();
                        Ok(effect)
                    }
                })
        } else {
            Effect::done()
        }
    }
}
