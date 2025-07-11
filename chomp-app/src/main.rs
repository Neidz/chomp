use app::App;
use chomp_db::prepare_conn;
use iced::{Task, Theme};
use tracing_subscriber::EnvFilter;

mod app;
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

    let db = match prepare_conn() {
        Ok(db) => db,
        Err(err) => {
            tracing::error!("Failed to prepare database connection: {err:?}");
            std::process::exit(1);
        }
    };

    iced::application("Chomp", App::update, App::view)
        .theme(|_| Theme::CatppuccinFrappe)
        .subscription(App::subscription)
        .run_with(|| (App::new(db), Task::none()))
}
