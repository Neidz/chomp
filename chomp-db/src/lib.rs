use std::{env, path::PathBuf};

use migrate::migrate;
use migrations::{
    CREATE_CALORIE_TARGETS_TABLE_QUERY_4, CREATE_MEALS_TABLE_QUERY_2,
    CREATE_MEAL_PRODUCTS_TABLE_QUERY_3, CREATE_PRODUCTS_TABLE_QUERY_1,
    CREATE_WEIGHTRS_TABLE_QUERY_5,
};
use rusqlite::Connection;

use crate::error::Error;

mod error;
mod migrate;
mod migrations;

fn get_home_dir() -> Option<PathBuf> {
    if cfg!(target_os = "windows") {
        env::var("USERPROFILE").ok().map(PathBuf::from)
    } else {
        env::var("HOME").ok().map(PathBuf::from)
    }
}

fn run_migrations(conn: &Connection) -> Result<(), String> {
    let migrations = vec![
        CREATE_PRODUCTS_TABLE_QUERY_1,
        CREATE_MEALS_TABLE_QUERY_2,
        CREATE_MEAL_PRODUCTS_TABLE_QUERY_3,
        CREATE_CALORIE_TARGETS_TABLE_QUERY_4,
        CREATE_WEIGHTRS_TABLE_QUERY_5,
    ];

    migrate(conn, migrations)
}

pub fn prepare_conn() -> Result<Connection, Error> {
    let home = match get_home_dir() {
        Some(d) => d,
        None => return Err(Error::IO("failed to get home directory".to_string())),
    };
    let db_path: PathBuf = home
        .join(".local")
        .join("share")
        .join("chomp")
        .join("data-gui.db");

    let db_parent = match db_path.parent() {
        Some(p) => p,
        None => return Err(Error::IO("failed to get parent of db path".to_string())),
    };
    if let Err(err) = std::fs::create_dir_all(db_parent) {
        return Err(Error::IO(format!(
            "failed to create directories for db path: {err}"
        )));
    }

    let conn = match Connection::open(db_path) {
        Ok(c) => c,
        Err(err) => {
            return Err(Error::Connection(format!(
                "failed to open database connection: {err}",
            )))
        }
    };

    if let Err(err) = run_migrations(&conn) {
        return Err(Error::Migration(format!(
            "failed to perform database migration: {err}"
        )));
    }

    Ok(conn)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_perform_all_migrations_multiple_times() {
        let conn = Connection::open_in_memory().unwrap();
        run_migrations(&conn).unwrap();
        run_migrations(&conn).unwrap();
    }
}
