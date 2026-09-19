use app::ComTourApp;
use iced_tea::App as _;

fn main() -> iced::Result {
    env_logger::init();
    ComTourApp::run()
}
