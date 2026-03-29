use itertools::Itertools;

use crate::{
    error::TournamentError,
    game::{POD_SIZE, match_player::MatchPlayer, matchup::Matchup},
    player::PlayerId,
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct GameRecord {
    matchup: Matchup,
    winner: PlayerId,
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
    pub const fn matchup(&self) -> &Matchup {
        &self.matchup
    }

    #[must_use]
    pub fn get_player(&self, id: PlayerId) -> Option<&MatchPlayer> {
        self.matchup().get_player(id)
    }

    #[must_use]
    pub const fn players(&self) -> &[MatchPlayer; POD_SIZE] {
        self.matchup().players()
    }

    #[must_use]
    pub fn ids(&self) -> [PlayerId; POD_SIZE] {
        self.matchup.ids()
    }

    #[must_use]
    pub const fn winner(&self) -> PlayerId {
        self.winner
    }

    #[must_use]
    pub fn losers(&self) -> [PlayerId; POD_SIZE - 1] {
        self.ids()
            .into_iter()
            .filter(|&id| id != self.winner)
            .collect_array()
            .expect("Incorrect number of losers")
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

    #[must_use]
    pub const fn decompose(self) -> (Matchup, PlayerId) {
        (self.matchup, self.winner)
    }
}
