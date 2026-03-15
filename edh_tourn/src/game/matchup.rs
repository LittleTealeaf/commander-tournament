use crate::{
    Tournament,
    error::TournamentError,
    game::{match_player::MatchPlayer, record::GameRecord},
    player::stats::PlayerStats,
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Matchup {
    players: [MatchPlayer; 4],
    snapshot: usize,
}

impl Matchup {
    #[must_use]
    pub(crate) const fn new(players: [MatchPlayer; 4], snapshot: usize) -> Self {
        Self { players, snapshot }
    }

    #[must_use]
    pub const fn snapshot(&self) -> usize {
        self.snapshot
    }

    #[must_use]
    pub const fn players(&self) -> &[MatchPlayer; 4] {
        &self.players
    }

    #[must_use]
    pub const fn has_player(&self, id: u32) -> bool {
        self.get_player(id).is_some()
    }

    #[must_use]
    pub const fn get_player(&self, id: u32) -> Option<&MatchPlayer> {
        let [a, b, c, d] = &self.players();
        if a.id() == id {
            Some(a)
        } else if b.id() == id {
            Some(b)
        } else if c.id() == id {
            Some(c)
        } else if d.id() == id {
            Some(d)
        } else {
            None
        }
    }

    #[must_use]
    pub const fn ids(&self) -> [u32; 4] {
        let [player_a, player_b, player_c, player_d] = &self.players;
        [player_a.id(), player_b.id(), player_c.id(), player_d.id()]
    }

    pub const fn record(self, winner: u32) -> Result<GameRecord, TournamentError> {
        GameRecord::new(self, winner)
    }
}

impl Tournament {
    pub fn update_match(&self, matchup: Matchup) -> Result<Matchup, TournamentError> {
        if matchup.snapshot() == self.snapshot {
            return Ok(matchup);
        }
        self.create_match(matchup.ids())
    }

    #[must_use]
    pub fn create_match_players<const T: usize>(&self, players: [u32; T]) -> [MatchPlayer; T] {
        struct TempMatchPlayer<'a> {
            id: u32,
            stats: &'a PlayerStats,
            scaled_elo: f64,
            scaled_wr: f64,
        }

        #[allow(clippy::cast_precision_loss)]
        let base_chance = 1.0 / (T as f64);

        // let base_chance = (0..T).map(|_| 1.0).sum::<f64>().powi(-1);
        let config = self.game_config();

        let id_stats = players.map(|id| {
            let stats = self.get_player_or_default_stats(id);
            TempMatchPlayer {
                scaled_wr: stats
                    .wr()
                    .unwrap_or(base_chance)
                    .powf(self.game_config().game_wr_pow_scale),
                scaled_elo: stats.elo().powf(self.game_config().game_elo_pow_scale),
                stats,
                id,
            }
        });

        let sum_wr: f64 = id_stats.iter().map(|p| p.scaled_wr).sum();
        let sum_elo: f64 = id_stats.iter().map(|p| p.scaled_elo).sum();

        let weight_total = config.game_wr_weight + config.game_elo_weight;
        let coef_wr = config.game_wr_weight / (weight_total * sum_wr);
        let coef_elo = config.game_elo_weight / (weight_total * sum_elo);
        let base_loss = 1.0 - base_chance;

        id_stats.map(|player| {
            let game_points = config.game_points;
            let expected = coef_wr.mul_add(player.scaled_wr, coef_elo * player.scaled_elo);
            let elo_win = game_points * (1.0 - expected) / base_loss;
            let elo_loss = game_points * expected / base_loss;
            MatchPlayer::new(player.id, player.stats.clone(), expected, elo_win, elo_loss)
        })
    }

    pub fn create_match(&self, ids: [u32; 4]) -> Result<Matchup, TournamentError> {
        // First check registration
        for id in &ids {
            if !self.is_id_registered(id) {
                return Err(TournamentError::InvalidPlayerId(*id));
            }
        }

        Ok(Matchup::new(self.create_match_players(ids), self.snapshot))
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
