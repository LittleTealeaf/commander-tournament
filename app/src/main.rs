use app::{
    App,
    style::{FONT_BYTES, font_default},
};
use iced::{Theme, application, window};

fn main() -> iced::Result {
    let mut app = application(App::boot, App::handle_update, App::handle_view).title(App::title);

    for font in FONT_BYTES {
        app = app.font(font);
    }

    app.theme(Theme::CatppuccinMocha)
        .default_font(font_default())
        .window(window::Settings {
            exit_on_close_request: false,
            ..Default::default()
        })
        .subscription(App::subscription)
        .run()
}
