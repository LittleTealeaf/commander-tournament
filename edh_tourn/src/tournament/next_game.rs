use std::collections::{HashMap, HashSet};

use crate::{
    error::TournamentError,
    game::{matchup::Matchup, next_mode::NextPlayerMode},
    player::{PlayerId, RegisteredPlayer},
    tournament::Tournament,
};

impl Tournament {
    pub fn next_game(&self, mode: NextPlayerMode) -> Result<Matchup, TournamentError> {
        self.matchmaker().create_match(
            self.next_game_player(mode)
                .ok_or(TournamentError::NotEnoughPlayers)?,
        )
    }

    fn next_game_player(&self, mode: NextPlayerMode) -> Option<PlayerId> {
        let players = self
            .registered_players()
            .filter(|player| (!player.info().is_archived()) && (!player.info().is_precon()));

        match mode {
            NextPlayerMode::LongestBreak => {
                let mut pool = players.map(|player| player.id()).collect::<HashSet<_>>();

                if pool.len() <= 1 {
                    return pool.into_iter().next();
                }

                let games = self.games().iter().rev();

                for game in games {
                    for player in game.players() {
                        pool.remove(&player.id());
                        if pool.len() <= 1 {
                            return pool.into_iter().next();
                        }
                    }
                }

                pool.into_iter().min()
            }
            NextPlayerMode::LeastPlayed => Some(
                players
                    .min_by_key(|player| (player.stats().games(), player.id()))?
                    .id(),
            ),

            NextPlayerMode::WinStreak => {
                struct Streak<'a> {
                    player: RegisteredPlayer<'a>,
                    count: u32,
                    elo_change: f64,
                }
                let mut finished_streaks = HashMap::new();

                let mut player_streaks = players
                    .map(|player| {
                        (
                            player.id(),
                            Streak {
                                player,
                                count: 0,
                                elo_change: player.stats().elo(),
                            },
                        )
                    })
                    .collect::<HashMap<_, _>>();

                for game in self.games().iter().rev() {
                    let winner = game.winner();
                    if let Some(entry) = player_streaks.get_mut(&winner)
                        && let Some(player) = game.get_player(winner)
                    {
                        entry.count += 1;
                        entry.elo_change += player.elo_win();
                    }
                    for player in game.losers() {
                        if let Some(entry) = player_streaks.remove(&player) {
                            finished_streaks.insert(player, entry);
                        }
                    }
                    if player_streaks.is_empty() {
                        break;
                    }
                }

                Some(
                    finished_streaks
                        .into_iter()
                        .max_by(|(left_id, left_streak), (right_id, right_streak)| {
                            left_streak
                                .count
                                .cmp(&right_streak.count)
                                .then_with(|| left_streak.elo_change.total_cmp(&right_streak.elo_change))
                                .then_with(|| left_id.cmp(right_id))
                        })?
                        .0,
                )
            }
        }
    }
}
