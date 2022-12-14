use crate::genetic::*;
use crate::op::*;

pub type Parents<G> = Vec<G>;
pub type Children<G> = Vec<G>;
pub type Matings<G> = Vec<Parents<G>>;
pub type Offspring<G> = Vec<Children<G>>;

pub type FitnessFunctionAlias<G, F> = dyn FitnessFunction<Genotype = G, Fitness = F>;

pub type Individuals<I> = Vec<I>;
pub type RankedIndividuals<I> = Vec<I>;

pub type MutateOperatorAlias<G> = dyn MutateOperator<G>;