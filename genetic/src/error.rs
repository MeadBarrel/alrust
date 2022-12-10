use thiserror;
use error_stack;


#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Conversion Error")]
    ConversionError,
    #[error("Error while running the algorithm")]
    GeneticError,
    #[error("{0}")]
    GenericError(String),
}


pub type Result<T> = error_stack::Result<T, Error>;