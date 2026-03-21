use iced::Font;

pub const JETBRAINS_MONO: &str = "JetBrains Mono";

pub const FONT_BYTES: [&[u8]; 16] = [
    include_bytes!("../res/fonts/JetBrainsMono-Bold.ttf"),
    include_bytes!("../res/fonts/JetBrainsMono-BoldItalic.ttf"),
    include_bytes!("../res/fonts/JetBrainsMono-ExtraBold.ttf"),
    include_bytes!("../res/fonts/JetBrainsMono-ExtraBoldItalic.ttf"),
    include_bytes!("../res/fonts/JetBrainsMono-ExtraLight.ttf"),
    include_bytes!("../res/fonts/JetBrainsMono-ExtraLightItalic.ttf"),
    include_bytes!("../res/fonts/JetBrainsMono-Italic.ttf"),
    include_bytes!("../res/fonts/JetBrainsMono-Light.ttf"),
    include_bytes!("../res/fonts/JetBrainsMono-LightItalic.ttf"),
    include_bytes!("../res/fonts/JetBrainsMono-Medium.ttf"),
    include_bytes!("../res/fonts/JetBrainsMono-MediumItalic.ttf"),
    include_bytes!("../res/fonts/JetBrainsMono-Regular.ttf"),
    include_bytes!("../res/fonts/JetBrainsMono-SemiBold.ttf"),
    include_bytes!("../res/fonts/JetBrainsMono-SemiBoldItalic.ttf"),
    include_bytes!("../res/fonts/JetBrainsMono-Thin.ttf"),
    include_bytes!("../res/fonts/JetBrainsMono-ThinItalic.ttf"),
];

#[must_use]
pub const fn default_font() -> Font {
    Font::with_name(JETBRAINS_MONO)
}
