use app::App;
use iced::{Task, Theme};

mod app;

fn main() -> iced::Result {
    iced::application("Chomp", App::update, App::view)
        .theme(|_| Theme::CatppuccinFrappe)
        .run_with(|| (App::new(), Task::none()))
}
