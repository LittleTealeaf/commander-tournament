use std::collections::HashMap;

use crate::{
    Tournament,
    error::TournamentError,
    player::color::{ColorIdentity, MtgColor},
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PlayerInfo {
    #[serde(rename = "n", alias = "name")]
    name: String,
    #[serde(
        skip_serializing_if = "String::is_empty",
        default,
        rename = "d",
        alias = "description"
    )]
    description: String,
    #[serde(
        skip_serializing_if = "ColorIdentity::is_colorless",
        default = "ColorIdentity::default",
        rename = "i",
        alias = "identity"
    )]
    identity: ColorIdentity,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        rename = "m",
        alias = "moxfield_id"
    )]
    moxfield_id: Option<String>,
}

impl PlayerInfo {
    #[must_use]
    pub const fn new(name: String) -> Self {
        Self {
            name,
            description: String::new(),
            identity: ColorIdentity::COLORLESS,
            moxfield_id: None,
        }
    }

    #[must_use]
    pub const fn name(&self) -> &String {
        &self.name
    }

    #[must_use]
    pub const fn moxfield_id(&self) -> Option<&String> {
        self.moxfield_id.as_ref()
    }

    #[must_use]
    pub fn moxfield_link(&self) -> Option<String> {
        self.moxfield_id
            .as_ref()
            .map(|id| format!("https://moxfield.com/decks/{id}"))
    }

    #[must_use]
    pub fn moxfield_goldfish_link(&self) -> Option<String> {
        self.moxfield_id
            .as_ref()
            .map(|id| format!("https://moxfield.com/decks/{id}/goldfish"))
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    #[must_use]
    pub fn with_description(self, description: String) -> Self {
        Self {
            description,
            ..self
        }
    }

    pub fn clear_moxfield_id(&mut self) {
        self.moxfield_id = None;
    }

    pub fn set_moxfield_id(&mut self, id: String) {
        const PATTERN: &str = "/decks/";
        if let Some(index) = id.find(PATTERN) {
            let start_index = PATTERN.len() + index;
            self.moxfield_id = id[start_index..].split('/').next().map(str::to_owned);
        } else {
            self.moxfield_id = Some(id);
        }
    }

    #[must_use]
    pub fn with_moxfield_id(self, moxfield_id: String) -> Self {
        Self {
            moxfield_id: Some(moxfield_id),
            ..self
        }
    }

    #[must_use]
    pub const fn color_identity(&self) -> &ColorIdentity {
        &self.identity
    }

    #[must_use]
    pub fn with_color_identity(self, identity: ColorIdentity) -> Self {
        Self { identity, ..self }
    }

    pub const fn set_color_identity(&mut self, identity: ColorIdentity) {
        self.identity = identity;
    }

    pub const fn add_color(&mut self, color: MtgColor) {
        self.identity.add_color(color);
    }

    pub const fn remove_color(&mut self, color: MtgColor) {
        self.identity.remove_color(color);
    }

    pub const fn toggle_color(&mut self, color: MtgColor) {
        if self.identity.has_color(color) {
            self.identity.remove_color(color);
        } else {
            self.identity.add_color(color);
        }
    }

    pub fn colors(&self) -> impl Iterator<Item = MtgColor> {
        self.identity.colors()
    }
}

impl Tournament {
    pub fn merge_players_from_tournament(
        &mut self,
        other: &Self,
    ) -> Result<HashMap<u32, u32>, TournamentError> {
        other
            .players()
            .iter()
            .map(|(id, player)| {
                let new_id = self.get_or_register_player_with_info(player.clone())?;
                Ok((*id, new_id))
            })
            .collect()
    }

    pub fn get_or_register_player(&mut self, name: String) -> Result<u32, TournamentError> {
        match self.register_player(name) {
            Ok(id) | Err(TournamentError::PlayerAlreadyRegistered(_, id)) => Ok(id),
            Err(err) => Err(err),
        }
    }

    pub fn get_or_register_player_with_info(
        &mut self,
        info: PlayerInfo,
    ) -> Result<u32, TournamentError> {
        match self.register_player_with_info(info) {
            // TODO: Merge info from other tournament
            Ok(id) | Err(TournamentError::PlayerAlreadyRegistered(_, id)) => Ok(id),
            Err(err) => Err(err),
        }
    }

    #[cfg(feature = "dev")]
    pub fn register_debug_player(&mut self) -> Result<u32, TournamentError> {
        let id = self.players().keys().max().copied().map_or(0, |i| i + 1);
        self.register_player(format!("debug-{id}"))
    }

    pub fn register_player(&mut self, name: String) -> Result<u32, TournamentError> {
        self.register_player_with_info(PlayerInfo::new(name))
    }

    pub fn register_player_with_info(&mut self, info: PlayerInfo) -> Result<u32, TournamentError> {
        if info.name.is_empty() {
            return Err(TournamentError::InvalidPlayerName(info.name));
        }

        if let Some(id) = self.player_names.get(&info.name) {
            return Err(TournamentError::PlayerAlreadyRegistered(info.name, *id));
        }

        let id = self.players.keys().max().map_or(0, |i| i + 1);

        self.player_names.insert(info.name.clone(), id);
        self.players.insert(id, info);

        Ok(id)
    }

    pub fn set_player_info(
        &mut self,
        player: u32,
        info: PlayerInfo,
    ) -> Result<(), TournamentError> {
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
    pub fn get_player_info(&self, id: &u32) -> Option<&PlayerInfo> {
        self.players().get(id)
    }

    #[must_use]
    pub fn get_player_name(&self, id: &u32) -> Option<&String> {
        self.get_player_info(id).map(PlayerInfo::name)
    }
}
