use iced::{
    Font,
    font::{Family, Stretch, Style, Weight},
};

pub const JETBRAINS_MONO: &str = "JetBrains Mono";

pub const FONT_BYTES: [&[u8]; 16] = [
    include_bytes!("../assets/fonts/JetBrainsMono-Bold.ttf"),
    include_bytes!("../assets/fonts/JetBrainsMono-BoldItalic.ttf"),
    include_bytes!("../assets/fonts/JetBrainsMono-ExtraBold.ttf"),
    include_bytes!("../assets/fonts/JetBrainsMono-ExtraBoldItalic.ttf"),
    include_bytes!("../assets/fonts/JetBrainsMono-ExtraLight.ttf"),
    include_bytes!("../assets/fonts/JetBrainsMono-ExtraLightItalic.ttf"),
    include_bytes!("../assets/fonts/JetBrainsMono-Italic.ttf"),
    include_bytes!("../assets/fonts/JetBrainsMono-Light.ttf"),
    include_bytes!("../assets/fonts/JetBrainsMono-LightItalic.ttf"),
    include_bytes!("../assets/fonts/JetBrainsMono-Medium.ttf"),
    include_bytes!("../assets/fonts/JetBrainsMono-MediumItalic.ttf"),
    include_bytes!("../assets/fonts/JetBrainsMono-Regular.ttf"),
    include_bytes!("../assets/fonts/JetBrainsMono-SemiBold.ttf"),
    include_bytes!("../assets/fonts/JetBrainsMono-SemiBoldItalic.ttf"),
    include_bytes!("../assets/fonts/JetBrainsMono-Thin.ttf"),
    include_bytes!("../assets/fonts/JetBrainsMono-ThinItalic.ttf"),
];

#[must_use]
pub const fn font_default() -> Font {
    Font::with_name(JETBRAINS_MONO)
}

pub trait FontBuilder: Sized {
    #[must_use]
    fn family(self, family: Family) -> Self;
    #[must_use]
    fn weight(self, weight: Weight) -> Self;
    #[must_use]
    fn stretch(self, stretch: Stretch) -> Self;
    #[must_use]
    fn style(self, style: Style) -> Self;
}

impl FontBuilder for Font {
    fn style(self, style: Style) -> Self {
        Self { style, ..self }
    }

    fn family(self, family: Family) -> Self {
        Self { family, ..self }
    }

    fn weight(self, weight: Weight) -> Self {
        Self { weight, ..self }
    }

    fn stretch(self, stretch: Stretch) -> Self {
        Self { stretch, ..self }
    }
}
