use app::{
    App,
    fonts::{FONT_BYTES, default_font},
};
use iced::{Theme, application};

pub fn main() -> iced::Result {
    let mut app = application(App::boot, App::updater, App::app_view)
        .subscription(App::autosave_subscription);

    for font in FONT_BYTES {
        app = app.font(font);
    }

    app.theme(Theme::CatppuccinMocha)
        .default_font(default_font())
        .run()
}
