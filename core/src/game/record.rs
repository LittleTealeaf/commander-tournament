use itertools::Itertools;

use crate::{
    error::TournamentError,
    game::{match_player::MatchPlayer, matchup::Matchup},
    player::PlayerId,
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct GameRecord {
    pub matchup: Matchup,
    pub winner: PlayerId,
}

impl GameRecord {
    pub fn new(matchup: Matchup, winner: PlayerId) -> Result<Self, TournamentError> {
        if !matchup.ids().contains(&winner) {
            return Err(TournamentError::PlayerNotInMatch(winner));
        }

        Ok(Self { matchup, winner })
    }

    #[must_use]
    pub fn has_player(&self, id: PlayerId) -> bool {
        self.get_player(id).is_some()
    }

    #[must_use]
    pub fn get_player(&self, id: PlayerId) -> Option<&MatchPlayer> {
        self.matchup.get_player(id)
    }

    #[must_use]
    pub fn players(&self) -> &Vec<MatchPlayer> {
        self.matchup.players()
    }

    pub fn ids(&self) -> impl Iterator<Item = PlayerId> {
        self.matchup.ids()
    }

    #[must_use]
    pub fn losers(&self) -> Vec<PlayerId> {
        let mut found = false;
        self.ids()
            .filter(|&id| {
                if id == self.winner && !found {
                    found = true;
                    false
                } else {
                    true
                }
            })
            .collect()
    }

    pub fn get_player_elo_change(&self, id: PlayerId) -> Result<f64, TournamentError> {
        let mut score = 0.0;
        let mut won = false;

        for player in self.matchup.players() {
            if player.id() != id {
                continue;
            }
            if player.id() == self.winner && !won {
                won = true;
                score += player.elo_win();
            } else {
                score -= player.elo_loss();
            }
        }

        Ok(score)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tournament::Tournament;

    #[test]
    fn losers_with_duplicates() {
        let mut tournament = Tournament::new();
        let player_a = tournament.register_debug_player().unwrap();

        let matchup = tournament.create_match(vec![player_a; 4]).unwrap();
        let record = matchup.record(player_a).unwrap();
        assert_eq!(record.winner, player_a);
        assert_eq!(record.losers(), vec![player_a; 3]);

        let player_b = tournament.register_debug_player().unwrap();
        let matchup = tournament
            .create_match(vec![player_a, player_a, player_b, player_b])
            .unwrap();
        let record = matchup.record(player_a).unwrap();
        assert_eq!(record.winner, player_a);
        assert_eq!(record.losers(), vec![player_a, player_b, player_b]);
    }

    #[test]
    fn losers_variable_player_counts() {
        let mut tournament = Tournament::new();
        let p1 = tournament.register_debug_player().unwrap();
        let p2 = tournament.register_debug_player().unwrap();
        let p3 = tournament.register_debug_player().unwrap();
        let p4 = tournament.register_debug_player().unwrap();

        // 2 players: winner p1, loser p2
        let m2 = tournament.create_match(vec![p1, p2]).unwrap();
        let r2 = m2.record(p1).unwrap();
        assert_eq!(r2.winner, p1);
        assert_eq!(r2.losers(), vec![p2]);

        // 3 players: winner p2, losers p1, p3
        let m3 = tournament.create_match(vec![p1, p2, p3]).unwrap();
        let r3 = m3.record(p2).unwrap();
        assert_eq!(r3.winner, p2);
        assert_eq!(r3.losers(), vec![p1, p3]);

        // 4 players: winner p4, losers p1, p2, p3
        let m4 = tournament.create_match(vec![p1, p2, p3, p4]).unwrap();
        let r4 = m4.record(p4).unwrap();
        assert_eq!(r4.winner, p4);
        assert_eq!(r4.losers(), vec![p1, p2, p3]);
    }

    #[test]
    fn losers_direct_matchup() {
        let p1 = PlayerId(10);
        let p2 = PlayerId(20);
        let p3 = PlayerId(30);
        let make_mp =
            |id| MatchPlayer::new(id, crate::player::stats::PlayerStats::new(1000.0), 0.0, 1.0, 1.0);

        // 2 players
        let m2 = Matchup::new(vec![make_mp(p1), make_mp(p2)], 0);
        let r2 = m2.record(p2).unwrap();
        assert_eq!(r2.losers(), vec![p1]);

        // 3 players
        let m3 = Matchup::new(vec![make_mp(p1), make_mp(p2), make_mp(p3)], 0);
        let r3 = m3.record(p1).unwrap();
        assert_eq!(r3.losers(), vec![p2, p3]);
    }
}
