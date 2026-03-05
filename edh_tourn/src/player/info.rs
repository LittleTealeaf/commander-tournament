use crate::{Tournament, error::TournamentError};

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Copy,
)]
pub enum MtgColor {
    #[serde(rename = "w", alias = "White")]
    White,
    #[serde(rename = "u", alias = "Blue")]
    Blue,
    #[serde(rename = "g", alias = "Green")]
    Green,
    #[serde(rename = "r", alias = "Red")]
    Red,
    #[serde(rename = "b", alias = "Black")]
    Black,
    #[serde(rename = "c", alias = "Colorless")]
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

    #[must_use]
    pub const fn letter(&self) -> &'static str {
        match self {
            Self::White => "W",
            Self::Blue => "U",
            Self::Green => "G",
            Self::Red => "R",
            Self::Black => "B",
            Self::Colorless => "C",
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, PartialEq, Eq)]
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
    pub(crate) const fn new(name: String) -> Self {
        Self {
            name,
            description: String::new(),
            colors: Vec::new(),
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

    pub fn add_color(&mut self, color: MtgColor) {
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

    #[must_use]
    pub fn has_color(&self, color: &MtgColor) -> bool {
        self.colors.contains(color)
    }

    #[must_use]
    pub const fn colors(&self) -> &Vec<MtgColor> {
        &self.colors
    }
}

impl Tournament {
    pub fn get_or_register_player(&mut self, name: String) -> Result<u32, TournamentError> {
        match self.register_player(name) {
            Ok(id) | Err(TournamentError::PlayerAlreadyRegistered(_, id)) => Ok(id),
            Err(err) => Err(err),
        }
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

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::{Tournament, error::TournamentError, player::info::PlayerInfo};

    #[test]
    fn set_info_invalid_id() {
        let mut t = Tournament::new();
        assert!(!t.players().keys().contains(&283));
        assert!(matches!(
            t.set_player_info(283, PlayerInfo::new(String::new())),
            Err(TournamentError::InvalidPlayerId(283))
        ));
    }

    #[test]
    fn set_info_duplicate_name() {
        let mut t = Tournament::new();
        let name = "Test".to_owned();
        let _ = t.register_player(name.clone()).unwrap();
        let id_2 = t.register_player("Test 2".to_owned()).unwrap();
        assert!(t.set_player_info(id_2, PlayerInfo::new(name)).is_err());
    }

    #[test]
    fn set_info_invalid_name() {
        let mut t = Tournament::new();
        let id = t.register_player(String::from("hi")).unwrap();
        let res = t.set_player_info(id, PlayerInfo::new(String::new()));
        assert!(matches!(res, Err(TournamentError::InvalidPlayerName(_))));
    }

    #[test]
    fn register_invalid_name() {
        let mut t = Tournament::new();

        let res = t.register_player(String::new());
        assert!(matches!(res, Err(TournamentError::InvalidPlayerName(_))));
    }

    #[test]
    fn register_duplicate_name() {
        let mut t = Tournament::new();
        let s = "hi".to_owned();
        let _ = t.register_player(s.clone()).unwrap();
        let res = t.register_player(s);
        assert!(matches!(
            res,
            Err(TournamentError::PlayerAlreadyRegistered(_, _))
        ));
    }
}
