#[derive(thiserror::Error, Debug)]
pub enum OptimizationError {
    #[error("Error while writing output")]
    OutputError,
    #[error("Error while optimizing")]
    OptimizationError,
    #[error("Error while loading")]
    LoadError,
}

pub type Result<T> = error_stack::Result<T, OptimizationError>;