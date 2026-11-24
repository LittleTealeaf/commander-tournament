#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    strum::VariantArray,
    derive_more::Display,
)]
pub enum Bracket {
    #[serde(rename = "1", alias = "Exhibition")]
    #[display("Bracket 1: Exhibition")]
    Exhibition = 1,
    #[serde(rename = "2", alias = "Core")]
    #[display("Bracket 2: Core")]
    Core = 2,
    #[serde(rename = "3", alias = "Upgraded")]
    #[display("Bracket 3: Upgraded")]
    Upgraded = 3,
    #[serde(rename = "4", alias = "Optimized")]
    #[display("Bracket 4: Optimized")]
    Optimized = 4,
    #[serde(rename = "5", alias = "Cedh")]
    #[display("Bracket 5: cEDH")]
    Cedh = 5,
}

impl Bracket {
    #[must_use]
    pub const fn number(self) -> u8 {
        self as u8
    }

    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Exhibition => "Exhibition",
            Self::Core => "Core",
            Self::Upgraded => "Upgraded",
            Self::Optimized => "Optimized",
            Self::Cedh => "cEDH",
        }
    }

    #[must_use]
    pub const fn short_label(self) -> &'static str {
        match self {
            Self::Exhibition => "B1 · Exhibition",
            Self::Core => "B2 · Core",
            Self::Upgraded => "B3 · Upgraded",
            Self::Optimized => "B4 · Optimized",
            Self::Cedh => "B5 · cEDH",
        }
    }

    #[must_use]
    pub const fn from_number(number: u8) -> Option<Self> {
        match number {
            1 => Some(Self::Exhibition),
            2 => Some(Self::Core),
            3 => Some(Self::Upgraded),
            4 => Some(Self::Optimized),
            5 => Some(Self::Cedh),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use strum::VariantArray;

    use super::*;

    #[test]
    fn bracket_numbers_roundtrip() {
        for (idx, &bracket) in Bracket::VARIANTS.iter().enumerate() {
            let expected = u8::try_from(idx + 1).expect("bracket index fits in u8");
            assert_eq!(bracket.number(), expected);
            assert_eq!(Bracket::from_number(expected), Some(bracket));
        }
        assert_eq!(Bracket::from_number(0), None);
        assert_eq!(Bracket::from_number(6), None);
    }

    #[test]
    fn bracket_ordering() {
        assert!(Bracket::Exhibition < Bracket::Core);
        assert!(Bracket::Core < Bracket::Upgraded);
        assert!(Bracket::Upgraded < Bracket::Optimized);
        assert!(Bracket::Optimized < Bracket::Cedh);
    }
}
