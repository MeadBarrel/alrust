use std::fmt::Debug;

use serde::Serialize;
use ordered_float::NotNan;

pub trait Genotype: Clone + Debug {}

/// `Fitness` indicates how well an individual is fit for a certain task. However, this value is
/// not usually used directly by the selection and reinsertion operators: instead, `Advantage`
/// is used.
pub trait Fitness: Clone + Debug {}

/// A `Constraint` is a property of an individual indicating how much it violates a certain
/// constraint. The lower the `Constraint` value, the higher the chances that an individual will
/// be selected for reproduction, which takes precedence over its `Fitness` and `Advantage` values.
pub trait Constraint: Clone + Debug + Ord {}

/// The `Advantage` value of an individual is a measure of the individual's relative fitness
/// compared to the other individuals in the population.
pub trait Advantage: Clone + Debug + Ord + Serialize {}

/// A specific genome in a genotype of an individual
pub trait Locus: Clone + Debug {}

pub type VectorEncoded<L> = Vec<L>;
impl<L: Locus> Genotype for VectorEncoded<L> {}


impl Constraint for NotNan<f64> {}
impl Constraint for Vec<NotNan<f64>> {}

pub trait FitnessFunction {
    type Genotype: Genotype;
    type Fitness: Fitness;
    type Constraint: Constraint;

    fn fitness(&self, genome: &Self::Genotype) -> Self::Fitness;
    fn constraint(&self, genome: &Self::Genotype) -> Self::Constraint;
}

pub trait AdvantageFunction {
    type Fitness: Fitness;
    type Advantage: Advantage;

    fn call(&self, fitnesses: Vec<&Self::Fitness>) -> Vec<Self::Advantage>;
}
