use core::fmt::Display;

use itertools::Itertools;

#[derive(
    Default,
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Copy,
)]
#[serde(transparent)]
pub struct ColorIdentity(pub(crate) u32);

impl ColorIdentity {
    #[must_use]
    pub const fn is_colorless(&self) -> bool {
        self.0 == 0
    }

    pub fn to_colors(&self) -> impl Iterator<Item = MtgColor> {
        MtgColor::COLORS
            .into_iter()
            .filter(|color| self.has_color(*color))
    }

    #[must_use]
    pub const fn has_color(&self, color: MtgColor) -> bool {
        (self.0 & color.to_identity_number()) == color.to_identity_number()
    }

    pub const fn add_color(&mut self, color: MtgColor) {
        self.0 |= color.to_identity_number();
    }

    pub const fn remove_color(&mut self, color: MtgColor) {
        if self.has_color(color) {
            self.0 -= color.to_identity_number();
        }
    }
}

impl Display for ColorIdentity {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self.0 {
                0 => "Colorless",
                1 => "White",
                2 => "Blue",
                3 => "Azorius",
                4 => "Black",
                5 => "Orzhov",
                6 => "Dimir",
                7 => "Esper",
                8 => "Red",
                9 => "Boros",
                10 => "Izzet",
                11 => "Jeskai",
                12 => "Rakdos",
                13 => "Mardu",
                14 => "Grixis",
                15 => "Yore",
                16 => "Green",
                17 => "Selesnya",
                18 => "Simic",
                19 => "Bant",
                20 => "Golgari",
                21 => "Abzan",
                22 => "Sultai",
                23 => "Witch",
                24 => "Gruul",
                25 => "Naya",
                26 => "Temur",
                27 => "Ink",
                28 => "Jund",
                29 => "Dune",
                30 => "Glint",
                31 => "WUBRG",
                _ => "Unknown",
            }
        )
    }
}

impl FromIterator<MtgColor> for ColorIdentity {
    fn from_iter<T: IntoIterator<Item = MtgColor>>(iter: T) -> Self {
        Self(
            iter.into_iter()
                .unique()
                .map(MtgColor::to_identity_number)
                .sum::<u32>(),
        )
    }
}

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
}

impl MtgColor {
    pub const COLORS: [Self; 5] = [Self::White, Self::Blue, Self::Green, Self::Red, Self::Black];

    #[must_use]
    pub const fn letter(&self) -> &'static str {
        match self {
            Self::White => "W",
            Self::Blue => "U",
            Self::Green => "G",
            Self::Red => "R",
            Self::Black => "B",
        }
    }

    const fn to_identity_number(self) -> u32 {
        match self {
            Self::White => 1,
            Self::Blue => 1 << 1,
            Self::Black => 1 << 2,
            Self::Red => 1 << 3,
            Self::Green => 1 << 4,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_conversions() {
        for i in 0..32 {
            let color = ColorIdentity(i);
            let values = color.to_colors().collect_vec();
            let new_color: ColorIdentity = values.into_iter().collect();
            assert_eq!(color, new_color);
        }
    }

    #[test]
    fn add_colors_work() {
        let mut i = ColorIdentity(0);
        i.add_color(MtgColor::White);
        assert!(i.has_color(MtgColor::White));

        i.add_color(MtgColor::White);
        assert!(i.has_color(MtgColor::White));
    }
}
