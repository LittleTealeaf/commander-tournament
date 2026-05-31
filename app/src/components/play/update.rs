use edh_tourn::{
    game::{POD_SIZE, matchup::Matchup, record::GameRecord},
    player::PlayerId,
    tournament::Tournament,
};

use crate::{
    components::play::{MatchPreview, PlayComponent, PlayMode, PlayNextMode},
    effect::Effect,
    traits::ComponentUpdate,
};

#[derive(Debug, Clone)]
pub enum PlayComponentMsg {
    Submit,
    Refresh,
    OpenLink(String),
    OpenMatchLinks,
    SetMode(PlayMode),
    SetNextMode(PlayNextMode),
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
                    *player_id = Some(id);
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
            PlayComponentMsg::SetMode(new_mode) => {
                self.mode = new_mode;
                self.refresh(context);
                Effect::done()
            }
            PlayComponentMsg::SetNextMode(new_mode) => {
                let PlayMode::Next(mode) = &mut self.mode else {
                    return Effect::done();
                };
                *mode = new_mode;
                self.refresh(context);
                Effect::done()
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

impl PlayComponent {
    pub(super) fn refresh(&mut self, tournament: &Tournament) {
        self.preview = self.mode.create_matchup(tournament).map(|matchup| MatchPreview {
            matchup,
            winner: None,
        });
    }
}

impl PlayMode {
    pub(super) fn create_matchup(&self, tournament: &Tournament) -> Option<Matchup> {
        match self {
            Self::Player(player_id) => player_id.and_then(|id| tournament.matchmaker().create_match(id).ok()),
            Self::Custom(players) => {
                let [a, b, c, d] = *players;
                tournament.create_match([a?, b?, c?, d?]).ok()
            }
            Self::Next(mode) => {
                let id = mode.get_player(tournament)?.id();
                tournament.matchmaker().create_match(id).ok()
            }
        }
    }
}
