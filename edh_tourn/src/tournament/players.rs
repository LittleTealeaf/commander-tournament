use std::collections::HashMap;

use crate::{
    error::TournamentError,
    player::{PlayerId, RegisteredPlayer, info::PlayerInfo},
    tournament::Tournament,
};

impl Tournament {
    pub fn update_player_names(&mut self) {
        self.player_names = self
            .players
            .iter()
            .map(|(id, info)| (info.name().to_owned(), *id))
            .collect();
    }

    #[must_use]
    pub const fn players(&self) -> &HashMap<PlayerId, PlayerInfo> {
        &self.players
    }

    #[must_use]
    pub fn is_id_registered(&self, id: &PlayerId) -> bool {
        self.players.contains_key(id)
    }

    pub fn require_id_registered(&self, id: PlayerId) -> Result<(), TournamentError> {
        if self.is_id_registered(&id) {
            Ok(())
        } else {
            Err(TournamentError::InvalidPlayerId(id))
        }
    }

    #[must_use]
    pub fn get_player_id(&self, name: &String) -> Option<PlayerId> {
        self.player_names.get(name).copied()
    }

    #[must_use]
    pub fn get_registered_player(&self, id: PlayerId) -> Option<RegisteredPlayer<'_>> {
        let info = self.get_player_info(&id)?;
        let stats = self.get_player_or_default_stats(id);
        Some(RegisteredPlayer::new(id, info, stats))
    }

    pub fn get_registered_players<I>(&self, ids: I) -> impl Iterator<Item = RegisteredPlayer<'_>>
    where
        I: IntoIterator<Item = PlayerId>,
    {
        ids.into_iter().filter_map(|id| self.get_registered_player(id))
    }

    pub fn registered_players(&self) -> impl Iterator<Item = RegisteredPlayer<'_>> {
        self.players()
            .iter()
            .map(|(id, info)| RegisteredPlayer::new(*id, info, self.get_player_or_default_stats(*id)))
    }

    pub fn unregister_player(&mut self, id: PlayerId) -> Result<(), TournamentError> {
        self.players
            .remove(&id)
            .ok_or(TournamentError::InvalidPlayerId(id))?;
        self.games.retain(|game| !game.has_player(id));
        self.reload()?;
        Ok(())
    }

    pub fn get_or_register_player(&mut self, name: String) -> Result<PlayerId, TournamentError> {
        self.get_player_id(&name)
            .map_or_else(|| self.register_player(name), Ok)
    }

    pub fn merge_players_from_tournament(
        &mut self,
        other: &Self,
    ) -> Result<HashMap<PlayerId, PlayerId>, TournamentError> {
        other
            .players()
            .iter()
            .map(|(id, player)| {
                let new_id = self.update_or_register_player(player.clone())?;
                Ok((*id, new_id))
            })
            .collect()
    }

    pub fn update_or_register_player<I>(&mut self, info: I) -> Result<PlayerId, TournamentError>
    where
        I: Into<PlayerInfo>,
    {
        let info = info.into();
        match self.get_player_id(info.name()) {
            Some(id) => {
                self.set_player_info(id, info)?;
                Ok(id)
            }
            None => self.register_player(info),
        }
    }

    pub fn register_player<I>(&mut self, info: I) -> Result<PlayerId, TournamentError>
    where
        I: Into<PlayerInfo>,
    {
        let info = info.into();
        if info.name().is_empty() {
            return Err(TournamentError::InvalidPlayerName(String::new()));
        }

        if let Some(id) = self.player_names.get(info.name()) {
            return Err(TournamentError::PlayerAlreadyRegistered(info.into_name(), *id));
        }

        let id = self.players.keys().max().map_or(0, |i| i.0 + 1);
        let id = PlayerId(id);

        self.player_names.insert(info.name().to_owned(), id);
        self.players.insert(id, info);

        Ok(id)
    }

    pub fn set_player_info(&mut self, player: PlayerId, info: PlayerInfo) -> Result<(), TournamentError> {
        let saved_info = self
            .players
            .get_mut(&player)
            .ok_or(TournamentError::InvalidPlayerId(player))?;

        if !saved_info.name().eq(info.name()) {
            if info.name().is_empty() {
                return Err(TournamentError::InvalidPlayerName(info.name().to_owned()));
            }

            if let Some(old_id) = self.player_names.get(info.name()) {
                return Err(TournamentError::PlayerAlreadyRegistered(
                    info.name().to_owned(),
                    *old_id,
                ));
            }

            self.player_names.remove(saved_info.name());
            self.player_names.insert(info.name().to_owned(), player);
        }

        *saved_info = info;

        Ok(())
    }

    #[must_use]
    pub fn get_player_info(&self, id: &PlayerId) -> Option<&PlayerInfo> {
        self.players().get(id)
    }

    pub fn update_player_info<F>(&mut self, id: &PlayerId, modifier: F) -> Result<(), TournamentError>
    where
        F: Fn(PlayerInfo) -> PlayerInfo,
    {
        let Some(info) = self.players.get(id) else {
            return Err(TournamentError::InvalidPlayerId(*id));
        };

        self.set_player_info(*id, modifier(info.clone()))
    }

    #[must_use]
    pub fn get_player_name(&self, id: &PlayerId) -> Option<&String> {
        self.get_player_info(id).map(PlayerInfo::name)
    }

    #[must_use]
    pub fn get_player_display_name(&self, id: &PlayerId) -> Option<String> {
        self.get_player_info(id).map(PlayerInfo::display_name)
    }
}
