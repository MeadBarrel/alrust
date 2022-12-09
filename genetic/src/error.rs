use thiserror;
use error_stack;


#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Conversion Error")]
    ConversionError,
    #[error("Cannot build: missing {0}")]
    BuilderMissingParameter(String),
    #[error("Cannot build: length of {0} must be between {1} and {2}")]
    BuilderInvalidLength(String, usize, usize),
    #[error("Error while running the algorithm")]
    GeneticError,
}


pub type Result<T> = error_stack::Result<T, Error>;