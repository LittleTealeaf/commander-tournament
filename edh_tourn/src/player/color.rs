use core::fmt::Display;
use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign};

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
pub struct ColorIdentity(u8);

impl ColorIdentity {
    #[must_use]
    pub const fn is_colorless(&self) -> bool {
        self.0 == 0
    }

    pub fn into_colors(self) -> impl Iterator<Item = MtgColor> {
        MtgColor::COLORS
            .map(|color| self.has_color(color).then_some(color))
            .into_iter()
            .flatten()
    }

    #[must_use]
    pub const fn num_colors(&self) -> u32 {
        self.0.count_ones()
    }

    pub fn colors(&self) -> impl Iterator<Item = MtgColor> {
        MtgColor::COLORS
            .into_iter()
            .filter(|color| self.has_color(*color))
    }

    #[must_use]
    pub const fn has_color(&self, color: MtgColor) -> bool {
        (self.0 & (color as u8)) == (color as u8)
    }

    pub const fn add_color(&mut self, color: MtgColor) {
        self.0 |= color as u8;
    }

    pub const fn remove_color(&mut self, color: MtgColor) {
        if self.has_color(color) {
            self.0 -= color as u8;
        }
    }
}

impl ColorIdentity {
    #[allow(clippy::cast_possible_truncation, clippy::indexing_slicing)]
    pub const IDENTITIES: [Self; 32] = {
        let mut arr = [Self(0); 32];
        let mut i = 0;
        while i < 32 {
            arr[i] = Self(i as u8);
            i += 1;
        }
        arr
    };

    pub const COLORLESS: Self = Self(0);
    pub const WHITE: Self = Self(1);
    pub const BLUE: Self = Self(2);
    pub const AZORIUS: Self = Self(3);
    pub const BLACK: Self = Self(4);
    pub const ORZHOV: Self = Self(5);
    pub const DIMIR: Self = Self(6);
    pub const ESPER: Self = Self(7);
    pub const RED: Self = Self(8);
    pub const BOROS: Self = Self(9);
    pub const IZZET: Self = Self(10);
    pub const JESKAI: Self = Self(11);
    pub const RAKDOS: Self = Self(12);
    pub const MARDU: Self = Self(13);
    pub const GRIXIS: Self = Self(14);
    pub const YORE: Self = Self(15);
    pub const GREEN: Self = Self(16);
    pub const SELESNYA: Self = Self(17);
    pub const SIMIC: Self = Self(18);
    pub const BANT: Self = Self(19);
    pub const GOLGARI: Self = Self(20);
    pub const ABZAN: Self = Self(21);
    pub const SULTAI: Self = Self(22);
    pub const WITCH: Self = Self(23);
    pub const GRUUL: Self = Self(24);
    pub const NAYA: Self = Self(25);
    pub const TEMUR: Self = Self(26);
    pub const INK: Self = Self(27);
    pub const JUND: Self = Self(28);
    pub const DUNE: Self = Self(29);
    pub const GLINT: Self = Self(30);
    pub const WUBRG: Self = Self(31);
}

impl Display for ColorIdentity {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self.0 & 0b1111 {
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
            _ => unreachable!("Unreachable"),
        })
    }
}

impl FromIterator<MtgColor> for ColorIdentity {
    fn from_iter<T: IntoIterator<Item = MtgColor>>(iter: T) -> Self {
        Self(
            iter.into_iter()
                .unique()
                .map(|color| color as u8)
                .sum::<u8>(),
        )
    }
}

impl From<MtgColor> for ColorIdentity {
    fn from(value: MtgColor) -> Self {
        Self(value as u8)
    }
}

impl BitAnd for ColorIdentity {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitOr for ColorIdentity {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitAndAssign for ColorIdentity {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitOrAssign for ColorIdentity {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Copy,
)]
pub enum MtgColor {
    #[serde(rename = "w", alias = "White")]
    White = 1,
    #[serde(rename = "u", alias = "Blue")]
    Blue = 2,
    #[serde(rename = "b", alias = "Black")]
    Black = 4,
    #[serde(rename = "r", alias = "Red")]
    Red = 8,
    #[serde(rename = "g", alias = "Green")]
    Green = 16,
}

impl Display for MtgColor {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            Self::White => "White",
            Self::Blue => "Blue",
            Self::Black => "Black",
            Self::Red => "Red",
            Self::Green => "Green",
        })
    }
}

impl MtgColor {
    pub const COLORS: [Self; 5] = [Self::White, Self::Blue, Self::Black, Self::Red, Self::Green];

    #[must_use]
    pub const fn letter(self) -> &'static str {
        match self {
            Self::White => "W",
            Self::Blue => "U",
            Self::Green => "G",
            Self::Red => "R",
            Self::Black => "B",
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn color_conversions() {
        for i in 0..32 {
            let color = ColorIdentity(i);
            let values = color.colors().collect_vec();
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

    #[test]
    fn identities_are_all_colors() {
        for identity in ColorIdentity::IDENTITIES {
            assert_ne!("Unknown", identity.to_string());
        }
    }

    #[test]
    fn identities_are_all_different() {
        let mut seen = HashSet::new();
        for identity in ColorIdentity::IDENTITIES {
            assert!(
                seen.insert(identity),
                "Duplicate identity found: {identity}"
            );
        }
    }

    #[test]
    fn color_matches_identity() {
        for color in MtgColor::COLORS {
            let identity: ColorIdentity = color.into();
            assert!(identity.has_color(color));
        }
    }

    #[test]
    fn color_bitand() {
        let white_blue = ColorIdentity::from_iter([MtgColor::White, MtgColor::Blue]);
        let blue_green = ColorIdentity::from_iter([MtgColor::Blue, MtgColor::Green]);
        let white_green = ColorIdentity::from_iter([MtgColor::White, MtgColor::Green]);

        assert_eq!(white_blue & blue_green, ColorIdentity::from(MtgColor::Blue));
        assert_eq!(
            white_blue & white_green,
            ColorIdentity::from(MtgColor::White)
        );
        assert_eq!(
            blue_green & white_green,
            ColorIdentity::from(MtgColor::Green)
        );
    }

    #[test]
    fn color_bitor() {
        let white_blue = ColorIdentity::from_iter([MtgColor::White, MtgColor::Blue]);
        let blue_green = ColorIdentity::from_iter([MtgColor::Blue, MtgColor::Green]);
        let white_green = ColorIdentity::from_iter([MtgColor::White, MtgColor::Green]);

        assert_eq!(
            white_blue | blue_green,
            ColorIdentity::from_iter([MtgColor::White, MtgColor::Blue, MtgColor::Green])
        );
        assert_eq!(
            white_blue | white_green,
            ColorIdentity::from_iter([MtgColor::White, MtgColor::Blue, MtgColor::Green])
        );
        assert_eq!(
            blue_green | white_green,
            ColorIdentity::from_iter([MtgColor::White, MtgColor::Blue, MtgColor::Green])
        );
    }

    #[test]
    fn color_bitand_assign() {
        let mut white_blue = ColorIdentity::from_iter([MtgColor::White, MtgColor::Blue]);
        let blue_green = ColorIdentity::from_iter([MtgColor::Blue, MtgColor::Green]);

        white_blue &= blue_green;
        assert_eq!(white_blue, ColorIdentity::from(MtgColor::Blue));
    }

    #[test]
    fn color_bitor_assign() {
        let mut white_blue = ColorIdentity::from_iter([MtgColor::White, MtgColor::Blue]);
        let blue_green = ColorIdentity::from_iter([MtgColor::Blue, MtgColor::Green]);

        white_blue |= blue_green;
        assert_eq!(
            white_blue,
            ColorIdentity::from_iter([MtgColor::White, MtgColor::Blue, MtgColor::Green])
        );
    }
}
