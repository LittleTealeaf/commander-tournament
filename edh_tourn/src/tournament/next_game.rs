use std::collections::HashSet;

use crate::{
    error::TournamentError,
    game::{matchup::Matchup, next_mode::NextPlayerMode, record::GameRecord},
    player::{PlayerId, RegisteredPlayer},
    tournament::Tournament,
};

impl Tournament {
    pub fn next_game(&self, mode: NextPlayerMode) -> Result<Matchup, TournamentError> {
        let Some(player) = self.next_game_player(mode) else {
            return Err(TournamentError::NotEnoughPlayers);
        };

        self.matchmaker().create_match(player)
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

            NextPlayerMode::WinStreak => calculate_streak(players, self.games(), Some(true)),
            NextPlayerMode::GameStreak => calculate_streak(players, self.games(), None),
            NextPlayerMode::LossStreak => calculate_streak(players, self.games(), Some(false)),
        }
    }
}

fn calculate_streak<'a, P>(
    players: P,
    games: &[GameRecord],
    pinned_direction: Option<bool>,
) -> Option<PlayerId>
where
    P: IntoIterator<Item = RegisteredPlayer<'a>>,
{
    let mut streaks = players
        .into_iter()
        .map(|player| (player.id(), Streak::from(player)))
        .collect::<im::HashMap<_, _>>();
    let mut finished_count = 0;

    for game in games.iter().rev() {
        for player in game.ids() {
            if let Some(entry) = streaks.get_mut(&player)
                && entry.add(game, pinned_direction)
            {
                finished_count += 1;
            }
        }
        if finished_count == streaks.len() {
            break;
        }
    }

    Some(
        streaks
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

#[derive(Clone, Debug)]
struct Streak<'a> {
    player: RegisteredPlayer<'a>,
    count: u32,
    elo_change: f64,
    is_winning: Option<bool>,
    broken: bool,
}

impl Streak<'_> {
    const fn is_winstreak(&self) -> bool {
        self.elo_change >= 0.0
    }

    /// Returns true if the streak was just broken
    fn add(&mut self, game: &GameRecord, pinned: Option<bool>) -> bool {
        if self.broken {
            return false;
        }
        let won_game = game.winner() == self.player.id();
        match self.is_winning {
            Some(is_winning) => {
                if won_game != is_winning {
                    self.broken = true;
                    return true;
                }
                if let Some(pin) = pinned
                    && pin != is_winning
                {
                    self.broken = true;
                    return true;
                }
            }
            None => {
                self.is_winning = Some(won_game);
            }
        }

        self.count += 1;
        if let Ok(change) = game.get_player_elo_change(self.player.id()) {
            self.elo_change += change.abs();
        }
        false
    }
}

impl<'a> From<RegisteredPlayer<'a>> for Streak<'a> {
    fn from(value: RegisteredPlayer<'a>) -> Self {
        Self {
            player: value,
            count: 0,
            elo_change: 0.0,
            is_winning: None,
            broken: false,
        }
    }
}
