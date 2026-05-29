use app::{
    App,
    fonts::{FONT_BYTES, FONT_NORMAL},
};
use iced::{Application, Theme, application, window};

fn main() -> iced::Result {
    let settings = window::Settings {
        exit_on_close_request: false,
        ..Default::default()
    };

    // Initialize and configure methods
    let app = application(App::boot, App::handle_update, App::handle_view)
        .title(App::title)
        .subscription(App::subscription)
        .window(settings);

    // Fonts and Theme
    let app = FONT_BYTES
        .into_iter()
        .fold(app, Application::font)
        .default_font(FONT_NORMAL)
        .theme(Theme::CatppuccinMocha);

    app.run()
}
