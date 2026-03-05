
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
    pub(crate) const fn new(elo: f64) -> Self {
        Self {
            elo,
            games: 0,
            wins: 0,
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
    use crate::Tournament;

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
