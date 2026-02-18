use crate::tourn::Tournament;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PlayerStats {
    pub(crate) elo: f64,
    pub(crate) games: u32,
    pub(crate) wins: u32,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            elo: 0.0,
            games: 0,
            wins: 0,
        }
    }
}

impl PlayerStats {
    pub fn elo(&self) -> f64 {
        self.elo
    }

    pub fn games(&self) -> u32 {
        self.games
    }

    pub fn wins(&self) -> u32 {
        self.wins
    }

    pub fn wr(&self) -> Option<f64> {
        (self.games > 0).then(|| (self.wins as f64) / (self.games as f64))
    }
}

impl Tournament {
    pub fn create_default_stats(&self) -> PlayerStats {
        PlayerStats {
            elo: self.config.starting_elo,
            games: 0,
            wins: 0,
        }
    }

    pub fn get_player_stats(&self, player: &u32) -> Option<&PlayerStats> {
        self.stats.get(player)
    }
}

#[cfg(test)]
mod tests {
    use crate::tourn::Tournament;

    #[test]
    fn default_stats_use_starting_elo() {
        let tournament = Tournament::default();
        let starting_elo = tournament.config.starting_elo;
        let stats = tournament.create_default_stats();
        assert_eq!(starting_elo, stats.elo);
    }
}
