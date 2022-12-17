use ordered_float::NotNan;
use std::fmt::Debug;

pub trait Genotype: Clone + Debug {}

/// `Fitness` indicates how well an individual is fit for a certain task.
pub trait Fitness: Clone + Debug + Ord {}

/// A specific genome in a genotype of an individual
pub trait Locus: Clone + Debug {}

pub type VectorEncoded<L> = Vec<L>;
impl<L: Locus> Genotype for VectorEncoded<L> {}

pub type Constraint = NotNan<f64>;

pub trait FitnessFunction {
    type Genotype: Genotype;
    type Fitness: Fitness;

    fn fitness(&self, genome: &Self::Genotype) -> Self::Fitness;
    fn constraint(&self, genome: &Self::Genotype) -> Constraint;
}
