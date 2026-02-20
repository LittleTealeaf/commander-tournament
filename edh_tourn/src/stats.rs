use crate::Tournament;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
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
    #[must_use]
    pub const fn elo(&self) -> f64 {
        self.elo
    }

    #[must_use]
    pub const fn games(&self) -> u32 {
        self.games
    }

    #[must_use]
    pub const fn wins(&self) -> u32 {
        self.wins
    }

    #[must_use]
    pub fn wr(&self) -> Option<f64> {
        (self.games > 0).then(|| f64::from(self.wins) / f64::from(self.games))
    }
}

impl Tournament {
    #[must_use]
    pub const fn create_default_stats(&self) -> PlayerStats {
        PlayerStats {
            elo: self.config.starting_elo,
            games: 0,
            wins: 0,
        }
    }

    #[must_use]
    pub fn get_player_stats(&self, player: u32) -> Option<&PlayerStats> {
        self.stats.get(&player)
    }
}

#[cfg(test)]
mod tests {
    use crate::Tournament;

    #[test]
    fn default_stats_use_starting_elo() {
        let tournament = Tournament::default();
        let starting_elo = tournament.config.starting_elo;
        let stats = tournament.create_default_stats();
        assert!(starting_elo.total_cmp(&stats.elo).is_eq());
    }

    #[test]
    fn all_players_start_with_default_elo() {
        let tournament = Tournament::generate_tournament(100, 0).unwrap();
        let starting_elo = tournament.config.starting_elo;
        for stats in tournament.stats.values() {
            assert!(starting_elo.total_cmp(&stats.elo).is_eq());
        }
    }
}
