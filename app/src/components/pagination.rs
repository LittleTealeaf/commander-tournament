use iced::{
    Element, Length,
    alignment::Vertical,
    widget::{button, container, row, space, text},
};
use iced_tea::{Component, Model, Signal};
use nerd_font_symbols::md;

use crate::{
    fonts::FONT_MEDIUM,
    style::{self, COLORS},
};

pub const DEFAULT_PAGE_SIZE: usize = 25;
pub const PAGE_SIZES: [usize; 3] = [25, 50, 100];

#[derive(Debug, Clone, Copy)]
pub enum PaginatorMsg {
    SetPage(usize),
    SetPageSize(usize),
}

#[derive(Debug, Clone, Copy)]
pub struct Paginator {
    page: usize,
    page_size: usize,
    show_size_picker: bool,
}

impl Default for Paginator {
    fn default() -> Self {
        Self::new(DEFAULT_PAGE_SIZE, true)
    }
}

impl Paginator {
    #[must_use]
    pub const fn new(page_size: usize, show_size_picker: bool) -> Self {
        Self {
            page: 0,
            page_size,
            show_size_picker,
        }
    }

    #[must_use]
    pub const fn compact() -> Self {
        Self::new(DEFAULT_PAGE_SIZE, false)
    }

    pub const fn reset(&mut self) {
        self.page = 0;
    }

    #[must_use]
    pub const fn page_size(&self) -> usize {
        if self.page_size == 0 { 1 } else { self.page_size }
    }

    #[must_use]
    pub fn total_pages(&self, total_items: usize) -> usize {
        total_items.div_ceil(self.page_size()).max(1)
    }

    #[must_use]
    pub fn current_page(&self, total_items: usize) -> usize {
        self.page.min(self.total_pages(total_items) - 1)
    }

    /// Returns `(skip_offset, take_count)` for slicing an iterator of `total_items`.
    #[must_use]
    pub fn slice_bounds(&self, total_items: usize) -> (usize, usize) {
        let size = self.page_size();
        (self.current_page(total_items) * size, size)
    }
}

impl Model for Paginator {
    type Message = PaginatorMsg;
    type OutMessage = ();
    type Context<'a> = usize;

    fn update<'a>(
        &'a mut self,
        message: Self::Message,
        _context: Self::Context<'a>,
    ) -> anyhow::Result<Signal<Self::Message, Self::OutMessage>> {
        match message {
            PaginatorMsg::SetPage(page) => {
                self.page = page;
            }
            PaginatorMsg::SetPageSize(size) => {
                self.page_size = size.max(1);
                self.page = 0;
            }
        }
        Signal::done()
    }
}

impl Component for Paginator {
    fn render<'a>(&'a self, context: Self::Context<'a>) -> Element<'a, Self::Message> {
        let total_items = context;
        let page_size = self.page_size();
        let total_pages = self.total_pages(total_items);
        let current_page = self.current_page(total_items);
        let start = current_page * page_size;
        let end = (start + page_size).min(total_items);

        let range_text = text(format!("Showing {}–{end} of {total_items}", start + 1))
            .size(12)
            .color(COLORS.subtext0);

        let can_prev = current_page > 0;
        let can_next = current_page + 1 < total_pages;

        let mut first_btn = button(text(md::MD_PAGE_FIRST).size(16))
            .padding([6, 8])
            .style(style::icon_button);
        let mut prev_btn = button(text(md::MD_CHEVRON_LEFT).size(16))
            .padding([6, 8])
            .style(style::icon_button);
        let mut next_btn = button(text(md::MD_CHEVRON_RIGHT).size(16))
            .padding([6, 8])
            .style(style::icon_button);
        let mut last_btn = button(text(md::MD_PAGE_LAST).size(16))
            .padding([6, 8])
            .style(style::icon_button);

        if can_prev {
            first_btn = first_btn.on_press(PaginatorMsg::SetPage(0));
            prev_btn = prev_btn.on_press(PaginatorMsg::SetPage(current_page - 1));
        }
        if can_next {
            next_btn = next_btn.on_press(PaginatorMsg::SetPage(current_page + 1));
            last_btn = last_btn.on_press(PaginatorMsg::SetPage(total_pages - 1));
        }

        let page_controls = row![
            first_btn,
            prev_btn,
            text(format!("Page {} of {total_pages}", current_page + 1))
                .font(FONT_MEDIUM)
                .size(12)
                .color(COLORS.subtext1),
            next_btn,
            last_btn,
        ]
        .spacing(6)
        .align_y(Vertical::Center);

        let mut bar = row![range_text, space::horizontal()]
            .align_y(Vertical::Center)
            .width(Length::Fill);

        if self.show_size_picker {
            let size_selector = PAGE_SIZES.into_iter().fold(
                row![text("Per page:").size(12).color(COLORS.subtext0)]
                    .spacing(6)
                    .align_y(Vertical::Center),
                |r, size| {
                    r.push(
                        button(text(size.to_string()).size(12))
                            .padding([4, 8])
                            .style(style::segment_button(size == page_size))
                            .on_press(PaginatorMsg::SetPageSize(size)),
                    )
                },
            );
            bar = bar.push(size_selector).push(space::horizontal().width(16));
        }

        container(bar.push(page_controls))
            .width(Length::Fill)
            .padding([10, 14])
            .style(style::card)
            .into()
    }
}
