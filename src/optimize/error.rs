#[derive(thiserror::Error, Debug)]
pub enum OptimizationError {
    #[error("Error while writing output")]
    OutputError,
    #[error("Error while optimizing")]
    OptimizationError,
    #[error("Error while loading")]
    LoadError,
    #[error("Threading error")]
    ThreadError,
    #[error("Generic error")]
    GenericError(#[from] genetic::error::Error),
    #[error("Expression evaluation failed")]
    EvalExprFailed(#[from] evalexpr::error::EvalexprError)
}

pub type Result<T> = std::result::Result<T, OptimizationError>;