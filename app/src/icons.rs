use core::iter::once;

use edh_tourn::player::color::{ColorIdentity, MtgColor};
use iced::widget::{Svg, svg::Handle};

const BYTES_W: &[u8] = include_bytes!("../assets/svg/W.svg");
const BYTES_U: &[u8] = include_bytes!("../assets/svg/U.svg");
const BYTES_B: &[u8] = include_bytes!("../assets/svg/B.svg");
const BYTES_R: &[u8] = include_bytes!("../assets/svg/R.svg");
const BYTES_G: &[u8] = include_bytes!("../assets/svg/G.svg");
const BYTES_C: &[u8] = include_bytes!("../assets/svg/C.svg");

#[must_use]
pub fn color_icon(color: MtgColor) -> Svg<'static> {
    Svg::new(match color {
        MtgColor::White => Handle::from_memory(BYTES_W),
        MtgColor::Blue => Handle::from_memory(BYTES_U),
        MtgColor::Black => Handle::from_memory(BYTES_B),
        MtgColor::Red => Handle::from_memory(BYTES_R),
        MtgColor::Green => Handle::from_memory(BYTES_G),
    })
    .height(15)
    .width(15)
}

#[must_use]
pub fn colorless_icon() -> Svg<'static> {
    Svg::new(Handle::from_memory(BYTES_C)).width(15).height(15)
}

pub trait ToColorIcons {
    fn to_icons(&self) -> impl Iterator<Item = Svg<'static>>;
}

impl ToColorIcons for MtgColor {
    fn to_icons(&self) -> impl Iterator<Item = Svg<'static>> {
        once(color_icon(*self))
    }
}

impl ToColorIcons for ColorIdentity {
    fn to_icons(&self) -> impl Iterator<Item = Svg<'static>> {
        self.colors()
            .map(color_icon)
            .chain(self.is_colorless().then(colorless_icon))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_duplicate_colors() {
        let icons = [BYTES_U, BYTES_R, BYTES_G, BYTES_W, BYTES_C, BYTES_B];
        for a in icons {
            let mut count = 0;
            for b in icons {
                if a == b {
                    count += 1;
                }
            }
            assert_eq!(count, 1, "Duplicate Icons Exist");
        }
    }
}
