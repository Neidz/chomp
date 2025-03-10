use app::App;
use db::prepare_conn;
use iced::{Task, Theme};

mod app;
mod data;
mod db;
mod style;
mod widget;

fn main() -> iced::Result {
    let db = prepare_conn();

    iced::application("Chomp", App::update, App::view)
        .theme(|_| Theme::CatppuccinFrappe)
        .run_with(|| (App::new(db), Task::none()))
}
