use app::{
    App,
    fonts::{FONT_BYTES, default_font},
};
use iced::application;

pub fn main() -> iced::Result {
    let mut app = application(App::boot, App::updater, App::app_view);

    for font in FONT_BYTES {
        app = app.font(font);
    }

    app.default_font(default_font()).run()
}
