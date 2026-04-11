use edh_tourn::{
    config::{game::GameConfig, matchmaker::MatchmakerConfig},
    error::TournamentError,
    game::record::GameRecord,
    player::{PlayerId, info::PlayerInfo},
    tournament::Tournament,
};

use crate::{App, effect::Effect, traits::HandleMessage};

#[derive(Clone, Debug)]
pub enum TournamentAction {
    Register(PlayerInfo),
    SetPlayerInfo(PlayerId, PlayerInfo),
    DeletePlayer(PlayerId),
    DeleteGame(usize),
    Record(Box<GameRecord>),
    Reload,
    SetGameConfig(GameConfig),
    SetMatchmakerConfig(MatchmakerConfig),
}

impl TournamentAction {
    pub fn apply(self, tournament: &mut Tournament) -> Result<(), TournamentError> {
        match self {
            Self::Register(player_info) => {
                tournament.register_player_with_info(player_info)?;
                Ok(())
            }
            Self::DeletePlayer(id) => tournament.unregister_player(id),
            Self::Record(game_record) => tournament.register_record(*game_record),
            Self::Reload => tournament.reload(),
            Self::SetGameConfig(game_config) => tournament.set_game_config(game_config),
            Self::DeleteGame(index) => tournament.delete_game(index),
            Self::SetPlayerInfo(id, info) => tournament.set_player_info(id, info),
            Self::SetMatchmakerConfig(config) => {
                tournament.set_matchmaker_config(config);
                Ok(())
            }
        }
    }
}

impl HandleMessage<TournamentAction> for App {
    fn handle_message(
        &mut self,
        message: TournamentAction,
        (): Self::UpdateContext<'_>,
    ) -> anyhow::Result<Effect<Self::Message, Self::OutMessage>> {
        message.apply(&mut self.tournament)?;
        self.modified = true;
        Effect::done()
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use edh_tourn::game::entry::GameEntry;

    use super::*;

    #[test]
    fn register() {
        let mut tournament = Tournament::new();
        let info = PlayerInfo::new("Hello".to_owned());
        TournamentAction::Register(info.clone())
            .apply(&mut tournament)
            .unwrap();
        let _ = tournament.get_player_id(info.name()).unwrap();
    }

    #[test]
    fn set_player_info() {
        let mut tournament = Tournament::new();
        let id = tournament.register_debug_player().unwrap();
        let name = tournament.get_player_name(&id).unwrap();
        let new_name = format!("new-{name}");
        let info = PlayerInfo::new(new_name.clone());
        TournamentAction::SetPlayerInfo(id, info)
            .apply(&mut tournament)
            .unwrap();
        assert_eq!(&new_name, tournament.get_player_name(&id).unwrap());

        TournamentAction::SetPlayerInfo(id, PlayerInfo::new(String::new()))
            .apply(&mut tournament)
            .unwrap_err();
    }

    #[test]
    fn delete_player() {
        let mut tournament = Tournament::generate_tournament(50, 50).unwrap();
        // Gets an id of a player that is in at least one game
        let id = tournament
            .games()
            .iter()
            .map(GameRecord::winner)
            .next()
            .unwrap();

        TournamentAction::DeletePlayer(id)
            .apply(&mut tournament)
            .unwrap();

        // Player does not exist
        assert!(tournament.get_registered_player(id).is_none());

        // No games have the player
        for game in tournament.games() {
            assert!(!game.has_player(id));
        }
    }

    #[test]
    fn delete_game() {
        let mut tournament = Tournament::generate_tournament(20, 100).unwrap();
        let id = 5;
        let next_record = GameEntry::from(tournament.games().get(id + 1).unwrap().clone());
        TournamentAction::DeleteGame(id)
            .apply(&mut tournament)
            .unwrap();
        let record = GameEntry::from(tournament.games().get(id).unwrap().clone());
        assert_eq!(record, next_record);
        assert_eq!(99, tournament.games().len());
    }

    #[test]
    fn record_game() {
        let mut tournament = Tournament::generate_tournament(20, 50).unwrap();
        let game = tournament.random_game().unwrap();
        let matchup = tournament.create_match(*game.players()).unwrap();
        let record = matchup.record(game.winner()).unwrap();

        TournamentAction::Record(Box::new(record))
            .apply(&mut tournament)
            .unwrap();
        assert_eq!(51, tournament.games().len());
    }

    #[test]
    fn reload() {
        let mut tournament = Tournament::generate_tournament(20, 20).unwrap();
        let initial = tournament.snapshot();
        TournamentAction::Reload.apply(&mut tournament).unwrap();
        assert_eq!(initial + 1, tournament.snapshot());
    }

    #[test]
    fn set_game_config() {
        let mut tournament = Tournament::new();
        let id = tournament.register_debug_player().unwrap();
        let starting_elo = tournament.get_player_or_default_stats(id).elo();
        let mut config = tournament.game_config().clone();
        config.starting_elo += 1500.0;
        TournamentAction::SetGameConfig(config)
            .apply(&mut tournament)
            .unwrap();
        let ending_elo = tournament.get_player_or_default_stats(id).elo();
        assert_relative_eq!(starting_elo + 1500.0, ending_elo);
    }
}
