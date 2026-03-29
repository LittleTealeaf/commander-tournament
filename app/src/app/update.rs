use iced::Task;

use crate::{
    App,
    app::{Message, View},
    core::{
        file::FileAction,
        state::{AppState, AppStateMsg},
    },
    effect::Effect,
    error::ErrorMsg,
    home::HomeMsg,
    player_details::{PlayerDetails, PlayerDetailsMsg, PlayerDetailsOut},
    services::system::open_link,
    traits::{ComponentUpdate, HandleMessage},
};

impl ComponentUpdate for App {
    type UpdateContext<'a> = ();
    fn update(
        &mut self,
        message: Self::Message,
        (): Self::UpdateContext<'_>,
    ) -> anyhow::Result<Effect<Self::Message, Self::OutMessage>> {
        #[cfg(debug_assertions)]
        {
            let dbg = format!("{message:?}");
            if dbg.len() > 1000 {
                println!("Update: {dbg:.1000}...");
            } else {
                println!("Update: {dbg}");
            }
        }

        match message {
            Message::CloseView => {
                self.views.pop();
                Effect::done()
            }
            Message::Nothing => Effect::done(),
            Message::AppState(message) => self.handle_message(message, ()),
            Message::AppStateLoaded(maybe_settings) => {
                self.state = maybe_settings;

                let Some(settings) = &self.state else {
                    return Effect::done();
                };

                let Some(last_opened) = settings.last_opened() else {
                    return Effect::done();
                };

                let path = last_opened.clone();

                Effect::global(FileAction::OpenFile(path)).ok()
            }
            Message::OnBoot => Effect::Task(Task::perform(
                async { AppState::load().await.ok() },
                Message::AppStateLoaded,
            ))
            .ok(),
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
            Message::OpenLink(link) => Effect::Task(Task::future(async {
                if let Err(err) = open_link(link).await {
                    eprintln!("Warning: {err}");
                }
                Message::Nothing
            }))
            .ok(),
        }
    }
}

macro_rules! try_into {
    ($variant: ident, $type: ty) => {
        impl<'a> TryFrom<&'a mut crate::app::View> for &'a mut $type {
            type Error = ();

            fn try_from(value: &'a mut crate::app::View) -> Result<Self, Self::Error> {
                if let View::$variant(state) = value {
                    Ok(state)
                } else {
                    Err(())
                }
            }
        }
    };
}

try_into!(Error, crate::error::Error);
try_into!(PlayerDetails, PlayerDetails);

impl App {
    pub fn update_view<'a, F, V, E>(&'a mut self, f: F) -> anyhow::Result<Effect<Message, ()>>
    where
        F: FnOnce(&'a mut V) -> anyhow::Result<Effect<Message, ()>> + 'a,
        V: 'a,
        &'a mut View: TryInto<&'a mut V, Error = E>,
    {
        self.get_view_mut().map_or_else(Effect::done, |view| {
            view.try_into().map_or_else(|_| Effect::done(), f)
        })
    }

    fn handle_view_message<'a, C, F, E>(
        &'a mut self,
        message: C::Message,
        context: C::UpdateContext<'a>,
        map_out: F,
    ) -> anyhow::Result<Effect<Message, ()>>
    where
        &'a mut View: TryInto<&'a mut C, Error = E>,
        C: ComponentUpdate + 'a,
        F: FnMut(C::OutMessage) -> anyhow::Result<Effect<Message, ()>> + 'a,
        C::Message: Into<Message>,
    {
        if let Some(view) = self.get_view_mut()
            && let Ok(component) = view.try_into()
        {
            component.handle_message(message, context)?.map(map_out)
        } else {
            Effect::done()
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
            .map_empty()
    }
}

impl HandleMessage<AppStateMsg> for App {
    fn handle_message(
        &mut self,
        message: AppStateMsg,
        (): Self::UpdateContext<'_>,
    ) -> anyhow::Result<Effect<Self::Message, Self::OutMessage>> {
        if let Some(state) = &mut self.state {
            state.handle_message(message, ())?.map_empty()
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
        self.handle_view_message::<crate::error::Error, _, _>(
            message,
            (),
            |message| match message {
                ErrorMsg::CloseError => Effect::global(Message::CloseView).ok(),
            },
        )
    }
}

impl HandleMessage<PlayerDetailsMsg> for App {
    fn handle_message(
        &mut self,
        message: PlayerDetailsMsg,
        (): Self::UpdateContext<'_>,
    ) -> anyhow::Result<Effect<Self::Message, Self::OutMessage>> {
        self.handle_view_message::<PlayerDetails, _, _>(message, (), |message| match message {
            PlayerDetailsOut::Close => Effect::global(Message::CloseView).ok(),
        })
    }
}
