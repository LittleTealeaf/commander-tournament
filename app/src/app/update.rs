use iced::Task;

use crate::{
    App,
    app::{Message, ViewUpdateContext},
    core::{
        file::FileAction,
        state::{AppState, AppStateMsg},
    },
    effect::Effect,
    home::HomeMsg,
    player_details::PlayerDetails,
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
            Message::Home(message) => self.handle_message(message, ()),
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
            Message::View(msg) => {
                if let Some(view) = self.views.last_mut() {
                    view.mapped_update(msg, ViewUpdateContext::new(&self.tournament), |msg| {
                        Effect::Msg(msg).ok()
                    })
                } else {
                    Effect::done()
                }
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
