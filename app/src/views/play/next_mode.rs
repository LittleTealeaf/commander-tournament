use core::cmp::Reverse;

use auto_const_array::auto_const_array;
use edh_tourn::game::POD_SIZE;
use edh_tourn::player::PlayerId;
use edh_tourn::{player::RegisteredPlayer, tournament::Tournament};
use im::{OrdMap, OrdSet, ordset};
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
}

impl PlayNextMode {
    auto_const_array! {
        pub const VALUES: [Self; _] = [
            Self::LongestBreak,
            Self::LongestLeadBreak,
            Self::LeastGames,
            Self::LeastWins,
            Self::OutlierWinrate,
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
            Self::LeastGames => players.min_by_key(|player| (player.stats().games(), player.id())),
            Self::LeastWins => players.min_by_key(|player| {
                (
                    player.stats().wins(),
                    Reverse(player.stats().games()),
                    player.id(),
                )
            }),
            Self::OutlierWinrate => players.min_by(|left, right| {
                #[allow(clippy::cast_precision_loss)]
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
                let enumerated = filtered_leads
                    .enumerate()
                    .map(|(game, player)| (player, game));
                let max_game_id = enumerated.into_grouping_map().max();
                let (id, _) = max_game_id.into_iter().min_by_key(|(_, i)| *i)?;
                tournament.get_registered_player(id)
            }
        }
    }
}
