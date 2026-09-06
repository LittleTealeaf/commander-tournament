use crate::{
    config::{game::GameConfig, matchmaker::MatchmakerConfig},
    error::TournamentError,
    tournament::Tournament,
};

impl Tournament {
    #[must_use]
    pub const fn game_config(&self) -> &GameConfig {
        self.config.game_config()
    }

    pub fn set_game_config(&mut self, config: GameConfig) -> Result<(), TournamentError> {
        self.config.set_game_config(config);
        self.reload()?;
        Ok(())
    }

    pub fn with_game_config(mut self, config: GameConfig) -> Result<Self, TournamentError> {
        self.set_game_config(config)?;
        Ok(self)
    }

    #[must_use]
    pub const fn matchmaker_config(&self) -> &MatchmakerConfig {
        self.config.matchmaker_config()
    }

    pub const fn set_matchmaker_config(&mut self, config: MatchmakerConfig) {
        self.config.set_matchmaker_config(config);
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
        config.starting_elo += 1500.0;
        tournament.set_game_config(config).unwrap();
        let elo_end = tournament.get_player_stats(id).unwrap().elo();
        assert!(elo_start.total_cmp(&elo_end).is_ne());
    }

    #[test]
    fn updating_config_updates_version() {
        let mut tournament = Tournament::generate_tournament(4, 1).unwrap();
        let mut config = tournament.game_config().clone();
        config.starting_elo += 1500.0;

        let version = tournament.snapshot;
        tournament.set_game_config(config).unwrap();
        let new_version = tournament.snapshot;
        assert_eq!(version + 1, new_version);
    }

    #[test]
    fn with_game_config_updates_version() {
        let tournament = Tournament::new();
        let ver = tournament.snapshot;
        let tournament = tournament.with_game_config(GameConfig::default()).unwrap();
        let new_ver = tournament.snapshot;
        assert_eq!(ver + 1, new_ver);
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
