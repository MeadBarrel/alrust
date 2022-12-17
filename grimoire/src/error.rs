use error_stack;
use thiserror;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Could not load database at {0}")]
    DatabaseConnectionFailed(String),
    #[error("Query error")]
    QueryFailed,
}

pub type Result<T> = error_stack::Result<T, Error>;
