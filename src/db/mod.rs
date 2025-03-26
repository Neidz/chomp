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
    let home = match get_home_dir() {
        Some(d) => d,
        None => {
            tracing::error!("Failed to get home directory");
            panic!();
        }
    };
    let db_path: PathBuf = home
        .join(".local")
        .join("share")
        .join("chomp")
        .join("data-gui.db");

    let db_parent = match db_path.parent() {
        Some(p) => p,
        None => {
            tracing::error!("Failed to get parent of db path");
            panic!();
        }
    };
    if let Err(err) = std::fs::create_dir_all(db_parent) {
        tracing::error!("Failed to create directories for db path: {}", err);
        panic!();
    }

    let conn = match Connection::open(db_path) {
        Ok(c) => c,
        Err(err) => {
            tracing::error!("Failed to open database connection: {}", err);
            panic!()
        }
    };

    let migrations = vec![
        CREATE_PRODUCTS_TABLE_QUERY_1,
        CREATE_MEALS_TABLE_QUERY_2,
        CREATE_MEAL_PRODUCTS_TABLE_QUERY_3,
        CREATE_CALORIE_TARGETS_TABLE_QUERY_4,
    ];
    if let Err(err) = migrate(&conn, migrations) {
        tracing::error!("Failed to perform database migration: {}", err);
        panic!()
    }

    conn
}
