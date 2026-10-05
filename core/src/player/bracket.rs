#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    derive_more::Display,
    strum::VariantArray,
    serde::Deserialize,
    serde::Serialize,
    Hash,
)]
pub enum Bracket {
    #[display("Bracket 1")]
    #[serde(rename = "1", alias = "Bracket1")]
    Bracket1,
    #[display("Bracket 2")]
    #[serde(rename = "2", alias = "Bracket2")]
    Bracket2,
    #[display("Bracket 3")]
    #[serde(rename = "3", alias = "Bracket3")]
    Bracket3,
    #[display("Bracket 4")]
    #[serde(rename = "4", alias = "Bracket4")]
    Bracket4,
    #[serde(rename = "5", alias = "Bracket5")]
    #[display("Bracket 5")]
    Bracket5,
}

impl Bracket {
    #[must_use]
    pub const fn value(self) -> usize {
        match self {
            Self::Bracket1 => 1,
            Self::Bracket2 => 2,
            Self::Bracket3 => 3,
            Self::Bracket4 => 4,
            Self::Bracket5 => 5,
        }
    }
}
