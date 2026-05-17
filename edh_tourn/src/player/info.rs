use crate::player::color::{ColorIdentity, MtgColor};

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
    #[serde(skip_serializing_if = "is_false", default, rename = "pc", alias = "precon")]
    precon: bool,
}

#[allow(clippy::trivially_copy_pass_by_ref)]
const fn is_false(b: &bool) -> bool {
    !(*b)
}

fn convert_moxfield_id(id: String) -> Option<String> {
    const PATTERN: &str = "/decks/";
    if let Some(index) = id.find(PATTERN) {
        let start_index = PATTERN.len() + index;
        id[start_index..].split('/').next().map(str::to_owned)
    } else {
        Some(id)
    }
}

impl PlayerInfo {
    #[must_use]
    pub const fn new(name: String) -> Self {
        Self {
            name,
            description: String::new(),
            identity: ColorIdentity::COLORLESS,
            moxfield_id: None,
            precon: false,
        }
    }

    #[must_use]
    pub const fn name(&self) -> &String {
        &self.name
    }

    #[must_use]
    pub fn display_name(&self) -> String {
        if self.precon {
            format!("{} (Precon)", self.name)
        } else {
            self.name.clone()
        }
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
        Self { description, ..self }
    }

    pub fn clear_moxfield_id(&mut self) {
        self.moxfield_id = None;
    }

    pub fn set_moxfield_id(&mut self, id: String) {
        self.moxfield_id = convert_moxfield_id(id);
    }

    #[must_use]
    pub fn with_moxfield_id(self, moxfield_id: String) -> Self {
        Self {
            moxfield_id: convert_moxfield_id(moxfield_id),
            ..self
        }
    }

    #[must_use]
    pub const fn color_identity(&self) -> ColorIdentity {
        self.identity
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

    #[must_use]
    pub fn into_name(self) -> String {
        self.name
    }

    #[must_use]
    pub const fn is_precon(&self) -> bool {
        self.precon
    }

    pub const fn set_precon(&mut self, precon: bool) {
        self.precon = precon;
    }
}

impl From<String> for PlayerInfo {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for PlayerInfo {
    fn from(value: &str) -> Self {
        Self::new(value.to_owned())
    }
}
