use app::{
    App,
    fonts::{FONT_BYTES, FONT_NORMAL},
};
use iced::{Application, window};
use iced_tea::App as _;

fn main() -> iced::Result {
    env_logger::init();
    let settings = window::Settings {
        exit_on_close_request: false,
        ..Default::default()
    };

    let app = App::application().window(settings);
    let app = FONT_BYTES
        .into_iter()
        .fold(app, Application::font)
        .default_font(FONT_NORMAL);

    app.run()
}
