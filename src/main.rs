use app::App;
use db::prepare_conn;
use iced::{Task, Theme};

mod app;
mod data;
mod db;
mod form_field;
mod meal_list;
mod product_form;
mod product_list;
mod style;

fn main() -> iced::Result {
    let db = prepare_conn();

    iced::application("Chomp", App::update, App::view)
        .theme(|_| Theme::CatppuccinFrappe)
        .run_with(|| (App::new(db), Task::none()))
}
