use core::fmt::Display;
use core::ops::{Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, Sub, SubAssign};

use itertools::Itertools;

#[derive(
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
    derive_more::Display,
)]
pub enum MtgColor {
    #[serde(rename = "w", alias = "White")]
    White = 1 << 0,
    #[serde(rename = "u", alias = "Blue")]
    Blue = 1 << 1,
    #[serde(rename = "b", alias = "Black")]
    Black = 1 << 2,
    #[serde(rename = "r", alias = "Red")]
    Red = 1 << 3,
    #[serde(rename = "g", alias = "Green")]
    Green = 1 << 4,
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

#[derive(
    Default, Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Copy,
)]
#[serde(transparent)]
pub struct ColorIdentity(u8);

impl ColorIdentity {
    #[must_use]
    pub const fn is_colorless(&self) -> bool {
        self.0 == 0
    }

    #[must_use]
    pub const fn num_colors(&self) -> u32 {
        self.0.count_ones()
    }

    pub fn colors(self) -> impl Iterator<Item = MtgColor> {
        MtgColor::COLORS
            .map(|color| self.has_color(color).then_some(color))
            .into_iter()
            .flatten()
    }

    pub const fn contains(&self, other: &Self) -> bool {
        (self.0 & other.0) == other.0
    }

    #[must_use]
    pub const fn has_color(&self, color: MtgColor) -> bool {
        (self.0 & (color as u8)) == (color as u8)
    }

    pub const fn add_color(&mut self, color: MtgColor) {
        self.0 |= color as u8;
    }

    pub const fn remove_color(&mut self, color: MtgColor) {
        self.add_color(color);
        self.0 -= color as u8;
    }
}

macro_rules! identity_const {
    // 1. The Entry Point: Accepts the starting ID and the list of colors
    (
        $start_id:expr, [$($color:ident),+]
    ) => {
        // Generate the individual constants
        identity_const!(@expand $start_id, $($color),+);

        // Generate the aggregate array
        pub const IDENTITIES: [Self; identity_const!(@count $($color),+)] = [
            $(Self::$color),+
        ];
    };

    // 2. Helper: Recursive expansion for individual constants
    (@expand $id:expr, $color:ident) => {
        pub const $color: Self = Self($id);
    };
    (@expand $id:expr, $color:ident, $($rest:ident),+) => {
        pub const $color: Self = Self($id);
        identity_const!(@expand $id + 1, $($rest),+);
    };

    // 3. Helper: Purely for counting the number of elements for the array size
    (@count $t1:ident $(, $t:ident)*) => {
        1 $(+ identity_const!(@count $t))*
    };
    (@count) => { 0 };
}

impl ColorIdentity {
    identity_const!(
        0,
        [
            COLORLESS, WHITE, BLUE, AZORIUS, BLACK, ORZHOV, DIMIR, ESPER, RED, BOROS, IZZET, JESKAI, RAKDOS,
            MARDU, GRIXIS, YORE, GREEN, SELESNYA, SIMIC, BANT, GOLGARI, ABZAN, SULTAI, WITCH, GRUUL, NAYA,
            TEMUR, INK, JUND, DUNE, GLINT, WUBRG
        ]
    );
}

impl Display for ColorIdentity {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self.0 {
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
        Self(iter.into_iter().unique().map(|color| color as u8).sum::<u8>())
    }
}

impl From<MtgColor> for ColorIdentity {
    fn from(value: MtgColor) -> Self {
        Self(value as u8)
    }
}

impl Add<MtgColor> for ColorIdentity {
    type Output = Self;
    #[allow(clippy::suspicious_arithmetic_impl)]
    fn add(self, rhs: MtgColor) -> Self::Output {
        Self(self.0 | rhs as u8)
    }
}

impl AddAssign<MtgColor> for ColorIdentity {
    #[allow(clippy::suspicious_op_assign_impl)]
    fn add_assign(&mut self, rhs: MtgColor) {
        self.add_color(rhs);
    }
}

impl Sub<MtgColor> for ColorIdentity {
    type Output = Self;
    fn sub(self, rhs: MtgColor) -> Self::Output {
        Self((self.0 | rhs as u8) - rhs as u8)
    }
}

impl SubAssign<MtgColor> for ColorIdentity {
    fn sub_assign(&mut self, rhs: MtgColor) {
        self.remove_color(rhs);
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

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_colors {
        ($($color:ident),+; $id: ident) => {
            {
                let identity = crate::player::color::ColorIdentity::$id;
                let identity_colors = [$(crate::player::color::MtgColor::$color),+];
                assert!(crate::player::color::ColorIdentity::IDENTITIES.contains(&identity), "Expected IDENTITIES const to include {identity}");
                assert_eq!(identity, identity_colors.clone().into_iter().collect());
                assert_eq!(identity, identity_colors.into_iter().rev().collect());

                let colors = identity.colors().collect::<Vec<_>>();
                $(
                    {
                        let color = crate::player::color::MtgColor::$color;
                        assert!(colors.contains(&color), "Expected {color} to be found in {identity}.colors()");
                        assert!(identity.has_color(color), "Expected {identity} to return `true` for has_color({color})");
                    }
                )+
                for color in crate::player::color::MtgColor::COLORS {
                    if !identity_colors.contains(&color) {
                        assert!(!identity.has_color(color), "Expected {identity} to return `false` for has_color({color})");
                    }
                }
            }
        };
    }

    #[test]
    fn color_to_identity_conversion() {
        test_colors!(Red; RED);
        test_colors!(White; WHITE);
        test_colors!(Blue; BLUE);
        test_colors!(Black; BLACK);
        test_colors!(Green; GREEN);
        test_colors!(White, Blue; AZORIUS);
        test_colors!(Blue, Black; DIMIR);
        test_colors!(Black, Red; RAKDOS);
        test_colors!(Red, Green; GRUUL);
        test_colors!(Green, White; SELESNYA);
        test_colors!(White, Black; ORZHOV);
        test_colors!(Blue, Red; IZZET);
        test_colors!(Black, Green; GOLGARI);
        test_colors!(Red, White; BOROS);
        test_colors!(Blue, Green; SIMIC);
        test_colors!(White, Green, Blue; BANT);
        test_colors!(White, Black, Blue; ESPER);
        test_colors!(Red, Black, Blue; GRIXIS);
        test_colors!(Red, Black, Green; JUND);
        test_colors!(Red, White, Green; NAYA);
        test_colors!(White, Black, Green; ABZAN);
        test_colors!(Black, Green, Blue; SULTAI);
        test_colors!(Green, Blue, Red; TEMUR);
        test_colors!(Blue, Red, White; JESKAI);
        test_colors!(Red, White, Black; MARDU);
        test_colors!(White, Blue, Black, Red; YORE);
        test_colors!(Green, Blue, Black, Red; GLINT);
        test_colors!(Green, White, Black, Red; DUNE);
        test_colors!(Green, White, Blue, Red; INK);
        test_colors!(Green, White, Blue, Black; WITCH);
        test_colors!(White, Blue, Black, Red, Green; WUBRG);
    }

    #[test]
    fn colors_has_all_colors() {
        let colors = MtgColor::COLORS;
        assert!(colors.contains(&MtgColor::Red));
        assert!(colors.contains(&MtgColor::Blue));
        assert!(colors.contains(&MtgColor::Black));
        assert!(colors.contains(&MtgColor::Green));
        assert!(colors.contains(&MtgColor::White));
    }

    #[test]
    fn identity_color_counts() {
        for identity in ColorIdentity::IDENTITIES {
            let colors = identity.colors();
            let count = colors.map(|_| 1).sum::<u32>();
            let num_colors = identity.num_colors();
            assert_eq!(
                count, num_colors,
                "{identity} returns {num_colors} colors, expected {count}",
            );
        }
    }

    #[test]
    fn add_assigns_to_identity() {
        for identity in ColorIdentity::IDENTITIES {
            let colors = identity.colors().collect::<Vec<_>>();
            let mut i = ColorIdentity(0);
            for color in colors {
                i += color;
            }
            assert_eq!(identity, i);
        }
    }

    #[test]
    fn adds_to_identity() {
        for identity in ColorIdentity::IDENTITIES {
            let colors = identity.colors().collect::<Vec<_>>();
            let ident = colors.into_iter().fold(ColorIdentity(0), |a, b| a + b);
            assert_eq!(identity, ident);
        }
    }

    #[test]
    fn identities_to_string_not_null() {
        for identity in ColorIdentity::IDENTITIES {
            assert_ne!("Unknown", identity.to_string());
        }
    }

    #[test]
    fn add_color() {
        let mut ident = ColorIdentity::WHITE;
        ident.add_color(MtgColor::Blue);
        assert_eq!(ColorIdentity::AZORIUS, ident);
        ident.add_color(MtgColor::Blue);
        assert_eq!(ColorIdentity::AZORIUS, ident);
    }

    #[test]
    fn add_assign_color() {
        let mut ident = ColorIdentity::WHITE;
        ident += MtgColor::Blue;
        assert_eq!(ColorIdentity::AZORIUS, ident);
        ident += MtgColor::Blue;
        assert_eq!(ColorIdentity::AZORIUS, ident);
    }

    #[test]
    fn remove_color() {
        let mut color = ColorIdentity::JESKAI;
        color.remove_color(MtgColor::Blue);
        assert_eq!(ColorIdentity::BOROS, color);
        color.remove_color(MtgColor::Blue);
        assert_eq!(ColorIdentity::BOROS, color);
    }

    #[test]
    fn sub_color() {
        let ident = ColorIdentity::JESKAI;
        let ident_1 = ident - MtgColor::Blue;
        assert_eq!(ident_1, ColorIdentity::BOROS);
        let ident_2 = ident - MtgColor::Blue;
        assert_eq!(ident_2, ColorIdentity::BOROS);
    }

    #[test]
    fn sub_assign_color() {
        let mut color = ColorIdentity::JESKAI;
        color -= MtgColor::Blue;
        assert_eq!(ColorIdentity::BOROS, color);
        color -= MtgColor::Blue;
        assert_eq!(ColorIdentity::BOROS, color);
    }

    #[test]
    fn color_bitand() {
        let white_blue = ColorIdentity::from_iter([MtgColor::White, MtgColor::Blue]);
        let blue_green = ColorIdentity::from_iter([MtgColor::Blue, MtgColor::Green]);
        let white_green = ColorIdentity::from_iter([MtgColor::White, MtgColor::Green]);

        assert_eq!(white_blue & blue_green, ColorIdentity::from(MtgColor::Blue));
        assert_eq!(white_blue & white_green, ColorIdentity::from(MtgColor::White));
        assert_eq!(blue_green & white_green, ColorIdentity::from(MtgColor::Green));
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

    #[test]
    #[should_panic(expected = "internal error: entered unreachable code: Unreachable")]
    fn out_of_bounds_unreachable() {
        let identity = ColorIdentity(32);
        let string = format!("{identity}");
        println!("{string}");
    }

    #[test]
    fn color_to_letter() {
        let tests = [
            (MtgColor::White, "W"),
            (MtgColor::Blue, "U"),
            (MtgColor::Green, "G"),
            (MtgColor::Red, "R"),
            (MtgColor::Black, "B"),
        ];

        for (color, letter) in tests {
            assert_eq!(color.letter(), letter);
        }
    }
}
