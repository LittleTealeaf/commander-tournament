use edh_tourn::{
    game::{POD_SIZE, matchup::Matchup},
    tournament::Tournament,
};

use crate::{
    effect::Effect,
    traits::ComponentUpdate,
    views::play::{
        PlayMode, PlayMsg, PlayOut, PlayView,
        match_preview::{MatchPreview, MatchPreviewOut},
    },
};

impl PlayMode {
    pub(super) fn create_matchup(&self, tournament: &Tournament) -> Option<Matchup> {
        match self {
            Self::Player(id) => tournament.matchmaker().create_match(*id).ok(),
            Self::Next { mode, ignore_precons } => {
                let players = tournament
                    .registered_players()
                    .filter(|player| !*ignore_precons || !player.info().is_precon());
                let id = mode.get_player_from_list(tournament, players)?.id();
                tournament.matchmaker().create_match(id).ok()
            }
            Self::Custom { players } => {
                let [a, b, c, d] = *players;
                tournament.create_match([a?, b?, c?, d?]).ok()
            }
        }
    }
}

impl ComponentUpdate for PlayView {
    type UpdateContext<'a> = &'a Tournament;

    fn update(
        &mut self,
        message: Self::Message,
        context: Self::UpdateContext<'_>,
    ) -> anyhow::Result<crate::effect::Effect<Self::Message, Self::OutMessage>> {
        let mut modified = false;

        let effect = match message {
            PlayMsg::RefreshMatchup => {
                modified = true;
                Effect::Done
            }
            PlayMsg::SetNextMode(play_next_mode) => {
                let PlayMode::Next { mode, .. } = &mut self.mode else {
                    return Effect::done();
                };
                *mode = play_next_mode;
                modified = true;
                Effect::Done
            }
            PlayMsg::OpenLink(link) => Effect::out(PlayOut::OpenLink(link)),
            PlayMsg::SetPlayer(index, id) => {
                let PlayMode::Custom { players } = &mut self.mode else {
                    return Effect::done();
                };

                let value = players.get_mut(index).ok_or_else(|| {
                    anyhow::anyhow!(
                        "Player Index {index} invalid: Must be between 0 and {}",
                        POD_SIZE - 1
                    )
                })?;
                *value = id;
                modified = true;
                Effect::Done
            }
            PlayMsg::Preview(msg) => {
                let Some(preview) = self.match_preview.as_mut() else {
                    return Effect::done();
                };
                preview.update(msg, context)?.map(|out| match out {
                    MatchPreviewOut::RecordGame(game) => Effect::out(PlayOut::RecordGame(game))
                        .chain(Effect::msg(PlayMsg::RefreshMatchup))
                        .ok(),
                    MatchPreviewOut::OpenLink(link) => Effect::out(PlayOut::OpenLink(link)).ok(),
                    MatchPreviewOut::OpenPlayerInfo(player_id) => {
                        Effect::out(PlayOut::OpenPlayerInfo(player_id)).ok()
                    }
                })?
            }
            PlayMsg::Close => Effect::out(PlayOut::Close),
            PlayMsg::IgnorePrecons(value) => {
                let PlayMode::Next { ignore_precons, .. } = &mut self.mode else {
                    return Effect::done();
                };

                *ignore_precons = value;
                modified = true;
                Effect::Done
            }
            PlayMsg::OpenConfig => Effect::out(PlayOut::OpenRankingConfig),
        };

        if modified {
            self.match_preview = self.mode.create_matchup(context).map(MatchPreview::new);
        }

        effect.ok()
    }
}
