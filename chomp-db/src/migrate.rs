use rusqlite::{params, Connection};

const CREATE_SCHEMA_MIGRATIONS_TABLE_QUERY: &str = "
    CREATE TABLE IF NOT EXISTS schema_migrations (
        version INTEGER NOT NULL,
        dirty INTEGER NOT NULL
    )";

const CREATE_INITIAL_MIGRATIONS_ROW_QUERY: &str = "
    INSERT INTO schema_migrations (version, dirty)
    VALUES (0, 0)";

const GET_SCHEMA_MIGRATIONS_STATE_QUERY: &str = "
    SELECT version, dirty
    FROM schema_migrations";

const UPDATE_SCHEMA_MIGRATIONS_VERSION_QUERY: &str = "
    UPDATE schema_migrations
    SET version = ?1";

const MARK_DIRTY_QUERY: &str = "
    UPDATE schema_migrations
    SET dirty = 1";

pub struct Migration {
    pub query: &'static str,
    pub id: usize,
}

pub fn migrate(conn: &Connection, migrations: Vec<Migration>) -> Result<(), String> {
    conn.execute(CREATE_SCHEMA_MIGRATIONS_TABLE_QUERY, ())
        .map_err(|err| err.to_string())?;

    let (version, dirty): (usize, usize) =
        match conn.query_row(GET_SCHEMA_MIGRATIONS_STATE_QUERY, (), |row| {
            Ok((row.get(0)?, row.get(1)?))
        }) {
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                conn.execute(CREATE_INITIAL_MIGRATIONS_ROW_QUERY, ())
                    .map_err(|err| err.to_string())?;
                (0, 0)
            }
            Ok(data) => data,
            Err(e) => return Err(e.to_string()),
        };

    if dirty == 1 {
        return Err(
            "database schema_migrations table is dirty, fix issue manually, aborting".to_string(),
        );
    }

    for migration in migrations {
        if migration.id > version {
            apply_migration(conn, migration.query, migration.id).map_err(|err| err.to_string())?;
        }
    }

    Ok(())
}

fn apply_migration(
    conn: &Connection,
    query: &'static str,
    id: usize,
) -> Result<(), rusqlite::Error> {
    match conn.execute(query, ()) {
        Ok(_) => {
            conn.execute(UPDATE_SCHEMA_MIGRATIONS_VERSION_QUERY, params![id])?;
            Ok(())
        }
        Err(e) => {
            conn.execute(MARK_DIRTY_QUERY, ())?;
            Err(e)
        }
    }
}
