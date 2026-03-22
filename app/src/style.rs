use iced::Font;

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
pub const fn default_font() -> Font {
    Font::with_name(JETBRAINS_MONO)
}
