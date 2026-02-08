use crate::{Tournament, error::TournamentError};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum MtgColor {
    #[serde(rename = "w")]
    White,
    #[serde(rename = "u")]
    Blue,
    #[serde(rename = "g")]
    Green,
    #[serde(rename = "r")]
    Red,
    #[serde(rename = "b")]
    Black,
    #[serde(rename = "c")]
    Colorless,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PlayerInfo {
    name: String,
    description: String,
    colors: Vec<MtgColor>,
    moxfield_id: Option<String>,
}

impl PlayerInfo {
    pub(crate) fn new(name: String) -> Self {
        Self {
            name,
            description: String::new(),
            colors: Vec::new(),
            moxfield_id: None,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn moxfield_link(&self) -> Option<String> {
        self.moxfield_id
            .as_ref()
            .map(|id| format!("https://moxfield.com/decks/{id}"))
    }

    pub fn moxfield_goldfish_link(&self) -> Option<String> {
        self.moxfield_id
            .as_ref()
            .map(|id| format!("https://moxfield.com/decks/{id}/goldfish"))
    }
}

impl Tournament {
    pub fn set_player_info(
        &mut self,
        player: u32,
        info: PlayerInfo,
    ) -> Result<(), TournamentError> {
        let saved_info = self
            .players
            .get_mut(&player)
            .ok_or(TournamentError::InvalidPlayerId(player))?;
        *saved_info = info;

        Ok(())
    }

    pub fn get_player_info(&self, id: u32) -> Result<PlayerInfo, TournamentError> {
        Ok(self
            .players
            .get(&id)
            .ok_or(TournamentError::InvalidPlayerId(id))?
            .clone())
    }
}
