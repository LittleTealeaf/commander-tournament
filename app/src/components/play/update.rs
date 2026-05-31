use edh_tourn::{
    game::{POD_SIZE, record::GameRecord},
    player::PlayerId,
    tournament::Tournament,
};

use crate::{
    components::play::{PlayComponent, PlayMode},
    effect::Effect,
    traits::ComponentUpdate,
};

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

impl ComponentUpdate for PlayComponent {
    type UpdateContext<'a> = &'a Tournament;

    fn update(
        &mut self,
        message: Self::Message,
        context: Self::UpdateContext<'_>,
    ) -> anyhow::Result<crate::effect::Effect<Self::Message, Self::OutMessage>> {
        match message {
            PlayComponentMsg::SetPlayer(row, id) => match &mut self.mode {
                PlayMode::Player(player_id) => {
                    if row != 0 {
                        return Err(anyhow::anyhow!("Only the first player is editable"));
                    }
                    *player_id = id;
                    self.refresh(context);
                    Effect::done()
                }
                PlayMode::Custom(players) => {
                    let entry = players.get_mut(row).ok_or_else(|| {
                        anyhow::anyhow!("Invalid index. Got {row}, array length of {POD_SIZE}")
                    })?;
                    *entry = Some(id);
                    self.refresh(context);
                    Effect::done()
                }
                PlayMode::Next { .. } => Effect::done(),
            },
            PlayComponentMsg::Refresh => {
                self.refresh(context);
                Effect::done()
            }
            PlayComponentMsg::Submit => {
                if let Some(preview) = &self.preview
                    && let Some(winner) = preview.winner
                {
                    Effect::out(PlayComponentOut::RecordGame(Box::new(
                        preview.matchup.clone().record(winner)?,
                    )))
                    .chain(Effect::msg(PlayComponentMsg::Refresh))
                    .ok()
                } else {
                    Effect::done()
                }
            }
            PlayComponentMsg::SetWinner(player_id) => {
                if let Some(preview) = &mut self.preview {
                    preview.winner = Some(player_id);
                }
                Effect::done()
            }
            PlayComponentMsg::ClickPlayer(player_id) => {
                Effect::out(PlayComponentOut::OpenPlayer(player_id)).ok()
            }
            PlayComponentMsg::OpenLink(url) => Effect::out(PlayComponentOut::OpenLink(url)).ok(),
            PlayComponentMsg::OpenMatchLinks => {
                let Some(preview) = &self.preview else {
                    return Effect::done();
                };

                Effect::sequence(
                    context
                        .get_registered_players(preview.matchup.ids())
                        .filter_map(|player| player.info().moxfield_goldfish_link())
                        .map(|link| Effect::out(PlayComponentOut::OpenLink(link))),
                )
                .ok()
            }
        }
    }
}
