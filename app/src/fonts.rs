use iced::{
    Font,
    font::{Family, Stretch, Style, Weight},
};

macro_rules! font_style {
    // 1. Explicitly handle (Normal, Normal) to avoid overlapping match arms
    (Normal, Normal) => {
        pub const FONT_NORMAL: Font = Font::with_name("JetBrains Mono");
    };
    // 2. Handle (Normal, Italic) -> FONT_ITALIC
    (Normal, $style:ident) => {
        paste::paste! {
            pub const [<FONT_ $style:upper>]: Font = Font {
                style: Style::$style,
                ..Font::with_name("JetBrains Mono")
            };
        }
    };
    // 3. Handle (Bold, Normal) -> FONT_BOLD
    ($weight:ident, Normal) => {
        paste::paste! {
            pub const [<FONT_ $weight:upper>]: Font = Font {
                weight: Weight::$weight,
                ..Font::with_name("JetBrains Mono")
            };
        }
    };
    // 4. Handle (Bold, Italic) -> FONT_BOLD_ITALIC
    ($weight:ident, $style:ident) => {
        paste::paste! {
            pub const [<FONT_ $weight:upper _ $style:upper>]: Font = Font {
                weight: Weight::$weight,
                style: Style::$style,
                ..Font::with_name("JetBrains Mono")
            };
        }
    };
}

macro_rules! font_styles {
    // Base case: No more weights left to process. Stop recursing.
    ([], [$($style:ident),+]) => {};

    // Recursive case: Pop the first weight, loop over all styles, then recurse.
    ([$first_weight:ident $(, $rest_weights:ident)*], [$($style:ident),+]) => {

        // Inner loop: Pair the current weight with every style
        $(
            font_style!($first_weight, $style);
        )+

        // Recurse: Call this macro again with the remaining weights
        font_styles!([$($rest_weights),*], [$($style),+]);
    };
}

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

// Generate all combinations!
font_styles!(
    [
        Bold, ExtraBold, ExtraLight, Normal, Light, Medium, Semibold, Thin
    ],
    [Normal, Italic, Oblique]
);

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
