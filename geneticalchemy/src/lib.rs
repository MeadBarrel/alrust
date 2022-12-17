pub mod algorithm;
pub mod fitness;
pub mod gene;
pub mod genetic;
pub mod genome;
pub mod mutate;

pub mod prelude {
    pub use super::{algorithm::*, fitness::*, gene::*, genetic::*, genome::*, mutate::*};
}
