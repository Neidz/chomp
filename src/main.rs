use app::App;
use db::prepare_conn;
use iced::{Task, Theme};

mod app;
mod create_product_screen;
mod dashboard_screen;
mod data;
mod db;
mod form_field;
mod meal_list_screen;
mod meal_product_form;
mod modal;
mod product_list_screen;
mod sidebar;
mod style;
mod update_product_screen;

fn main() -> iced::Result {
    let db = prepare_conn();

    iced::application("Chomp", App::update, App::view)
        .theme(|_| Theme::CatppuccinFrappe)
        .run_with(|| (App::new(db), Task::none()))
}
