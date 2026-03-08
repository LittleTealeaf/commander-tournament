use core::cmp::Ordering;
use std::collections::HashMap;

use itertools::Itertools;

use crate::{Tournament, error::TournamentError, player::RegisteredPlayer};

#[derive(Debug)]
pub struct GamesPlayedRankedEntry<'a> {
    player: RegisteredPlayer<'a>,
    games_played: usize,
    games_won: usize,
    games_lost: usize,
}

impl GamesPlayedRankedEntry<'_> {
    #[must_use]
    pub const fn player(&self) -> &RegisteredPlayer<'_> {
        &self.player
    }

    #[must_use]
    pub const fn games_played(&self) -> usize {
        self.games_played
    }

    #[must_use]
    pub const fn games_won_against(&self) -> usize {
        self.games_won
    }

    #[must_use]
    pub const fn games_lost_against(&self) -> usize {
        self.games_lost
    }

    #[must_use]
    pub const fn games_lost_with(&self) -> usize {
        self.games_played - (self.games_won + self.games_lost)
    }
}

impl<'a> From<GamesPlayedRankedEntry<'a>> for RegisteredPlayer<'a> {
    fn from(value: GamesPlayedRankedEntry<'a>) -> Self {
        value.player
    }
}

fn with_elo_tie_breaker(
    cmp: Ordering,
    elo: f64,
    player_a: &GamesPlayedRankedEntry<'_>,
    player_b: &GamesPlayedRankedEntry<'_>,
) -> Ordering {
    let Ordering::Equal = cmp else {
        return cmp;
    };

    let elo_diff_a = (elo - player_a.player().stats().elo()).abs();
    let elo_diff_b = (elo - player_b.player().stats().elo()).abs();

    let cmp = elo_diff_a.total_cmp(&elo_diff_b);

    let Ordering::Equal = cmp else {
        return cmp;
    };

    player_a.player().id().cmp(&player_b.player().id())
}

impl Tournament {
    fn get_games_played_entries(
        &self,
        id: u32,
    ) -> Result<impl Iterator<Item = GamesPlayedRankedEntry<'_>>, TournamentError> {
        self.ensure_id_registered(id)?;

        let mut records: HashMap<_, _> = self
            .get_registered_players()
            .map(|player| {
                (
                    player.id(),
                    GamesPlayedRankedEntry {
                        player,
                        games_lost: 0,
                        games_won: 0,
                        games_played: 0,
                    },
                )
            })
            .collect();

        for game in self.get_player_games(id)? {
            for player in game.ids().into_iter().unique() {
                records.entry(player).and_modify(|entry| {
                    entry.games_played += 1;
                    if player == game.winner() {
                        entry.games_lost += 1;
                    } else if id == game.winner() {
                        entry.games_won += 1;
                    }
                });
            }
        }

        records.remove(&id);

        Ok(records.into_values())
    }

    pub fn get_player_ranked_nemesis(
        &self,
        id: u32,
    ) -> Result<impl Iterator<Item = GamesPlayedRankedEntry<'_>>, TournamentError> {
        let iter = self.get_games_played_entries(id)?;
        let elo_base = self.get_player_or_default_stats(id).elo();
        let sorted = iter.sorted_by(|player_a, player_b| {
            let total_games = player_a.games_played.max(player_b.games_played);
            let score_a = total_games + player_a.games_won - player_a.games_lost;
            let score_b = total_games + player_b.games_won - player_b.games_lost;

            with_elo_tie_breaker(score_a.cmp(&score_b), elo_base, player_a, player_b)
        });

        Ok(sorted)
    }

    pub fn get_player_ranked_lost_with(
        &self,
        id: u32,
    ) -> Result<impl Iterator<Item = GamesPlayedRankedEntry<'_>>, TournamentError> {
        let iter = self.get_games_played_entries(id)?;
        let elo_base = self.get_player_or_default_stats(id).elo();
        let sorted = iter.sorted_by(|player_a, player_b| {
            let score_a = player_a.games_lost_with();
            let score_b = player_b.games_lost_with();

            with_elo_tie_breaker(
                score_a.cmp(&score_b).reverse(),
                elo_base,
                player_a,
                player_b,
            )
        });
        Ok(sorted)
    }

    pub fn get_player_ranked_least_played(
        &self,
        id: u32,
    ) -> Result<impl Iterator<Item = GamesPlayedRankedEntry<'_>>, TournamentError> {
        let iter = self.get_games_played_entries(id)?;
        let elo_base = self.get_player_or_default_stats(id).elo();
        let sorted = iter.sorted_by(|player_a, player_b| {
            let score_a = player_a.games_played();
            let score_b = player_b.games_played();

            with_elo_tie_breaker(score_a.cmp(&score_b), elo_base, player_a, player_b)
        });
        Ok(sorted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn games_played_entries_dont_return_self() {
        for tourn in Tournament::test_tournaments() {
            let ids = tourn.players().keys().copied();
            for id in ids {
                let iter = tourn
                    .get_games_played_entries(id)
                    .expect("Expected Iter of Games Played");

                for item in iter {
                    assert_ne!(
                        id,
                        item.player().id(),
                        "Found id in results of the function!"
                    );
                }
            }
        }
    }

    #[test]
    fn games_played_is_greatest() {
        for tourn in Tournament::test_tournaments() {
            let ids = tourn.players().keys().copied();
            for id in ids {
                let iter = tourn
                    .get_games_played_entries(id)
                    .expect("Expected Iter of Games Played");

                for item in iter {
                    assert!(
                        item.games_played >= item.games_lost,
                        "More losses than games"
                    );
                    assert!(item.games_played >= item.games_won, "More wins than games");
                    assert!(
                        item.games_played >= item.games_won + item.games_lost,
                        "More wins and losses than games"
                    );
                }
            }
        }
    }
}
