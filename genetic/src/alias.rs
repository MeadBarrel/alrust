use crate::genetic::*;
use crate::op::*;
use crate::individual::*;

pub type Parents<G> = Vec<G>;
pub type Children<G> = Vec<G>;
pub type Matings<G> = Vec<Parents<G>>;
pub type Offspring<G> = Vec<Children<G>>;

pub type FitnessFunctionAlias<G, F, C> = Box<dyn FitnessFunction<Genotype = G, Fitness = F, Constraint = C>>;
pub type AdvantageFunctionAlias<F, A> = Box<dyn AdvantageFunction<Fitness = F, Advantage = A>>;

pub type Individuals<G, F, C> = Vec<Individual<G, F, C>>;
pub type RankedIndividuals<G, F, C, A> = Vec<RankedIndividual<G, F, C, A>>;

pub type MutateOperatorAlias<G> = Box<dyn MutateOperator<G>>;
pub type CrossoverOperatorAlias<G> = Box<dyn CrossoverOperator<G>>;
pub type SelectOperatorAlias<F, G, C, A> = Box<dyn SelectOperator<G, F, C, A>>;
pub type ReinsertOperatorAlias<G, F, C, A> = Box<dyn ReinsertOperator<G, F, C, A>>;