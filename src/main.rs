pub mod m3u;
pub mod types;
pub mod ui;
pub mod views;

use crate::ui::App;

use iced::Theme;

fn main() -> iced::Result {
    iced::application("Rustream", App::update, App::view)
        .theme(|_| Theme::Dark)
        .resizable(true)
        .centered()
        .run_with(|| App::new())
}
