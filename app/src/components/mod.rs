pub mod cards;
pub mod deck_picker;
pub mod pagination;
pub mod progress;

use commander_tournament_core::player::color::ColorIdentity;
use iced::{
    Element,
    alignment::Vertical,
    widget::{container, row, text},
};

use crate::{fonts::FONT_MEDIUM, icons::ToColorIcons, style};

pub use cards::{empty_card, kpi_card};
pub use deck_picker::{DeckPicker, DeckPickerMsg};
pub use pagination::{DEFAULT_PAGE_SIZE, PAGE_SIZES, Paginator, PaginatorMsg};
pub use progress::progress_bar;

#[must_use]
pub fn pill<'a, M, T>(label: T, accent: catppuccin::Color) -> Element<'a, M>
where
    M: 'a,
    T: text::IntoFragment<'a>,
{
    container(text(label).font(FONT_MEDIUM).size(11))
        .padding([2, 8])
        .style(style::pill_badge(accent))
        .into()
}

#[must_use]
pub fn mana_pips<'a, M: 'a>(identity: ColorIdentity) -> Element<'a, M> {
    identity
        .to_icons()
        .fold(row![].spacing(4).align_y(Vertical::Center), row::Row::push)
        .into()
}
