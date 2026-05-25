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

use super::{MenuMsg, view::View};

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
            Message::Refresh => self
                .views
                .last()
                .and_then(View::on_resume)
                .map_or(Effect::Msg(Message::Home(HomeMsg::Refresh)), Effect::msg)
                .ok(),
            Message::Menu(msg) => self.menu.map_update(msg, (), |out| {
                match out {
                    MenuMsg::New => Effect::msg(FileAction::RequestNew),
                    MenuMsg::Open => Effect::msg(FileAction::RequestOpen),
                    MenuMsg::Save => Effect::msg(FileAction::Save),
                    MenuMsg::SaveAs => Effect::msg(FileAction::SaveAs),
                    MenuMsg::OpenGameConfig => Effect::msg(Message::OpenGameConfig),
                }
                .ok()
            }),
            Message::CloseView => {
                self.views.pop();
                Effect::msg(Message::Refresh).ok()
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
            Message::OpenPlayView(play_mode) => self.push_view(PlayView::new(play_mode, &self.tournament)).ok(),
            Message::QuitRequested => {
                if self.modified && !self.close_requested {
                    self.close_requested = true;
                    Effect::done()
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
            Message::ClearError => {
                self.error = None;
                Effect::done()
            }
            Message::ClearOverwrite => {
                self.overwrite_requested = None;
                Effect::done()
            }
            Message::ConfirmOverwrite => {
                let mut overwrite = None;
                core::mem::swap(&mut overwrite, &mut self.overwrite_requested);
                overwrite.map_or(Ok(Effect::Done), |action| self.handle_message(action, ()))
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
        self.home.map_update(message, &self.tournament, |out| match out {
            HomeOut::RecordGame(game_record) => Effect::msg(TournamentAction::Record(game_record)).ok(),
            HomeOut::OpenPlayerDetails(player_id) => {
                Effect::msg(Message::OpenPlayerDetails(Some(player_id))).ok()
            }
            HomeOut::OpenNewPlayer => Effect::msg(Message::OpenPlayerDetails(None)).ok(),
            HomeOut::OpenLink(link) => Effect::msg(Message::OpenLink(link)).ok(),
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
