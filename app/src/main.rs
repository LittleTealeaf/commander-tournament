use app::App;

pub fn main() -> iced::Result {
    iced::run(App::updater, App::app_view)
}
