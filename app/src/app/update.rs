use iced::Task;

use crate::{
    App,
    app::{Message, ViewUpdateContext},
    components::confirm::ConfirmDialog,
    core::{
        file::FileAction,
        state::{AppState, AppStateMsg},
        tournament::TournamentAction,
    },
    effect::Effect,
    home::{HomeMsg, HomeOut},
    play::PlayView,
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
            Message::OpenConfirm(dialog) => {
                self.push_view(*dialog);
                Effect::done()
            }
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

                Effect::msg(FileAction::OpenFile(path)).ok()
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
            Message::OpenPlay(play_mode) => {
                self.push_view(PlayView::new(play_mode, &self.tournament));
                Effect::done()
            }
            Message::QuitRequested => {
                if self.modified && !self.close_requested {
                    self.close_requested = true;
                    Effect::msg(Message::OpenConfirm(Box::new(ConfirmDialog::new(
                        "Unsaved Changes".to_owned(),
                        "You have unsaved changes. Are you sure you want to exit without saving?"
                            .to_owned(),
                        Message::QuitConfirmed,
                        Some(Message::QuitCancelled),
                    ))))
                    .ok()
                } else {
                    Effect::Task(iced::exit()).ok()
                }
            }
            Message::QuitConfirmed => {
                self.close_requested = false;
                Effect::Task(iced::exit()).ok()
            }
            Message::QuitCancelled => {
                self.close_requested = false;
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
            .map(|out| match out {
                HomeOut::RecordGame(game_record) => {
                    Effect::msg(TournamentAction::Record(game_record)).ok()
                }
                HomeOut::OpenLink(link) => Effect::msg(Message::OpenLink(link)).ok(),
                HomeOut::FileNew => Effect::msg(FileAction::New).ok(),
                HomeOut::FileOpen => Effect::msg(FileAction::Open).ok(),
                HomeOut::FileSave => Effect::msg(FileAction::Save).ok(),
                HomeOut::FileSaveAs => Effect::msg(FileAction::SaveAs).ok(),
                HomeOut::OpenPlayerDetails(player_id) => {
                    Effect::msg(Message::OpenPlayerDetails(Some(player_id))).ok()
                }
                HomeOut::OpenNewPlayer => Effect::msg(Message::OpenPlayerDetails(None)).ok(),
                HomeOut::OpenPlayView(mode) => Effect::msg(Message::OpenPlay(mode)).ok(),
            })
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
