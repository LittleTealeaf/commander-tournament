use std::collections::HashMap;

use itertools::Itertools;

use crate::{
    analytics::winloss::MatchPerformance,
    error::TournamentError,
    game::record::GameRecord,
    player::{
        PlayerId, RegisteredPlayer,
        color::{ColorIdentity, MtgColor},
    },
    tournament::analytics::Analytics,
};

impl<'a> Analytics<'a> {
    pub(crate) fn player_performance(
        self,
        id: PlayerId,
    ) -> Result<HashMap<PlayerId, MatchPerformance>, TournamentError> {
        self.tourn.require_id_registered(id)?;

        let mut map = self
            .tourn
            .get_player_games(id)?
            .flat_map(|game| game_match_perfs(game, |i| i == id))
            .into_grouping_map()
            .sum();

        if !self.include_precons {
            map.retain(|id, _| {
                self.tourn
                    .get_player_info(id)
                    .is_none_or(|info| !info.is_precon())
            });
        }

        Ok(map)
    }

    pub(crate) fn player_performance_all(
        self,
        id: PlayerId,
    ) -> Result<HashMap<PlayerId, MatchPerformance>, TournamentError> {
        let mut map = self.player_performance(id)?;
        for (id, info) in self.tourn.players() {
            if self.include_precons || !info.is_precon() {
                map.entry(*id).or_default();
            }
        }
        Ok(map)
    }

    pub(crate) fn player_performance_all_others(
        self,
        id: PlayerId,
    ) -> Result<HashMap<PlayerId, MatchPerformance>, TournamentError> {
        let mut map = self.player_performance_all(id)?;
        map.remove(&id);
        Ok(map)
    }

    pub(crate) fn player_vs_player_all_performances(
        self,
        id: PlayerId,
    ) -> Result<impl Iterator<Item = (RegisteredPlayer<'a>, MatchPerformance)> + 'a, TournamentError> {
        Ok(self
            .player_performance(id)?
            .into_iter()
            .filter_map(|(id, perf)| Some((self.tourn.get_registered_player(id)?, perf))))
    }

    pub fn player_vs_player_performance(
        self,
        id: PlayerId,
    ) -> Result<impl Iterator<Item = (RegisteredPlayer<'a>, MatchPerformance)> + 'a, TournamentError> {
        let players = self.player_vs_player_all_performances(id)?;
        let filtered = players.filter(move |(player, _)| player.id() != id);
        Ok(filtered)
    }

    pub fn player_vs_identity_performance(
        self,
        id: PlayerId,
    ) -> Result<HashMap<ColorIdentity, MatchPerformance>, TournamentError> {
        Ok(self
            .player_vs_player_all_performances(id)?
            .map(|(player, perf)| (player.info().color_identity(), perf))
            .into_grouping_map()
            .sum())
    }

    pub fn player_vs_color_performance(
        self,
        id: PlayerId,
    ) -> Result<HashMap<MtgColor, MatchPerformance>, TournamentError> {
        Ok(self
            .player_vs_player_all_performances(id)?
            .flat_map(|(player, perf)| {
                player
                    .info()
                    .color_identity()
                    .colors()
                    .map(move |color| (color, perf))
            })
            .into_grouping_map()
            .sum())
    }
}

impl Analytics<'_> {
    #[must_use]
    pub fn identity_vs_identity_performance(
        self,
        identity: ColorIdentity,
    ) -> HashMap<ColorIdentity, MatchPerformance> {
        self.tourn
            .games()
            .iter()
            .flat_map(|game| {
                game_match_perfs(game, |id| {
                    let Some(info) = self.tourn.get_player_info(&id) else {
                        return false;
                    };
                    (self.include_precons || !info.is_precon()) && info.color_identity() == identity
                })
            })
            .filter_map(|(id, perf)| {
                let info = self.tourn.get_player_info(&id)?;
                (self.include_precons || !info.is_precon()).then_some((info.color_identity(), perf))
            })
            .into_grouping_map()
            .sum()
    }

    #[must_use]
    pub fn identity_vs_color_performance(
        self,
        identity: ColorIdentity,
    ) -> HashMap<MtgColor, MatchPerformance> {
        self.identity_vs_identity_performance(identity)
            .into_iter()
            .flat_map(|(identity, perf)| identity.colors().map(move |color| (color, perf)))
            .into_grouping_map()
            .sum()
    }
}

impl Analytics<'_> {
    #[must_use]
    pub fn color_vs_color_performance(self, color: MtgColor) -> HashMap<MtgColor, MatchPerformance> {
        self.tourn
            .games()
            .iter()
            .flat_map(|game| {
                game_match_perfs(game, |id| {
                    let Some(info) = self.tourn.get_player_info(&id) else {
                        return false;
                    };
                    (self.include_precons || !info.is_precon()) && info.color_identity().has_color(color)
                })
            })
            .filter_map(|(id, perf)| {
                let info = self.tourn.get_player_info(&id)?;
                (self.include_precons || !info.is_precon()).then_some((info.color_identity(), perf))
            })
            .flat_map(|(identity, perf)| identity.colors().map(move |color| (color, perf)))
            .into_grouping_map()
            .sum()
    }
}

fn game_match_perfs<F>(game: &GameRecord, check: F) -> impl Iterator<Item = (PlayerId, MatchPerformance)>
where
    F: Fn(PlayerId) -> bool,
{
    let winner = game.winner;
    let losers = game.losers();
    let mut perfs = Vec::new();

    if check(winner) {
        for &loser in &losers {
            perfs.push((loser, MatchPerformance::WIN));
        }
    }

    for (i, &loser) in losers.iter().enumerate() {
        if check(loser) {
            perfs.push((winner, MatchPerformance::LOSS));
            for (j, &other_loser) in losers.iter().enumerate() {
                if i != j {
                    perfs.push((other_loser, MatchPerformance::DRAW));
                }
            }
        }
    }

    perfs.into_iter()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::{match_player::MatchPlayer, matchup::Matchup};
    use crate::player::stats::PlayerStats;

    fn make_game(players: Vec<PlayerId>, winner: PlayerId) -> GameRecord {
        let match_players = players
            .into_iter()
            .map(|id| MatchPlayer::new(id, PlayerStats::new(1000.0), 0.0, 1.0, 1.0))
            .collect();
        let matchup = Matchup::new(match_players, 0);
        matchup.record(winner).unwrap()
    }

    #[test]
    fn two_player_game_performances() {
        let p1 = PlayerId(1);
        let p2 = PlayerId(2);
        let game = make_game(vec![p1, p2], p1);

        // From winner's perspective (p1)
        let winner_perfs: Vec<_> = game_match_perfs(&game, |id| id == p1).collect();
        assert_eq!(winner_perfs, vec![(p2, MatchPerformance::WIN)]);
        let first_win = winner_perfs.first().unwrap();
        assert_eq!(first_win.1.wins(), 1);
        assert_eq!(first_win.1.losses(), 0);
        assert_eq!(first_win.1.draws(), 0);

        // From loser's perspective (p2)
        let loser_perfs: Vec<_> = game_match_perfs(&game, |id| id == p2).collect();
        assert_eq!(loser_perfs, vec![(p1, MatchPerformance::LOSS)]);
        let first_loss = loser_perfs.first().unwrap();
        assert_eq!(first_loss.1.wins(), 0);
        assert_eq!(first_loss.1.losses(), 1);
        assert_eq!(first_loss.1.draws(), 0);

        // Check false predicate
        let none_perfs: Vec<_> = game_match_perfs(&game, |_| false).collect();
        assert_eq!(none_perfs, Vec::<(PlayerId, MatchPerformance)>::new());
    }

    #[test]
    fn three_player_game_performances() {
        let p1 = PlayerId(1);
        let p2 = PlayerId(2);
        let p3 = PlayerId(3);
        let game = make_game(vec![p1, p2, p3], p1);

        // Winner (p1) gets a WIN against each loser
        let winner_perfs: Vec<_> = game_match_perfs(&game, |id| id == p1).collect();
        assert_eq!(
            winner_perfs,
            vec![(p2, MatchPerformance::WIN), (p3, MatchPerformance::WIN)]
        );

        // Loser 1 (p2) gets a LOSS against winner and DRAW against other loser (p3)
        let p2_perfs: Vec<_> = game_match_perfs(&game, |id| id == p2).collect();
        assert_eq!(
            p2_perfs,
            vec![(p1, MatchPerformance::LOSS), (p3, MatchPerformance::DRAW)]
        );
        let mut p2_it = p2_perfs.into_iter();
        assert_eq!(p2_it.next().unwrap().1.losses(), 1);
        assert_eq!(p2_it.next().unwrap().1.draws(), 1);

        // Loser 2 (p3) gets a LOSS against winner and DRAW against other loser (p2)
        let p3_perfs: Vec<_> = game_match_perfs(&game, |id| id == p3).collect();
        assert_eq!(
            p3_perfs,
            vec![(p1, MatchPerformance::LOSS), (p2, MatchPerformance::DRAW)]
        );
        let mut p3_it = p3_perfs.into_iter();
        assert_eq!(p3_it.next().unwrap().1.losses(), 1);
        assert_eq!(p3_it.next().unwrap().1.draws(), 1);
    }

    #[test]
    fn four_player_game_performances() {
        let p1 = PlayerId(1);
        let p2 = PlayerId(2);
        let p3 = PlayerId(3);
        let p4 = PlayerId(4);
        let game = make_game(vec![p1, p2, p3, p4], p2); // p2 wins

        // Winner (p2) gets WINs against p1, p3, p4
        let winner_perfs: Vec<_> = game_match_perfs(&game, |id| id == p2).collect();
        assert_eq!(
            winner_perfs,
            vec![
                (p1, MatchPerformance::WIN),
                (p3, MatchPerformance::WIN),
                (p4, MatchPerformance::WIN),
            ]
        );

        // Loser (p1) gets LOSS against p2, DRAWs against p3 and p4
        let p1_perfs: Vec<_> = game_match_perfs(&game, |id| id == p1).collect();
        assert_eq!(
            p1_perfs,
            vec![
                (p2, MatchPerformance::LOSS),
                (p3, MatchPerformance::DRAW),
                (p4, MatchPerformance::DRAW),
            ]
        );
        let mut p1_it = p1_perfs.iter();
        assert_eq!(p1_it.next().unwrap().1, MatchPerformance::LOSS);
        assert_eq!(p1_it.next().unwrap().1, MatchPerformance::DRAW);
        assert_eq!(p1_it.next().unwrap().1, MatchPerformance::DRAW);

        // Aggregate for a participant across opponents
        let p1_aggregated: HashMap<PlayerId, MatchPerformance> =
            p1_perfs.into_iter().into_grouping_map().sum();
        assert_eq!(p1_aggregated.get(&p2).copied(), Some(MatchPerformance::LOSS));
        assert_eq!(p1_aggregated.get(&p3).copied(), Some(MatchPerformance::DRAW));
        assert_eq!(p1_aggregated.get(&p4).copied(), Some(MatchPerformance::DRAW));
    }

    #[test]
    fn five_player_game_performances() {
        let ids: Vec<PlayerId> = (1..=5).map(PlayerId).collect();
        let winner = *ids.first().unwrap();
        let game = make_game(ids.clone(), winner);

        let winner_perfs: Vec<_> = game_match_perfs(&game, |id| id == winner).collect();
        assert_eq!(winner_perfs.len(), 4);
        assert!(winner_perfs.iter().all(|(_, p)| *p == MatchPerformance::WIN));

        // Each of the 4 losers gets 1 loss and 3 draws
        for &loser in ids.iter().skip(1) {
            let loser_perfs: Vec<_> = game_match_perfs(&game, |id| id == loser).collect();
            assert_eq!(loser_perfs.len(), 4);
            assert_eq!(loser_perfs.first().copied(), Some((winner, MatchPerformance::LOSS)));
            for other in loser_perfs.iter().skip(1) {
                assert_eq!(other.1, MatchPerformance::DRAW);
            }
        }
    }
}
