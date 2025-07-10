use app::App;
use db::prepare_conn;
use iced::{Task, Theme};
use tracing_subscriber::EnvFilter;

mod app;
mod data;
mod db;
mod widget;

fn main() -> iced::Result {
    let default_level = if cfg!(debug_assertions) {
        "info"
    } else {
        "error"
    };
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        EnvFilter::new(format!("{}={}", env!("CARGO_PKG_NAME"), default_level))
    });
    tracing_subscriber::fmt().with_env_filter(filter).init();

    let db = prepare_conn();

    iced::application("Chomp", App::update, App::view)
        .theme(|_| Theme::CatppuccinFrappe)
        .subscription(App::subscription)
        .run_with(|| (App::new(db), Task::none()))
}
