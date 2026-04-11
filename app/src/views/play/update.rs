use core::cmp::Ordering;
use std::collections::HashSet;

use edh_tourn::{
    game::{POD_SIZE, matchup::Matchup},
    player::PlayerId,
    tournament::Tournament,
};

use crate::{
    effect::Effect,
    traits::ComponentUpdate,
    views::play::{
        PlayMode, PlayMsg, PlayNextMode, PlayOut, PlayView,
        match_preview::{MatchPreview, MatchPreviewOut},
    },
};

fn get_longest_break<I>(tournament: &Tournament, players: I) -> Option<PlayerId>
where
    I: IntoIterator<Item = PlayerId>,
{
    let mut players = players.into_iter().collect::<HashSet<_>>();

    for player in &players {
        if tournament.get_player_or_default_stats(*player).games() == 0 {
            return Some(*player);
        }
    }

    for game in tournament.games().iter().rev() {
        for player in game.players() {
            if players.len() == 1 {
                return players.into_iter().next();
            }

            players.remove(&player.id());
        }
    }

    players.into_iter().next()
}

impl PlayMode {
    pub(super) fn create_matchup(&self, tournament: &Tournament) -> Option<Matchup> {
        match self {
            Self::Player(id) => tournament.matchmaker().create_match(*id).ok(),
            Self::Next {
                mode,
                ignore_precons,
            } => {
                let players = tournament
                    .get_registered_players()
                    .filter(|player| !*ignore_precons || !player.info().is_precon());
                let id = match mode {
                    PlayNextMode::LeastGames => players
                        .min_by_key(|player| (player.stats().games(), player.id()))?
                        .id(),
                    PlayNextMode::LongestBreak => {
                        get_longest_break(tournament, players.map(|player| player.id()))?
                    }
                    PlayNextMode::LeastWins => players
                        .min_by_key(|player| (player.stats().wins(), player.id()))?
                        .id(),
                    PlayNextMode::LowestWinrate => players
                        .min_by(|a, b| {
                            a.stats()
                                .wr_unwrap()
                                .partial_cmp(&b.stats().wr_unwrap())
                                .unwrap_or(Ordering::Equal)
                                .then_with(|| a.id().cmp(&b.id()))
                        })?
                        .id(),
                    PlayNextMode::HighestWinrate => players
                        .max_by(|a, b| {
                            a.stats()
                                .wr_unwrap()
                                .partial_cmp(&b.stats().wr_unwrap())
                                .unwrap_or(Ordering::Equal)
                                .then_with(|| a.id().cmp(&b.id()))
                        })?
                        .id(),
                    PlayNextMode::OutlierWinrate => players
                        .max_by(|a, b| {
                            let a_wr = a.stats().wr_unwrap();
                            let b_wr = b.stats().wr_unwrap();
                            #[allow(clippy::cast_precision_loss)]
                            let target = 1.0 / (POD_SIZE as f64);
                            let a_diff = (a_wr - target).abs();
                            let b_diff = (b_wr - target).abs();
                            a_diff.total_cmp(&b_diff).then_with(|| a.id().cmp(&b.id()))
                        })?
                        .id(),
                };
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
