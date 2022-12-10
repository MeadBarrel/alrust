pub mod mutate;
pub mod genetic;
pub mod fitness;
pub mod algorithm;
pub mod builder;
pub mod incubator;


pub mod prelude {
    pub use super::mutate::*;
    pub use super::genetic::*;
    pub use super::fitness::*;
    pub use super::algorithm::*;
    pub use super::builder::*;
    pub use super::incubator::*;
}