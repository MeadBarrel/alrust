mod mask;
mod paretto;

pub mod genetic;
pub mod error;
pub mod op;
pub mod algorithm;
pub mod alias;
pub mod population;
pub mod individual;
pub mod printer;

pub mod operators;
pub mod moga;


pub mod prelude {
    pub use super::algorithm::*;
    pub use super::error::*;
    pub use super::genetic::*;
    pub use super::individual::*;
    pub use super::printer::*;
    pub use super::population::*;
    pub use super::operators::*;

    pub use super::moga::*;
    pub use super::operators::*;
}