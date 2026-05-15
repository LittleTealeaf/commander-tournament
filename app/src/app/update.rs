use iced::Task;

use crate::{
    App,
    app::{Message, ViewUpdateContext},
    core::{file::FileAction, state::AppStateMsg, tournament::TournamentAction},
    effect::Effect,
    home::{HomeMsg, HomeOut},
    services::system::open_link,
    traits::{ComponentUpdate, HandleMessage},
    views::{
        game_config::GameConfigView, matchmaker_config::MatchmakerConfigView, play::PlayView,
        player::PlayerView,
    },
};

use super::view::View;

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

                self.views
                    .last()
                    .and_then(View::on_resume)
                    .map(Effect::msg)
                    .unwrap_or_default()
                    .ok()
            }
            Message::Nothing => Effect::done(),
            Message::AppState(message) => self.handle_message(message, ()),
            Message::AppStateLoaded(maybe_settings) => {
                self.state = maybe_settings;
                self.state
                    .as_ref()
                    .and_then(|state| state.last_opened().as_ref())
                    .map(|last_opened| Effect::msg(FileAction::OpenFile(last_opened.clone())))
                    .unwrap_or_default()
                    .ok()
            }
            Message::Tournament(action) => self.handle_message(action, ()),
            Message::Home(message) => self.handle_message(message, ()),
            Message::TournFile(message) => self.handle_message(message, ()),
            Message::Error(error) => Err(anyhow::anyhow!("{error}")),
            Message::OpenPlayerDetails(maybe_id) => self
                .push_view(PlayerView::new(
                    maybe_id.and_then(|id| self.tournament().get_registered_player(id)),
                ))
                .ok(),
            Message::OpenLink(link) => {
                Effect::Task(Task::future(async { open_link(link).await }).discard()).ok()
            }
            Message::View(msg) => self.views.last_mut().map_or(Effect::done(), |view| {
                view.map_update(msg, ViewUpdateContext::new(&self.tournament), |msg| {
                    Effect::msg(msg).ok()
                })
            }),
            Message::OpenPlay(play_mode) => self.push_view(PlayView::new(play_mode, &self.tournament)).ok(),
            Message::CloseModal => {
                self.modals.pop();
                Effect::done()
            }
            Message::Modal(modal_msg) => {
                if let Some(modal) = self.modals.last_mut() {
                    modal
                        .update(modal_msg, &self.tournament)?
                        .map(|out| Effect::msg(out).ok())
                } else {
                    Effect::done()
                }
            }
            Message::QuitRequested => {
                if self.modified && !self.close_requested {
                    self.close_requested = true;
                    Effect::confirm(
                        "Unsaved Changes".to_owned(),
                        "You have unsaved changes. Are you sure you want to exit without saving?".to_owned(),
                        Message::QuitConfirm(true),
                        Some(Message::QuitConfirm(false)),
                    )
                    .ok()
                } else {
                    Effect::Task(iced::exit()).ok()
                }
            }
            Message::QuitConfirm(close) => {
                self.close_requested = false;
                if close {
                    Effect::Task(iced::exit()).ok()
                } else {
                    Effect::done()
                }
            }
            Message::OpenPlayConfig => self
                .push_view(MatchmakerConfigView::new(
                    self.tournament.matchmaker_config().clone(),
                ))
                .ok(),
            Message::OpenGameConfig => self
                .push_view(GameConfigView::new(self.tournament.game_config().clone()))
                .ok(),
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
            .map_update(message, (&self.tournament, &self.file), |out| match out {
                HomeOut::RecordGame(game_record) => Effect::msg(TournamentAction::Record(game_record)).ok(),
                HomeOut::OpenLink(link) => Effect::msg(Message::OpenLink(link)).ok(),
                HomeOut::FileNew => Effect::msg(FileAction::RequestNew).ok(),
                HomeOut::FileOpen => Effect::msg(FileAction::RequestOpen).ok(),
                HomeOut::FileSave => Effect::msg(FileAction::Save).ok(),
                HomeOut::FileSaveAs => Effect::msg(FileAction::SaveAs).ok(),
                HomeOut::OpenPlayerDetails(player_id) => {
                    Effect::msg(Message::OpenPlayerDetails(Some(player_id))).ok()
                }
                HomeOut::OpenNewPlayer => Effect::msg(Message::OpenPlayerDetails(None)).ok(),
                HomeOut::OpenPlayView(mode) => Effect::msg(Message::OpenPlay(mode)).ok(),
                HomeOut::OpenGameConfig => Effect::msg(Message::OpenGameConfig).ok(),
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
