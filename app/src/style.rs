use iced::widget::{button, container, text_input};
use iced::{Background, Border, Color, Theme, theme};

const COLORS: catppuccin::FlavorColors = catppuccin::PALETTE.mocha.colors;

pub fn app_style<S>(_state: &S, _theme: &Theme) -> theme::Style {
    theme::Style {
        background_color: COLORS.crust.into(),
        text_color: COLORS.text.into(),
    }
}

// --- Containers / Cards / Panels ---

#[must_use]
pub fn sidebar_container(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(COLORS.mantle.into()),
        text_color: Some(COLORS.text.into()),
        border: Border {
            color: COLORS.surface1.into(),
            width: 1.0,
            radius: 0.0.into(),
        },
        ..Default::default()
    }
}

#[must_use]
pub fn topbar_container(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(COLORS.mantle.into()),
        text_color: Some(COLORS.text.into()),
        border: Border {
            color: COLORS.surface1.into(),
            width: 1.0,
            radius: 0.0.into(),
        },
        ..Default::default()
    }
}

#[must_use]
pub fn card(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(COLORS.surface0.into()),
        text_color: Some(COLORS.text.into()),
        border: Border {
            color: COLORS.surface1.into(),
            width: 1.0,
            radius: 10.0.into(),
        },
        ..Default::default()
    }
}

pub fn pod_seat_card(is_target: bool, is_winner: bool) -> impl Fn(&Theme) -> container::Style {
    move |_theme| {
        let border_color: Color = if is_winner {
            COLORS.green.into()
        } else if is_target {
            COLORS.blue.into()
        } else {
            COLORS.surface2.into()
        };

        let bg: Background = if is_winner {
            COLORS.surface1.into()
        } else {
            COLORS.surface0.into()
        };

        container::Style {
            background: Some(bg),
            text_color: Some(COLORS.text.into()),
            border: Border {
                color: border_color,
                width: if is_winner || is_target { 1.5 } else { 1.0 },
                radius: 8.0.into(),
            },
            ..Default::default()
        }
    }
}

pub fn pill_badge(accent: catppuccin::Color) -> impl Fn(&Theme) -> container::Style {
    move |_theme| {
        let accent_color: Color = accent.into();
        container::Style {
            background: Some(COLORS.surface1.into()),
            text_color: Some(accent_color),
            border: Border {
                color: accent_color,
                width: 1.0,
                radius: 999.0.into(),
            },
            ..Default::default()
        }
    }
}

pub fn table_row(selected: bool) -> impl Fn(&Theme) -> container::Style {
    move |_theme| container::Style {
        background: if selected {
            Some(COLORS.surface2.into())
        } else {
            Some(COLORS.surface0.into())
        },
        text_color: Some(COLORS.text.into()),
        border: Border {
            color: COLORS.surface1.into(),
            width: 1.0,
            radius: 6.0.into(),
        },
        ..Default::default()
    }
}

// --- Buttons ---

pub fn nav_button(active: bool) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let bg: Option<Background> = match (active, status) {
            (true, _) | (false, button::Status::Hovered | button::Status::Pressed) => {
                Some(COLORS.surface1.into())
            }
            _ => None,
        };

        let text_color: Color = if active {
            COLORS.blue.into()
        } else {
            COLORS.subtext1.into()
        };

        button::Style {
            background: bg,
            text_color,
            border: Border {
                color: if active {
                    COLORS.surface2.into()
                } else {
                    Color::TRANSPARENT
                },
                width: 1.0,
                radius: 8.0.into(),
            },
            ..Default::default()
        }
    }
}

#[must_use]
pub fn primary_button(_theme: &Theme, status: button::Status) -> button::Style {
    let bg: Color = match status {
        button::Status::Hovered => COLORS.lavender.into(),
        button::Status::Pressed => COLORS.sapphire.into(),
        button::Status::Disabled => COLORS.surface1.into(),
        _ => COLORS.blue.into(),
    };

    let text_color: Color = match status {
        button::Status::Disabled => COLORS.subtext0.into(),
        _ => COLORS.crust.into(),
    };

    button::Style {
        background: Some(bg.into()),
        text_color,
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 8.0.into(),
        },
        ..Default::default()
    }
}

#[must_use]
pub fn secondary_button(_theme: &Theme, status: button::Status) -> button::Style {
    let bg: Option<Background> = match status {
        button::Status::Hovered | button::Status::Pressed => Some(COLORS.surface1.into()),
        _ => Some(COLORS.surface0.into()),
    };

    button::Style {
        background: bg,
        text_color: COLORS.text.into(),
        border: Border {
            color: COLORS.surface2.into(),
            width: 1.0,
            radius: 8.0.into(),
        },
        ..Default::default()
    }
}

#[must_use]
pub fn danger_button(_theme: &Theme, status: button::Status) -> button::Style {
    let is_active = matches!(status, button::Status::Hovered | button::Status::Pressed);

    let bg: Color = if is_active {
        COLORS.red.into()
    } else {
        COLORS.surface1.into()
    };

    let text_color: Color = if is_active {
        COLORS.crust.into()
    } else {
        COLORS.red.into()
    };

    button::Style {
        background: Some(bg.into()),
        text_color,
        border: Border {
            color: COLORS.red.into(),
            width: 1.0,
            radius: 8.0.into(),
        },
        ..Default::default()
    }
}

pub fn segment_button(selected: bool) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let bg: Option<Background> = if selected {
            Some(COLORS.blue.into())
        } else if matches!(status, button::Status::Hovered) {
            Some(COLORS.surface1.into())
        } else {
            Some(COLORS.surface0.into())
        };

        let fg: Color = if selected {
            COLORS.crust.into()
        } else {
            COLORS.subtext1.into()
        };

        button::Style {
            background: bg,
            text_color: fg,
            border: Border {
                color: if selected {
                    COLORS.blue.into()
                } else {
                    COLORS.surface2.into()
                },
                width: 1.0,
                radius: 6.0.into(),
            },
            ..Default::default()
        }
    }
}

pub fn icon_button(_theme: &Theme, status: button::Status) -> button::Style {
    let bg: Option<Background> = match status {
        button::Status::Hovered => Some(COLORS.surface0.into()),
        button::Status::Pressed => Some(COLORS.surface1.into()),
        button::Status::Active | button::Status::Disabled => None,
    };

    let text_color: Color = match status {
        button::Status::Hovered | button::Status::Pressed => COLORS.text.into(),
        button::Status::Active => COLORS.subtext1.into(),
        button::Status::Disabled => COLORS.overlay0.into(),
    };

    button::Style {
        background: bg,
        text_color,
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 6.0.into(),
        },
        ..Default::default()
    }
}

// --- Inputs ---

#[must_use]
pub fn search_input(_theme: &Theme, status: text_input::Status) -> text_input::Style {
    let border_color: Color = match status {
        text_input::Status::Focused { .. } => COLORS.blue.into(),
        text_input::Status::Hovered => COLORS.surface2.into(),
        _ => COLORS.surface1.into(),
    };

    text_input::Style {
        background: COLORS.surface0.into(),
        border: Border {
            color: border_color,
            width: 1.0,
            radius: 8.0.into(),
        },
        icon: COLORS.subtext0.into(),
        placeholder: COLORS.overlay0.into(),
        value: COLORS.text.into(),
        selection: COLORS.surface2.into(),
    }
}
