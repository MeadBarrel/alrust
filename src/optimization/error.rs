use std::fmt::Display;
use error_stack;
use thiserror;


#[derive(thiserror::Error, Debug)]
pub enum OptimizationError {
    #[error("Error while loading")]
    LoadError,
    #[error("Error while writing output")]
    OutputError,
    #[error("Error while optimizing")]
    OptimizationError,
}


pub type Result<T> = error_stack::Result<T, OptimizationError>;