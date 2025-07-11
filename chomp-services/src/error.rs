use rusqlite::{Error as SqliteError, ErrorCode};
use std::fmt;

#[derive(Debug)]
pub enum ServiceError {
    UniqueConstraintViolation(String),
    DatabaseError(String),
    NoRows,
    Custom(String),
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ServiceError::UniqueConstraintViolation(field) => {
                write!(f, "unique constraint violation on field: {field}")
            }
            ServiceError::DatabaseError(err) => write!(f, "database error: {err}"),
            ServiceError::NoRows => write!(f, "query returned no rows"),
            ServiceError::Custom(err) => write!(f, "{err}"),
        }
    }
}

impl From<SqliteError> for ServiceError {
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

                ServiceError::UniqueConstraintViolation(field.to_string())
            }

            SqliteError::QueryReturnedNoRows => ServiceError::NoRows,
            SqliteError::SqliteFailure(_, Some(msg)) => {
                if msg.starts_with("UNIQUE constraint failed:") {
                    let field = msg
                        .split(':')
                        .nth(1)
                        .map(|s| s.trim())
                        .unwrap_or(msg.as_str());

                    ServiceError::UniqueConstraintViolation(field.to_string())
                } else {
                    ServiceError::DatabaseError(msg)
                }
            }
            SqliteError::SqliteFailure(_, None) => {
                ServiceError::DatabaseError("unexpected database error".to_string())
            }
            _ => ServiceError::DatabaseError(format!("unexpected SQLite error: {err:?}")),
        }
    }
}
