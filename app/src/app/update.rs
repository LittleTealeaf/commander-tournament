use iced::Task;
use iced_tea::{HandleMessage, Model, Signal};

use crate::{
    App,
    app::Message,
    core::{file::FileAction, state::AppStateMsg, tournament::TournamentAction},
    home::{HomeMsg, HomeOut},
    services::system::open_link,
    views::{
        game_config::GameConfigView, matchmaker_config::MatchmakerConfigView, play::PlayView,
        player::PlayerView,
    },
};

use super::{MenuMsg, view::View};

impl Model for App {
    type Message = Message;
    type OutMessage = ();
    type Context<'a> = ();

    fn update<'a>(
        &'a mut self,
        message: Self::Message,
        (): Self::Context<'a>,
    ) -> anyhow::Result<Signal<Self::Message, Self::OutMessage>> {
        log::debug!("Processing Update: {message:?}");

        match message {
            Message::Refresh => self
                .views
                .last()
                .and_then(View::on_resume)
                .map_or(Signal::Message(Message::Home(HomeMsg::Refresh)), Signal::msg)
                .ok(),
            Message::Menu(msg) => self.menu.map_update(msg, &self.file, |out| {
                match out {
                    MenuMsg::New => Signal::msg(FileAction::RequestNew),
                    MenuMsg::Open => Signal::msg(FileAction::RequestOpen),
                    MenuMsg::Save => Signal::msg(FileAction::Save),
                    MenuMsg::SaveAs => Signal::msg(FileAction::SaveAs),
                    MenuMsg::OpenGameConfig => Signal::msg(Message::OpenGameConfig),
                }
                .ok()
            }),
            Message::CloseView => {
                self.views.pop();
                Signal::msg(Message::Refresh).ok()
            }
            Message::Nothing => Signal::done(),
            Message::AppState(message) => self.handle_message(message, ()),
            Message::AppStateLoaded(maybe_settings) => {
                self.state = maybe_settings;
                self.state
                    .as_ref()
                    .and_then(|state| state.last_opened().as_ref())
                    .map(|last_opened| Signal::msg(FileAction::OpenFile(last_opened.clone())))
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
                Signal::task(Task::future(async { open_link(link).await }).discard()).ok()
            }
            Message::View(msg) => self.views.last_mut().map_or(Signal::done(), |view| {
                view.map_update(msg, &self.tournament, |msg| Signal::msg(msg).ok())
            }),
            Message::OpenPlayView(play_mode) => {
                self.push_view(PlayView::new(play_mode, &self.tournament)).ok()
            }
            Message::QuitRequested => {
                if self.modified && !self.close_requested {
                    self.close_requested = true;
                    Signal::done()
                } else {
                    Signal::task(iced::exit()).ok()
                }
            }
            Message::QuitConfirm(close) => {
                self.close_requested = false;
                if close {
                    Signal::task(iced::exit()).ok()
                } else {
                    Signal::done()
                }
            }
            Message::OpenMatchmakerConfig => self
                .push_view(MatchmakerConfigView::new(
                    self.tournament.matchmaker_config().clone(),
                ))
                .ok(),
            Message::OpenGameConfig => self
                .push_view(GameConfigView::new(self.tournament.game_config().clone()))
                .ok(),
            Message::ClearError => {
                self.error = None;
                Signal::done()
            }
            Message::ClearOverwrite => {
                self.overwrite_requested = None;
                Signal::done()
            }
            Message::ConfirmOverwrite => {
                let mut overwrite = None;
                core::mem::swap(&mut overwrite, &mut self.overwrite_requested);
                overwrite.map_or(Ok(Signal::Done), |action| self.handle_message(action, ()))
            }
        }
    }
}

impl HandleMessage<HomeMsg> for App {
    fn handle_message<'a>(
        &'a mut self,
        message: HomeMsg,
        (): Self::Context<'a>,
    ) -> anyhow::Result<Signal<Self::Message, Self::OutMessage>> {
        self.home.map_update(message, &self.tournament, |out| match out {
            HomeOut::RecordGame(game_record) => Signal::msg(TournamentAction::Record(game_record)).ok(),
            HomeOut::OpenPlayerDetails(player_id) => {
                Signal::msg(Message::OpenPlayerDetails(Some(player_id))).ok()
            }
            HomeOut::OpenNewPlayer => Signal::msg(Message::OpenPlayerDetails(None)).ok(),
            HomeOut::OpenLink(link) => Signal::msg(Message::OpenLink(link)).ok(),
            HomeOut::OpenMatchmakerConfig => Signal::msg(Message::OpenMatchmakerConfig).ok(),
        })
    }
}

impl HandleMessage<AppStateMsg> for App {
    fn handle_message<'a>(
        &'a mut self,
        message: AppStateMsg,
        (): Self::Context<'a>,
    ) -> anyhow::Result<Signal<Self::Message, Self::OutMessage>> {
        if let Some(state) = &mut self.state {
            state.handle_message(message, ())?.map_empty()
        } else {
            Signal::done()
        }
    }
}
