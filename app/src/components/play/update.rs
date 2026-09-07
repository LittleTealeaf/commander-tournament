use edh_tourn::{
    game::{POD_SIZE, record::GameRecord},
    player::PlayerId,
    tournament::Tournament,
};

use iced_tea::{Model, Signal};

use crate::components::play::{PlayComponent, PlayMode};

#[derive(Debug, Clone)]
pub enum PlayComponentMsg {
    Submit,
    Refresh,
    OpenLink(String),
    OpenMatchLinks,
    SetWinner(PlayerId),
    ClickPlayer(PlayerId),
    SetPlayer(usize, PlayerId),
}

#[derive(Debug)]
pub enum PlayComponentOut {
    OpenPlayer(PlayerId),
    OpenLink(String),
    RecordGame(Box<GameRecord>),
}

impl Model for PlayComponent {
    type Message = PlayComponentMsg;
    type OutMessage = PlayComponentOut;
    type Context<'a> = &'a Tournament;

    fn update<'a>(
        &'a mut self,
        message: Self::Message,
        context: Self::Context<'a>,
    ) -> anyhow::Result<Signal<Self::Message, Self::OutMessage>> {
        match message {
            PlayComponentMsg::SetPlayer(row, id) => match &mut self.mode {
                PlayMode::Player(player_id) => {
                    if row != 0 {
                        return Err(anyhow::anyhow!("Only the first player is editable"));
                    }
                    *player_id = id;
                    self.refresh(context);
                    Signal::done()
                }
                PlayMode::Custom(players) => {
                    let entry = players.get_mut(row).ok_or_else(|| {
                        anyhow::anyhow!("Invalid index. Got {row}, array length of {POD_SIZE}")
                    })?;
                    *entry = Some(id);
                    self.refresh(context);
                    Signal::done()
                }
                PlayMode::Next { .. } => Signal::done(),
            },
            PlayComponentMsg::Refresh => {
                self.refresh(context);
                Signal::done()
            }
            PlayComponentMsg::Submit => {
                if let Some(preview) = &self.preview
                    && let Some(winner) = preview.winner
                {
                    Signal::out(PlayComponentOut::RecordGame(Box::new(
                        preview.matchup.clone().record(winner)?,
                    )))
                    .chain(Signal::msg(PlayComponentMsg::Refresh))
                    .ok()
                } else {
                    Signal::done()
                }
            }
            PlayComponentMsg::SetWinner(player_id) => {
                if let Some(preview) = &mut self.preview {
                    preview.winner = Some(player_id);
                }
                Signal::done()
            }
            PlayComponentMsg::ClickPlayer(player_id) => {
                Signal::out(PlayComponentOut::OpenPlayer(player_id)).ok()
            }
            PlayComponentMsg::OpenLink(url) => Signal::out(PlayComponentOut::OpenLink(url)).ok(),
            PlayComponentMsg::OpenMatchLinks => {
                let Some(preview) = &self.preview else {
                    return Signal::done();
                };

                Signal::sequence(
                    context
                        .get_registered_players(preview.matchup.ids())
                        .filter_map(|player| player.info().moxfield_goldfish_link())
                        .map(|link| Signal::out(PlayComponentOut::OpenLink(link))),
                )
                .ok()
            }
        }
    }
}
