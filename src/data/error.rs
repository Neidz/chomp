use rusqlite::{Error as SqliteError, ErrorCode};
use std::fmt;

#[allow(unused)]
#[derive(Debug)]
pub enum DataError {
    UniqueConstraintViolation(String),
    DatabaseError(String),
    NoRows,
    Custom(String),
}

impl fmt::Display for DataError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DataError::UniqueConstraintViolation(field) => {
                write!(f, "unique constraint violation on field: {}", field)
            }
            DataError::DatabaseError(err) => write!(f, "database error: {}", err),
            DataError::NoRows => write!(f, "query returned no rows"),
            DataError::Custom(err) => write!(f, "{}", err),
        }
    }
}

impl From<SqliteError> for DataError {
    fn from(err: SqliteError) -> Self {
        match err {
            SqliteError::SqliteFailure(
                rusqlite::ffi::Error {
                    code: ErrorCode::ConstraintViolation,
                    extended_code: 2067,
                },
                Some(msg),
            ) => {
                let field = msg
                    .split(':')
                    .nth(1)
                    .map(|s| s.trim())
                    .unwrap_or(msg.as_str());

                DataError::UniqueConstraintViolation(field.to_string())
            }
            SqliteError::QueryReturnedNoRows => DataError::NoRows,
            SqliteError::SqliteFailure(_, Some(msg)) => DataError::DatabaseError(msg),
            SqliteError::SqliteFailure(_, None) => {
                DataError::DatabaseError("unexpected database error".to_string())
            }
            _ => DataError::DatabaseError(format!("unexpected SQLite error: {:?}", err)),
        }
    }
}
