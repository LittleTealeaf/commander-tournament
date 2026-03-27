use edh_tourn::{
    config::{game::GameConfig, ranking::RankingConfig},
    error::TournamentError,
    game::record::GameRecord,
    player::info::PlayerInfo,
    tournament::Tournament,
};

use crate::{App, effect::Effect, traits::HandleMessage};

#[derive(Clone, Debug)]
pub enum TournamentAction {
    Register(PlayerInfo),
    SetPlayerInfo(u32, PlayerInfo),
    DeletePlayer(u32),
    DeleteGame(usize),
    Record(Box<GameRecord>),
    Reload,
    SetGameConfig(GameConfig),
    SetRankingConfig(RankingConfig),
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
            Self::SetRankingConfig(ranking_config) => {
                tournament.set_ranking_config(ranking_config);
                Ok(())
            }
            Self::DeleteGame(index) => tournament.delete_game(index),
            Self::SetPlayerInfo(id, info) => tournament.set_player_info(id, info),
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
