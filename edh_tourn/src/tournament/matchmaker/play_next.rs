use core::cmp::Reverse;

use im::OrdSet;

use crate::{
    error::TournamentError,
    game::{POD_SIZE, matchup::Matchup, next_mode::NextPlayerMode},
    player::PlayerId,
    tournament::matchmaker::Matchmaker,
};

impl Matchmaker<'_> {
    pub fn play_next(self, mode: NextPlayerMode) -> Result<Matchup, TournamentError> {
        let Some(player) = self.get_next_lead_player(mode) else {
            return Err(TournamentError::NotEnoughPlayers);
        };

        let matchup = self.create_match(player)?;

        Ok(matchup)
    }

    pub fn get_next_lead_player(self, mode: NextPlayerMode) -> Option<PlayerId> {
        let players = self
            .0
            .registered_players()
            .filter(|player| !(self.0.matchmaker_config().exclude_precons && player.info().is_precon()));

        match mode {
            NextPlayerMode::LeastGames => players
                .min_by_key(|player| (player.stats().games(), player.id()))
                .map(|p| p.id()),
            NextPlayerMode::LeastWins => players
                .min_by_key(|player| {
                    (
                        player.stats().wins(),
                        Reverse(player.stats().games()),
                        player.id(),
                    )
                })
                .map(|p| p.id()),
            NextPlayerMode::LongestBreak => {
                let mut pool = players.map(|player| player.id()).collect::<OrdSet<_>>();

                if pool.len() <= 1 {
                    return pool.into_iter().next();
                }

                let games = self.0.games().iter().rev();

                for game in games {
                    for player in game.players() {
                        pool.remove(&player.id());
                        if pool.len() <= 1 {
                            return pool.into_iter().next();
                        }
                    }
                }

                pool.into_iter().next()
            }
            NextPlayerMode::LongestLeadBreak => {
                let mut pool = players.map(|player| player.id()).collect::<OrdSet<_>>();

                if pool.len() <= 1 {
                    return pool.into_iter().next();
                }

                let games = self.0.games().iter().rev();

                for game in games {
                    let [lead, ..] = game.players();
                    pool.remove(&lead.id());

                    if pool.len() <= 1 {
                        return pool.into_iter().next();
                    }
                }

                pool.into_iter().next()
            }
            NextPlayerMode::LongestSinceWin => {
                let mut pool = OrdSet::new();

                for player in players {
                    if player.stats().wins() == 0 {
                        return Some(player.id());
                    }
                    pool.insert(player.id());
                }

                if pool.len() <= 1 {
                    return pool.into_iter().next();
                }

                for game in self.0.games().iter().rev() {
                    pool.remove(&game.winner());
                    if pool.len() <= 1 {
                        return pool.into_iter().next();
                    }
                }

                pool.into_iter().next()
            }
            NextPlayerMode::OutlierWinrate => {
                #[allow(clippy::cast_precision_loss, reason = "u32 to f64 casting")]
                let target = 1.0 / (POD_SIZE as f64);
                if self.0.config.matchmaker_config().outlier_include_extremes {
                    players
                        .min_by(|left, right| {
                            let left_diff = (target - left.stats().wr_unwrap()).abs();
                            let right_diff = (target - right.stats().wr_unwrap()).abs();
                            left_diff
                                .total_cmp(&right_diff)
                                .reverse()
                                .then_with(|| left.id().cmp(&right.id()))
                        })
                        .map(|p| p.id())
                } else {
                    #[derive(Copy, Clone)]
                    struct Entry {
                        id: PlayerId,
                        wr: f64,
                        outlier: f64,
                        elo: f64,
                    }

                    let mut players = players.map(|pl| Entry {
                        id: pl.id(),
                        wr: pl.stats().wr_unwrap(),
                        outlier: (pl.stats().wr_unwrap() - target).abs(),
                        elo: pl.stats().elo(),
                    });

                    let mut lowest = players.next()?;
                    let mut highest = lowest;
                    let mut outlier: Option<Entry> = None;

                    for player in players {
                        if lowest.elo > player.elo {
                            if outlier.is_none_or(|outlier| outlier.outlier < lowest.outlier) {
                                outlier = Some(lowest);
                            }
                            lowest = player;
                            if player.wr < target {
                                continue;
                            }
                        }

                        if highest.elo < player.elo {
                            if outlier.is_none_or(|outlier| outlier.outlier < highest.outlier) {
                                outlier = Some(highest);
                            }
                            highest = player;
                            if player.wr > target {
                                continue;
                            }
                        }

                        if outlier.is_none_or(|outlier| outlier.outlier < player.outlier) {
                            outlier = Some(player);
                        }
                    }

                    outlier.map(|outlier| outlier.id)
                }
            }
            NextPlayerMode::PeakElo => players
                .min_by(|a, b| {
                    let stats_a = a.stats();
                    let elo_a = stats_a.elo();
                    let diff_a = stats_a.elo_peak() - elo_a;

                    let stats_b = b.stats();
                    let elo_b = stats_b.elo();
                    let diff_b = stats_b.elo_peak() - elo_b;

                    diff_a
                        .total_cmp(&diff_b)
                        .then_with(|| elo_a.total_cmp(&elo_b).reverse())
                        .then_with(|| a.id().cmp(&b.id()))
                })
                .map(|p| p.id()),
        }
    }
}
