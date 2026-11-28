use crate::{config::game::GameConfig, error::TournamentError, tournament::Tournament};

impl Tournament {
    pub fn set_game_config(&mut self, config: GameConfig) -> Result<(), TournamentError> {
        self.game_config = config;
        self.reload()?;
        Ok(())
    }

    pub fn with_game_config(mut self, config: GameConfig) -> Result<Self, TournamentError> {
        self.set_game_config(config)?;
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn updating_config_updates_stats() {
        let mut tournament = Tournament::generate_tournament(4, 1).unwrap();
        let id = *tournament.players.keys().next().unwrap();
        let elo_start = tournament.get_player_stats(id).unwrap().elo();
        let mut config = tournament.game_config().clone();
        config.initial_elo += 1500.0;
        tournament.set_game_config(config).unwrap();
        let elo_end = tournament.get_player_stats(id).unwrap().elo();
        assert!(elo_start.total_cmp(&elo_end).is_ne());
    }

    #[test]
    fn set_matchmaker_config() {
        let mut tournament = Tournament::new();
        let mut matchmaker = tournament.matchmaker_config().clone();
        matchmaker.min_pool_size += 1;
        tournament.set_matchmaker_config(matchmaker.clone());
        assert_eq!(
            tournament.matchmaker_config().min_pool_size,
            matchmaker.min_pool_size
        );
    }
}
