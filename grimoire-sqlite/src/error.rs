use thiserror;

use diesel;


#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Could not establish connection")]
    CannotEstablishConnection(#[from] diesel::ConnectionError),
    #[error("Could not run migrations")]
    MigrationFailed,
    #[error("Query Failed")]
    QueryFailed {
        #[from]
        source: diesel::result::Error,
    }
}


pub type Result<T>  = std::result::Result<T, Error>;
