use crate::fs::FSOperationError;

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
    #[error("{0}")]
    GenericError(String),
    #[error("Genetic error")]
    GeneticError(#[from] genetic::error::Error),
    #[error("Expression evaluation failed")]
    EvalExprFailed(#[from] evalexpr::error::EvalexprError),
    #[error("REPL error")]
    REPLError(#[from] reedline_repl_rs::Error),
    #[error("FS Operation Error")]
    FSError(#[from] FSOperationError)
}

pub type Result<T> = std::result::Result<T, OptimizationError>;