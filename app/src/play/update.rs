use core::iter::once;
use std::collections::HashSet;

use edh_tourn::{
    game::{POD_SIZE, matchup::Matchup},
    player::PlayerId,
    tournament::Tournament,
};
use itertools::Itertools;

use crate::{
    effect::Effect,
    play::{
        PlayMode, PlayMsg, PlayNextMode, PlayOut, PlayView,
        match_preview::{MatchPreview, MatchPreviewOut},
    },
    traits::ComponentUpdate,
};

fn get_longest_break(tournament: &Tournament) -> Option<PlayerId> {
    let mut players = tournament.players().keys().collect::<HashSet<_>>();
    for &id in &players {
        if tournament.get_player_stats(*id).is_none() {
            return Some(*id);
        }
    }

    for game in tournament.games().iter().rev() {
        for player in game.players() {
            if players.len() == 1 {
                return Some(*players.into_iter().next()?);
            }

            players.remove(&player.id());
        }
    }
    Some(*players.into_iter().next()?)
}

impl PlayMode {
    pub(super) fn create_matchup(&self, tournament: &Tournament) -> Option<Matchup> {
        match self {
            Self::Player { id, ranking } => {
                let rankings = tournament.ranking().ranked(*id, *ranking).ok()?;
                let opponents = rankings.into_iter().take(3).map(|p| p.id());
                let players = once(*id).chain(opponents).collect_array()?;
                tournament.create_match(players).ok()
            }
            Self::Next { ranking, mode } => {
                let id = match mode {
                    PlayNextMode::LeastGames => tournament
                        .get_registered_players()
                        .sorted_by_key(|player| player.stats().games())
                        .next()?
                        .id(),
                    PlayNextMode::LongestBreak => get_longest_break(tournament)?,
                };
                let rankings = tournament.ranking().ranked(id, *ranking).ok()?;
                let opponents = rankings.into_iter().take(3).map(|p| p.id());
                let players = once(id).chain(opponents).collect_array()?;
                tournament.create_match(players).ok()
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
            PlayMsg::SetRankingMethod(ranking_method) => match &mut self.mode {
                PlayMode::Player { ranking, .. } | PlayMode::Next { ranking, .. } => {
                    *ranking = ranking_method;
                    modified = true;
                    Effect::Done
                }
                PlayMode::Custom { .. } => Effect::Done,
            },
            PlayMsg::SetNextMode(play_next_mode) => {
                let PlayMode::Next { mode, .. } = &mut self.mode else {
                    return Effect::done();
                };
                *mode = play_next_mode;
                modified = true;
                Effect::Done
            }
            PlayMsg::OpenLink(link) => Effect::out(PlayOut::OpenLink(link)),
            PlayMsg::OpenLinks(links) => Effect::sequence(
                links
                    .into_iter()
                    .map(|link| Effect::out(PlayOut::OpenLink(link))),
            ),
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
        };

        if modified {
            self.match_preview = self.mode.create_matchup(context).map(MatchPreview::new);
        }

        effect.ok()
    }
}
