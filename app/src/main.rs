use app::{
    App,
    fonts::{default_font, jetbrains_mono_bytes},
};
use iced::application;

pub fn main() -> iced::Result {
    application(App::boot, App::updater, App::app_view)
        .font(jetbrains_mono_bytes())
        .default_font(default_font())
        .run()
}
