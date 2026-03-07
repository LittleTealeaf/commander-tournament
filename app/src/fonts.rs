use iced::Font;

pub const JETBRAINS_MONO: &str = "JetBrains Mono";

#[must_use]
pub const fn default_font() -> Font {
    Font::with_name(JETBRAINS_MONO)
}

#[must_use]
pub const fn jetbrains_mono_bytes() -> &'static [u8] {
    include_bytes!("../res/JetBrainsMono-Regular.ttf")
}
