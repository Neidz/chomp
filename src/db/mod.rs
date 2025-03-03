use std::{env, path::PathBuf};

use migrate::migrate;
use migrations::{
    CREATE_CALORIE_TARGETS_TABLE_QUERY_4, CREATE_MEALS_TABLE_QUERY_2,
    CREATE_MEAL_PRODUCTS_TABLE_QUERY_3, CREATE_PRODUCTS_TABLE_QUERY_1,
};
use rusqlite::Connection;

mod migrate;
mod migrations;

fn get_home_dir() -> Option<PathBuf> {
    if cfg!(target_os = "windows") {
        env::var("USERPROFILE").ok().map(PathBuf::from)
    } else {
        env::var("HOME").ok().map(PathBuf::from)
    }
}

pub fn prepare_conn() -> Connection {
    let home = get_home_dir().expect("Unable to get home directory");

    let db_path: PathBuf = home
        .join(".local")
        .join("share")
        .join("chomp")
        .join("data-gui.db");

    std::fs::create_dir_all(db_path.parent().expect("Invalid database path"))
        .expect("Unable to create directories for db");

    let conn = Connection::open(db_path).unwrap_or_else(|err| {
        panic!("Unable to open database connection: {}", err);
    });

    let migrations = vec![
        CREATE_PRODUCTS_TABLE_QUERY_1,
        CREATE_MEALS_TABLE_QUERY_2,
        CREATE_MEAL_PRODUCTS_TABLE_QUERY_3,
        CREATE_CALORIE_TARGETS_TABLE_QUERY_4,
    ];
    migrate(&conn, migrations).unwrap_or_else(|err| {
        panic!("Failed to perform database migrations: {}", err);
    });

    conn
}
