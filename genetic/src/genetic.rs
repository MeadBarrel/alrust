use std::fmt::Debug;

use serde::Serialize;
use ordered_float::NotNan;

pub trait Genotype: Clone + Debug {}
pub trait Fitness: Clone + Debug {}
pub trait Constraint: Clone + Debug + Ord {}
pub trait Advantage: Clone + Debug + Ord + Serialize {}
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
