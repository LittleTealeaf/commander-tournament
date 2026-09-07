use crate::player::color::{ColorIdentity, MtgColor};

#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    Default,
    PartialEq,
    Eq,
    Hash,
    getset::Getters,
    getset::CopyGetters,
    getset::Setters,
    getset::WithSetters,
)]
pub struct PlayerInfo {
    #[serde(rename = "n", alias = "name")]
    #[getset(get = "pub", set = "pub", set_with = "pub")]
    name: String,
    #[serde(
        skip_serializing_if = "String::is_empty",
        default,
        rename = "d",
        alias = "description"
    )]
    #[getset(get = "pub", set = "pub", set_with = "pub")]
    description: String,
    #[serde(
        skip_serializing_if = "ColorIdentity::is_colorless",
        default = "ColorIdentity::default",
        rename = "i",
        alias = "identity"
    )]
    #[getset(get_copy = "pub", set = "pub", set_with = "pub")]
    color_identity: ColorIdentity,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        rename = "m",
        alias = "moxfield_id"
    )]
    moxfield_id: Option<String>,
    #[serde(skip_serializing_if = "is_false", default, rename = "pc", alias = "precon")]
    #[getset(get_copy = "pub", set = "pub", set_with = "pub")]
    is_precon: bool,
    #[serde(skip_serializing_if = "is_false", default, rename = "ar", alias = "archived")]
    #[getset(get_copy = "pub", set = "pub", set_with = "pub")]
    is_archived: bool,
}

#[allow(clippy::trivially_copy_pass_by_ref, reason = "Serialization reference")]
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
            color_identity: ColorIdentity::COLORLESS,
            moxfield_id: None,
            is_precon: false,
            is_archived: false,
        }
    }

    #[must_use]
    pub fn display_name(&self) -> String {
        if self.is_precon {
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

    pub const fn add_color(&mut self, color: MtgColor) {
        self.color_identity.add_color(color);
    }

    pub const fn remove_color(&mut self, color: MtgColor) {
        self.color_identity.remove_color(color);
    }

    pub const fn toggle_color(&mut self, color: MtgColor) {
        if self.color_identity.has_color(color) {
            self.color_identity.remove_color(color);
        } else {
            self.color_identity.add_color(color);
        }
    }

    #[must_use]
    pub fn into_name(self) -> String {
        self.name
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
