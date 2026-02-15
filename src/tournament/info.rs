use crate::{Tournament, error::TournamentError};

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Copy,
)]
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

impl MtgColor {
    pub const COLORS: [Self; 6] = [
        Self::White,
        Self::Blue,
        Self::Green,
        Self::Red,
        Self::Black,
        Self::Colorless,
    ];

    pub fn letter(&self) -> &str {
        match self {
            MtgColor::White => "W",
            MtgColor::Blue => "B",
            MtgColor::Green => "G",
            MtgColor::Red => "R",
            MtgColor::Black => "B",
            MtgColor::Colorless => "C",
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct PlayerInfo {
    name: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    description: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    colors: Vec<MtgColor>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
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

    pub fn moxfield_id(&self) -> Option<&String> {
        self.moxfield_id.as_ref()
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

    pub fn set_name(&mut self, name: String) {
        self.name = name
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn set_moxfield_id(&mut self, id: Option<String>) {
        self.moxfield_id = id;
    }

    pub fn remove_color(&mut self, color: &MtgColor) {
        for i in 0..self.colors.len() {
            let Some(c) = self.colors.get(i) else {
                continue;
            };
            if c.eq(color) {
                self.colors.remove(i);
                return;
            }
        }
    }

    pub fn add(&mut self, color: MtgColor) {
        if !self.colors.contains(&color) {
            self.colors.push(color);
        }
    }

    pub fn toggle_color(&mut self, color: MtgColor) {
        if self.colors.contains(&color) {
            self.remove_color(&color);
        } else {
            self.colors.push(color);
        }
    }

    pub fn has_color(&self, color: &MtgColor) -> bool {
        self.colors.contains(color)
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

        if !saved_info.name().eq(info.name()) {
            if info.name().is_empty() {
                return Err(TournamentError::InvalidPlayerName(info.name().to_string()));
            }

            if let Some(old_id) = self.player_names.get(info.name()) {
                return Err(TournamentError::PlayerAlreadyRegistered(
                    info.name().to_string(),
                    *old_id,
                ));
            }
        }

        self.player_names.insert(info.name().to_string(), player);
        self.player_names.remove(saved_info.name());

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
