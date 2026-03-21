use app::{
    App,
    style::{FONT_BYTES, default_font},
};
use iced::{Theme, application};

fn main() -> iced::Result {
    let mut app = application(App::boot, App::handle_update, App::handle_view);

    for font in FONT_BYTES {
        app = app.font(font);
    }

    app.theme(Theme::CatppuccinMocha)
        .default_font(default_font())
        .run()
}
