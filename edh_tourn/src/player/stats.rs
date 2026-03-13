use crate::Tournament;

const MINIMUM_ELO: f64 = 1.0;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct PlayerStats {
    elo: f64,
    games: u32,
    wins: u32,
    elo_peak: f64,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            elo: 0.0,
            games: 0,
            wins: 0,
            elo_peak: 0.0,
        }
    }
}

impl PlayerStats {
    #[must_use]
    pub(crate) const fn new(elo: f64) -> Self {
        Self {
            elo: elo.max(MINIMUM_ELO),
            games: 0,
            wins: 0,
            elo_peak: elo,
        }
    }

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

    #[must_use]
    pub const fn elo_peak(&self) -> f64 {
        self.elo_peak
    }

    pub fn add_win(&mut self, elo_change: f64) {
        self.games += 1;
        self.wins += 1;
        self.elo += elo_change.abs();
        if self.elo > self.elo_peak {
            self.elo_peak = self.elo;
        }
    }

    pub fn add_loss(&mut self, elo_change: f64) {
        self.games += 1;
        self.elo -= elo_change.abs();
        if self.elo < MINIMUM_ELO {
            self.elo = MINIMUM_ELO;
        }
    }
}

impl Tournament {
    #[must_use]
    pub const fn default_stats(&self) -> &PlayerStats {
        &self.default_stats
    }

    #[must_use]
    pub fn get_player_or_default_stats(&self, player: u32) -> &PlayerStats {
        self.get_player_stats(player).unwrap_or(&self.default_stats)
    }

    #[must_use]
    pub fn get_player_stats(&self, player: u32) -> Option<&PlayerStats> {
        self.stats.get(&player)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod player_stats {
        use super::*;

        #[test]
        fn loss_with_negative_elo() {
            let mut stats = PlayerStats::new(1500.0);
            stats.add_loss(-500.0);
            assert_relative_eq!(1000.0, stats.elo());
        }

        #[test]
        fn win_with_negative_elo() {
            let mut stats = PlayerStats::new(1500.0);
            stats.add_win(-500.0);
            assert_relative_eq!(2000.0, stats.elo());
        }

        #[test]
        fn wr_only_when_games() {
            let mut stats = PlayerStats::new(1500.0);
            assert!(stats.wr().is_none());
            stats.add_loss(100.0);
            assert!(stats.wr().is_some());
            stats.add_win(100.0);
            assert!(stats.wr().is_some());
        }

        #[test]
        fn add_win_increases_counts() {
            let mut stats = PlayerStats::new(1500.0);
            stats.add_win(500.0);
            assert_eq!(1, stats.games());
            assert_eq!(1, stats.wins());
        }

        #[test]
        fn add_loss_increases_counts() {
            let mut stats = PlayerStats::new(1500.0);
            stats.add_loss(500.0);
            assert_eq!(1, stats.games());
            assert_eq!(0, stats.wins());
        }

        #[test]
        fn add_win_increases_elo() {
            let elo = 2000.0;
            let mut stats = PlayerStats::new(elo);
            stats.add_win(100.0);
            assert_relative_eq!(2100.0, stats.elo());
        }

        #[test]
        fn add_loss_decreases_elo() {
            let elo = 2000.0;
            let mut stats = PlayerStats::new(elo);
            stats.add_loss(100.0);
            assert_relative_eq!(1900.0, stats.elo());
        }

        #[test]
        fn peak_elo_starts_at_elo() {
            const ELO: f64 = 2000.0;
            let stats = PlayerStats::new(ELO);
            assert_relative_eq!(ELO, stats.elo_peak());
        }

        #[test]
        fn peak_elo_increases_with_win() {
            let mut stats = PlayerStats::new(1500.0);
            stats.add_win(500.0);
            assert_relative_eq!(2000.0, stats.elo_peak());
        }

        #[test]
        fn peak_elo_remains_with_loss() {
            let mut stats = PlayerStats::new(1500.0);
            stats.add_loss(400.0);
            assert_relative_eq!(1500.0, stats.elo_peak());
        }

        #[test]
        fn peak_elo_remains_when_winning_below() {
            let mut stats = PlayerStats::new(1500.0);
            stats.add_loss(500.0);
            stats.add_win(400.0);
            assert_relative_eq!(1400.0, stats.elo());
            assert_relative_eq!(1500.0, stats.elo_peak());
        }

        #[test]
        fn elo_has_minimum() {
            assert_relative_eq!(MINIMUM_ELO, PlayerStats::new(0.0).elo());
            let mut stats = PlayerStats::new(1500.0);
            stats.add_loss(1600.0);
            assert_relative_eq!(MINIMUM_ELO, stats.elo());
        }
    }

    #[test]
    fn default_stats_use_starting_elo() {
        let tournament = Tournament::default();
        let starting_elo = tournament.config.starting_elo;
        let stats = tournament.default_stats();
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

    #[test]
    fn get_player_or_default_returns_default() {
        let mut tourn = Tournament::new();
        let id = tourn.register_player("Sampe Player".to_owned()).unwrap();
        let stats = tourn.get_player_or_default_stats(id);
        assert_eq!(tourn.default_stats(), stats);
    }

    #[test]
    fn get_player_or_default_returns_player() {
        let mut tourn = Tournament::new();
        let player_1 = tourn.register_player("1".to_owned()).unwrap();
        let player_2 = tourn.register_player("2".to_owned()).unwrap();
        let player_3 = tourn.register_player("3".to_owned()).unwrap();
        let player_4 = tourn.register_player("4".to_owned()).unwrap();

        tourn
            .register_record(
                tourn
                    .create_match([player_1, player_2, player_3, player_4])
                    .unwrap()
                    .record(player_1)
                    .unwrap(),
            )
            .unwrap();

        let stats = tourn.get_player_or_default_stats(player_1);
        assert_ne!(tourn.default_stats(), stats);
    }
}
