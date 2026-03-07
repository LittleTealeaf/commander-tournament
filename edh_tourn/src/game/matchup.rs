use crate::{
    Tournament,
    error::TournamentError,
    game::{match_player::MatchPlayer, record::GameRecord},
    player::stats::PlayerStats,
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Matchup {
    players: [MatchPlayer; 4],
    version: usize,
}

impl Matchup {
    #[must_use]
    pub(crate) const fn new(players: [MatchPlayer; 4], version: usize) -> Self {
        Self { players, version }
    }

    #[must_use]
    pub const fn version(&self) -> usize {
        self.version
    }

    #[must_use]
    pub const fn players(&self) -> &[MatchPlayer; 4] {
        &self.players
    }

    #[must_use]
    pub const fn ids(&self) -> [u32; 4] {
        let [player_a, player_b, player_c, player_d] = &self.players;
        [player_a.id(), player_b.id(), player_c.id(), player_d.id()]
    }

    pub fn record(self, winner: u32) -> Result<GameRecord, TournamentError> {
        GameRecord::new(self, winner)
    }
}

struct TempMatchPlayer<'a> {
    id: u32,
    stats: &'a PlayerStats,
    scaled_elo: f64,
    scaled_wr: f64,
}

impl TempMatchPlayer<'_> {
    fn into_match_player(self, game_points: f64, coef_elo: f64, coef_wr: f64) -> MatchPlayer {
        let expected = coef_wr.mul_add(self.scaled_wr, coef_elo * self.scaled_elo);
        let elo_win = game_points * (1.0 - expected) / 0.75;
        let elo_loss = game_points * expected / 0.75;
        MatchPlayer::new(self.id, self.stats.clone(), expected, elo_win, elo_loss)
    }
}

impl Tournament {
    pub fn update_match(&self, matchup: Matchup) -> Result<Matchup, TournamentError> {
        if matchup.version() == self.snapshot {
            return Ok(matchup);
        }
        self.create_match(matchup.ids())
    }

    fn create_temp_match_player(&self, id: u32) -> TempMatchPlayer<'_> {
        let stats = self.get_player_or_default_stats(id);
        TempMatchPlayer {
            scaled_wr: stats
                .wr()
                .unwrap_or(0.25)
                .powf(self.config.game_wr_pow_scale),
            scaled_elo: stats.elo().powf(self.config.game_elo_pow_scale),
            stats,
            id,
        }
    }

    pub fn create_match(&self, ids: [u32; 4]) -> Result<Matchup, TournamentError> {
        // First check registration
        for id in &ids {
            if !self.is_id_registered(id) {
                return Err(TournamentError::InvalidPlayerId(*id));
            }
        }

        let players = ids.map(|id| self.create_temp_match_player(id));

        let sum_elo = players.iter().map(|player| player.scaled_elo).sum::<f64>();
        let sum_wr = players.iter().map(|player| player.scaled_wr).sum::<f64>();

        let weight_total = self.config.game_wr_weight + self.config.game_elo_weight;
        let weight_wr = self.config.game_wr_weight / weight_total;
        let weight_elo = self.config.game_elo_weight / weight_total;

        let coef_wr = weight_wr / sum_wr;
        let coef_elo = weight_elo / sum_elo;

        let match_players = players
            .map(|player| player.into_match_player(self.config.game_points, coef_elo, coef_wr));

        Ok(Matchup::new(match_players, self.snapshot))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn create_match_invalid_ids() {
        let tourn = Tournament::new();
        tourn.create_match([1, 2, 3, 4]).unwrap_err();
    }

    #[test]
    fn mirror_matchup_equal_expected() {
        let mut tourn = Tournament::new();
        let id = tourn.register_player("A".to_owned()).unwrap();
        let mu = tourn.create_match([id, id, id, id]).unwrap();
        for p in mu.players() {
            assert_relative_eq!(0.25, *p.expected());
        }
    }

    #[test]
    fn record_winner_must_be_player() {
        let tournament = Tournament::generate_tournament(5, 0).unwrap();
        let mut ids = tournament.players().keys().copied();
        let player_a = ids.next().unwrap();
        let player_b = ids.next().unwrap();
        let player_c = ids.next().unwrap();
        let player_d = ids.next().unwrap();
        let player_e = ids.next().unwrap();

        let mu = tournament
            .create_match([player_a, player_b, player_c, player_d])
            .unwrap();
        mu.clone().record(player_a).unwrap();
        mu.clone().record(player_b).unwrap();
        mu.clone().record(player_c).unwrap();
        mu.clone().record(player_d).unwrap();
        mu.clone().record(player_e).unwrap_err();
        mu.record(u32::MAX).unwrap_err();
    }
}
