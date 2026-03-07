use crate::{
    Tournament,
    error::TournamentError,
    game::{entry::GameEntry, match_player::MatchPlayer, matchup::Matchup},
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct GameRecord {
    matchup: Matchup,
    winner: u32,
}

impl GameRecord {
    pub fn new(matchup: Matchup, winner: u32) -> Result<Self, TournamentError> {
        let winner_in_matchup = matchup
            .players()
            .iter()
            .map(MatchPlayer::id)
            .any(|i| i == winner);
        if !winner_in_matchup {
            return Err(TournamentError::PlayerNotInMatch(winner));
        }

        Ok(Self { matchup, winner })
    }

    #[must_use]
    pub fn has_player(&self, id: u32) -> bool {
        self.matchup
            .players()
            .iter()
            .any(|player| player.id() == id)
    }

    #[must_use]
    pub const fn matchup(&self) -> &Matchup {
        &self.matchup
    }

    pub fn get_player(&self, id: u32) -> Result<&MatchPlayer, TournamentError> {
        self.players()
            .iter()
            .find(|player| player.id() == id)
            .ok_or(TournamentError::PlayerNotInMatch(id))
    }

    #[must_use]
    pub const fn players(&self) -> &[MatchPlayer; 4] {
        self.matchup().players()
    }

    #[must_use]
    pub const fn ids(&self) -> [u32; 4] {
        self.matchup.ids()
    }

    #[must_use]
    pub const fn winner(&self) -> u32 {
        self.winner
    }

    pub fn get_player_elo_change(&self, id: u32) -> Result<f64, TournamentError> {
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

impl Tournament {
    pub fn update_record(&self, record: GameRecord) -> Result<GameRecord, TournamentError> {
        self.update_match(record.matchup)?.record(record.winner)
    }
}

impl From<GameRecord> for GameEntry {
    fn from(value: GameRecord) -> Self {
        Self::new_unchecked(value.ids(), value.winner)
    }
}
