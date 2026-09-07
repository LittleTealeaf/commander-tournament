use edh_tourn::tournament::Tournament;
use iced::Element;
use iced::widget::{button, column, text};
use iced_tea::{Component, Model, Signal};

use crate::{
    App,
    app::message::Message,
    components::play::PlayMode,
    core::tournament::TournamentAction,
    popup::Popup,
    views::{
        ViewScreen,
        game_config::{GameConfigMsg, GameConfigOut, GameConfigView},
        matchmaker_config::{MatchmakerConfigMsg, MatchmakerConfigOut, MatchmakerConfigView},
        play::{PlayView, PlayViewMsg, PlayViewOut},
        player::{PlayerDetailsMsg, PlayerDetailsOut, PlayerView},
    },
};

#[derive(Clone, Debug, derive_more::From)]
pub enum View {
    Play(PlayView),
    PlayConfig(MatchmakerConfigView),
    GameConfig(GameConfigView),
    PlayerDetails(PlayerView),
}

impl View {
    pub fn on_resume(&self) -> Option<ViewMsg> {
        match self {
            Self::Play(_) => PlayView::ON_RESUME.map(Into::into),
            Self::PlayConfig(_) => MatchmakerConfigView::ON_RESUME.map(Into::into),
            Self::PlayerDetails(_) => PlayerView::ON_RESUME.map(Into::into),
            Self::GameConfig(_) => GameConfigView::ON_RESUME.map(Into::into),
        }
    }
}

#[derive(Debug, Clone, derive_more::From)]
pub enum ViewMsg {
    PlayerDetails(PlayerDetailsMsg),
    PlayConfig(MatchmakerConfigMsg),
    GameConfig(GameConfigMsg),
    Play(PlayViewMsg),
}

impl Model for View {
    type OutMessage = Message;
    type Message = ViewMsg;
    type Context<'a> = &'a Tournament;

    fn update<'a>(
        &'a mut self,
        message: Self::Message,
        context: Self::Context<'a>,
    ) -> anyhow::Result<Signal<Self::Message, Self::OutMessage>> {
        const CLOSE_VIEW: Signal<ViewMsg, Message> = Signal::Out(Message::CloseView);

        match (self, message) {
            (Self::PlayerDetails(state), ViewMsg::PlayerDetails(msg)) => {
                state.map_update(msg, context, |out| match out {
                    PlayerDetailsOut::OpenPlayerDetails(player_id) => {
                        Signal::Out(Message::OpenPlayerDetails(Some(player_id))).ok()
                    }
                    PlayerDetailsOut::DeletePlayer(player_id) => {
                        Signal::out(TournamentAction::DeletePlayer(player_id))
                            .chain(CLOSE_VIEW)
                            .ok()
                    }
                    PlayerDetailsOut::OpenLink(link) => Signal::Out(Message::OpenLink(link)).ok(),
                    PlayerDetailsOut::SaveAndClose(player_id, player_info) => Signal::out(match player_id {
                        Some(id) => TournamentAction::SetPlayerInfo(id, player_info),
                        None => TournamentAction::Register(player_info),
                    })
                    .chain(CLOSE_VIEW)
                    .ok(),
                    PlayerDetailsOut::OpenPlayerMatches(player_id) => {
                        Signal::out(Message::OpenPlayView(PlayMode::Player(player_id))).ok()
                    }
                    PlayerDetailsOut::Close => CLOSE_VIEW.ok(),
                })
            }
            (Self::Play(state), ViewMsg::Play(msg)) => state.map_update(msg, context, |out| match out {
                PlayViewOut::Close => CLOSE_VIEW.ok(),
                PlayViewOut::OpenPlayer(player_id) => {
                    Signal::out(Message::OpenPlayerDetails(Some(player_id))).ok()
                }
                PlayViewOut::OpenLink(link) => Signal::out(Message::OpenLink(link)).ok(),
                PlayViewOut::RecordGame(game_record) => {
                    Signal::out(TournamentAction::Record(game_record)).ok()
                }
                PlayViewOut::OpenMatchmakerConfig => Signal::out(Message::OpenMatchmakerConfig).ok(),
            }),
            (Self::PlayConfig(state), ViewMsg::PlayConfig(msg)) => {
                state.map_update(msg, context, |out| match out {
                    MatchmakerConfigOut::Close => CLOSE_VIEW.ok(),
                    MatchmakerConfigOut::SaveAndClose(ranking_config) => {
                        Signal::out(TournamentAction::SetMatchmakerConfig(ranking_config))
                            .chain(CLOSE_VIEW)
                            .ok()
                    }
                })
            }
            (Self::GameConfig(state), ViewMsg::GameConfig(msg)) => {
                state.map_update(msg, context, |out| match out {
                    GameConfigOut::Close => CLOSE_VIEW.ok(),
                    GameConfigOut::SaveAndClose(game_config) => {
                        Signal::out(TournamentAction::SetGameConfig(game_config))
                            .chain(CLOSE_VIEW)
                            .ok()
                    }
                })
            }
            (_, message) => {
                eprintln!("Received Message {message:?} when view did not expect it.");
                Signal::done()
            }
        }
    }
}

impl Component for View {
    fn render<'a>(&'a self, context: Self::Context<'a>) -> Element<'a, Self::Message> {
        match self {
            Self::PlayConfig(settings) => settings.screen_view_into(context),
            Self::PlayerDetails(player_details) => player_details.screen_view_into(context),
            Self::Play(play) => play.screen_view_into(context),
            Self::GameConfig(game_config) => game_config.screen_view_into(context),
        }
    }
}

impl App {
    #[must_use]
    pub fn handle_view(&self) -> Element<'_, Message> {
        let content = self.views.last().map_or_else(
            || {
                column![
                    self.menu.render_into(&self.file),
                    self.home.render_into(&self.tournament)
                ]
                .spacing(5)
                .into()
            },
            |view| view.render_into(&self.tournament),
        );

        if self.close_requested {
            return Popup::new(
                "Close Application?",
                text("You have unsaved changes").into(),
                vec![
                    button("Cancel").on_press(Message::QuitConfirm(false)).into(),
                    button("Close").on_press(Message::QuitConfirm(true)).into(),
                ],
            )
            .overlay(content);
        }

        if let Some(error) = &self.error {
            return Popup::new(
                "Application Error",
                text(error).into(),
                vec![button("Close").on_press(Message::ClearError).into()],
            )
            .overlay(content);
        }

        if self.overwrite_requested.is_some() {
            return Popup::new(
                "Overwrite Tournament?",
                text("All saved changes will be lost").into(),
                vec![
                    button("Cancel").on_press(Message::ClearOverwrite).into(),
                    button("Confirm").on_press(Message::ConfirmOverwrite).into(),
                ],
            )
            .overlay(content);
        }

        content
    }

    #[must_use]
    pub fn get_view(&self) -> Option<&View> {
        self.views.last()
    }

    pub fn push_view<V>(&mut self, view: V) -> Signal<Message, ()>
    where
        V: Into<View>,
    {
        let view: View = view.into();
        let on_resume = view.on_resume();
        self.views.push(view);
        on_resume.map(Signal::msg).unwrap_or_default()
    }
}
