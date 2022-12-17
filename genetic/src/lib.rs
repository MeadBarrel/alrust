mod mask;
mod paretto;

pub mod algorithm;
pub mod alias;
pub mod error;
pub mod genetic;
pub mod individual;
pub mod op;
pub mod population;
pub mod printer;

pub mod moga;
pub mod operators;

pub mod prelude {
    pub use super::{
        algorithm::*, error::*, genetic::*, individual::*, operators::*, population::*, printer::*,
    };

    pub use super::{moga::*, operators::*};
}
