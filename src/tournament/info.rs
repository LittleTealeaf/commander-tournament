use crate::{Tournament, error::TournamentError, stats::PlayerStats};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum MtgColor {
    #[serde(alias = "w")]
    White,
    #[serde(alias = "u")]
    Blue,
    #[serde(alias = "g")]
    Green,
    #[serde(alias = "r")]
    Red,
    #[serde(alias = "b")]
    Black,
    #[serde(alias = "c")]
    Colorless,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PlayerInfo {
    name: String,
    stats: Option<PlayerStats>,
    description: String,
    colors: Vec<MtgColor>,
    moxfield_id: Option<String>,
}

impl PlayerInfo {
    pub(crate) fn new(name: String) -> Self {
        Self {
            name,
            stats: None,
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
        player: usize,
        info: PlayerInfo,
    ) -> Result<(), TournamentError> {
        let player_info = self
            .players
            .get_mut(&player)
            .ok_or(TournamentError::PlayerNotRegistered(player))?;

        info.stats = None;

        *player_info = info;
        Ok(())
    }

    pub fn get_player_info(&self, id: usize) -> Result<PlayerInfo, TournamentError> {
        let info = self
            .players
            .get(id)
            .ok_or(TournamentError::InvalidPlayerId(id))?
            .clone();
        info.stats = self.stats.get(&id).cloned();
        Ok(info)
    }
}
