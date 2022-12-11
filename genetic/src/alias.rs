use crate::genetic::*;
use crate::op::*;
use crate::individual::*;

pub type Parents<G> = Vec<G>;
pub type Children<G> = Vec<G>;
pub type Matings<G> = Vec<Parents<G>>;
pub type Offspring<G> = Vec<Children<G>>;

pub type FitnessFunctionAlias<G, F, C> = dyn FitnessFunction<Genotype = G, Fitness = F, Constraint = C>;
pub type AdvantageFunctionAlias<F, A> = dyn AdvantageFunction<Fitness = F, Advantage = A>;

pub type Individuals<I> = Vec<I>;
pub type RankedIndividuals<I> = Vec<I>;

pub type MutateOperatorAlias<G> = dyn MutateOperator<G>;
pub type CrossoverOperatorAlias<G> = dyn CrossoverOperator<G>;
pub type SelectOperatorAlias<I> = dyn SelectOperator<I>;
pub type ReinsertOperatorAlias<I> = dyn ReinsertOperator<I>;