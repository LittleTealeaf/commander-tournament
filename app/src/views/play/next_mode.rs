use core::cmp::Reverse;

use auto_const_array::auto_const_array;
use edh_tourn::game::POD_SIZE;
use edh_tourn::game::record::GameRecord;
use edh_tourn::player::PlayerId;
use edh_tourn::{player::RegisteredPlayer, tournament::Tournament};
use im::OrdSet;
use itertools::Itertools;

#[derive(Debug, Copy, Clone, PartialEq, Eq, derive_more::Display, Default)]
pub enum PlayNextMode {
    #[display("Least Games")]
    LeastGames,
    #[display("Longest Break")]
    #[default]
    LongestBreak,
    #[display("Longest Lead Break")]
    LongestLeadBreak,
    #[display("Least Wins")]
    LeastWins,
    #[display("Outlier Winrate")]
    OutlierWinrate,
    #[display("Longest Since Win")]
    LongestSinceWin,
    #[display("Closest to Peak")]
    PeakElo,
}

impl PlayNextMode {
    auto_const_array! {
        pub const VALUES: [Self; _] = [
            Self::LongestBreak,
            Self::LongestLeadBreak,
            Self::LeastGames,
            Self::LeastWins,
            Self::OutlierWinrate,
            Self::LongestSinceWin,
            Self::PeakElo
        ]
    }

    #[must_use]
    pub fn get_player(self, tournament: &Tournament) -> Option<RegisteredPlayer<'_>> {
        self.get_player_from_list(tournament, tournament.get_registered_players())
    }

    #[must_use]
    pub fn get_player_from_list<'a, I>(
        self,
        tournament: &'a Tournament,
        players: I,
    ) -> Option<RegisteredPlayer<'a>>
    where
        I: IntoIterator<Item = RegisteredPlayer<'a>>,
    {
        let players = players.into_iter();

        match self {
            Self::PeakElo => players.min_by(|a, b| {
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
            }),
            Self::LongestSinceWin => {
                let mut ids = OrdSet::new();
                for player in players {
                    if player.stats().wins() == 0 {
                        return Some(player);
                    }
                    ids.insert(player.id());
                }

                if ids.is_empty() {
                    return None;
                }

                let (id, _) = tournament
                    .games()
                    .iter()
                    .map(GameRecord::winner)
                    .filter(|winner| ids.contains(winner))
                    .enumerate()
                    .map(|(game, player)| (player, game))
                    .into_grouping_map()
                    .max()
                    .into_iter()
                    .min_by_key(|(_, i)| *i)?;
                tournament.get_registered_player(id)
            }
            Self::LeastGames => players.min_by_key(|player| (player.stats().games(), player.id())),
            Self::LeastWins => players.min_by_key(|player| {
                (
                    player.stats().wins(),
                    Reverse(player.stats().games()),
                    player.id(),
                )
            }),
            Self::OutlierWinrate => players.min_by(|left, right| {
                #[allow(clippy::cast_precision_loss, reason = "u32 to f64 casting")]
                let target = 1.0 / (POD_SIZE as f64);
                let left_diff = (target - left.stats().wr_unwrap()).abs();
                let right_diff = (target - right.stats().wr_unwrap()).abs();
                left_diff
                    .total_cmp(&right_diff)
                    .reverse()
                    .then_with(|| left.id().cmp(&right.id()))
            }),
            Self::LongestBreak => {
                let players = players.collect::<Vec<_>>();
                for player in &players {
                    if player.stats().games() == 0 {
                        return Some(*player);
                    }
                }
                let mut ids = players
                    .into_iter()
                    .map(|pl| pl.id())
                    .collect::<OrdSet<PlayerId>>();

                for game in tournament.games().iter().rev() {
                    for player in game.players() {
                        if let Some(id) = ids.remove(&player.id())
                            && ids.is_empty()
                        {
                            return tournament.get_registered_player(id);
                        }
                    }
                }

                tournament.get_registered_player(ids.into_iter().next()?)
            }
            Self::LongestLeadBreak => {
                let players = players.collect::<Vec<_>>();
                for player in &players {
                    if player.stats().games() == 0 {
                        return Some(*player);
                    }
                }

                let filtered_ids = players
                    .into_iter()
                    .map(|player| player.id())
                    .collect::<OrdSet<PlayerId>>();
                let games = tournament.games().iter();
                let leads = games.map(|game| {
                    let [lead, ..] = game.players();
                    lead.id()
                });
                let filtered_leads = leads.filter(|id| filtered_ids.contains(id));
                let enumerated = filtered_leads.enumerate().map(|(game, player)| (player, game));
                let max_game_id = enumerated.into_grouping_map().max();
                let (id, _) = max_game_id.into_iter().min_by_key(|(_, i)| *i)?;
                tournament.get_registered_player(id)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use edh_tourn::game::entry::GameEntry;

    use super::*;

    #[test]
    fn longest_since_win_prioritizes_zero_wins() {
        let mut tournament = Tournament::new();

        // Create players
        let players: [PlayerId; 4] = tournament.register_debug_players().unwrap();
        let [target, others @ ..] = players;

        for winner in others {
            let entry = GameEntry::new(players, winner).unwrap();
            tournament.record_entry(entry).unwrap();
        }

        let next_player = PlayNextMode::LongestSinceWin.get_player(&tournament).unwrap();
        assert_eq!(next_player.id(), target);
    }

    #[test]
    fn longest_since_win_takes_oldest_win() {
        let mut tournament = Tournament::new();
        let players: [PlayerId; 4] = tournament.register_debug_players().unwrap();

        for winner in players {
            let entry = GameEntry::new(players, winner).unwrap();
            tournament.record_entry(entry).unwrap();
        }

        let [target, ..] = players;

        let next_player = PlayNextMode::LongestSinceWin.get_player(&tournament).unwrap();

        assert_eq!(next_player.id(), target);
    }
}
