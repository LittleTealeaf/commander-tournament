use crate::{
    config::{
        TournamentConfig, game::GameConfig, matchmaker::MatchmakerConfig, ranking::RankingConfig,
    },
    error::TournamentError,
    tournament::Tournament,
};

impl Tournament {
    #[must_use]
    pub const fn game_config(&self) -> &GameConfig {
        &self.config.game
    }

    #[must_use]
    pub const fn ranking_config(&self) -> &RankingConfig {
        &self.config.ranking
    }

    pub fn set_game_config(&mut self, config: GameConfig) -> Result<(), TournamentError> {
        self.config.game = config;
        self.reload()?;
        Ok(())
    }

    pub const fn set_ranking_config(&mut self, config: RankingConfig) {
        self.config.ranking = config;
    }

    pub fn with_game_config(mut self, config: GameConfig) -> Result<Self, TournamentError> {
        self.set_game_config(config)?;
        Ok(self)
    }

    #[must_use]
    pub fn with_ranking_config(self, config: RankingConfig) -> Self {
        Self {
            config: TournamentConfig {
                ranking: config,
                ..self.config
            },
            ..self
        }
    }

    #[must_use]
    pub const fn matchmaker_config(&self) -> &MatchmakerConfig {
        &self.config.matchmaker
    }

    pub const fn set_matchmaker_config(&mut self, config: MatchmakerConfig) {
        self.config.matchmaker = config;
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
}
