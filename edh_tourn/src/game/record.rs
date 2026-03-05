use crate::{
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
            return Err(TournamentError::WinnerNotInMatch(winner));
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

    #[must_use]
    pub const fn players(&self) -> &[MatchPlayer; 4] {
        self.matchup().players()
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

impl From<GameRecord> for (Matchup, u32) {
    fn from(value: GameRecord) -> Self {
        (value.matchup, value.winner)
    }
}

impl TryFrom<GameRecord> for GameEntry {
    type Error = TournamentError;
    fn try_from(value: GameRecord) -> Result<Self, Self::Error> {
        let [a, b, c, d] = value.matchup.players();
        let players = [a.id(), b.id(), c.id(), d.id()];
        let winner = value.winner;
        Self::new(players, winner)
    }
}

impl<'a> TryFrom<&'a GameRecord> for GameEntry {
    type Error = TournamentError;
    fn try_from(value: &'a GameRecord) -> Result<Self, Self::Error> {
        let [a, b, c, d] = value.matchup.players();
        let players = [a.id(), b.id(), c.id(), d.id()];
        let winner = value.winner;
        Self::new(players, winner)
    }
}
