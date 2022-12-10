use crate::genetic::*;
use crate::op::*;
use crate::individual::*;

pub type Parents<G> = Vec<G>;
pub type Children<G> = Vec<G>;
pub type Matings<G> = Vec<Parents<G>>;
pub type Offspring<G> = Vec<Children<G>>;

pub type FitnessFunctionAlias<G, F, C> = dyn FitnessFunction<Genotype = G, Fitness = F, Constraint = C>;
pub type AdvantageFunctionAlias<F, A> = dyn AdvantageFunction<Fitness = F, Advantage = A>;

pub type Individuals<G, F, C> = Vec<Individual<G, F, C>>;
pub type RankedIndividuals<G, F, C, A> = Vec<RankedIndividual<G, F, C, A>>;

pub type MutateOperatorAlias<G> = dyn MutateOperator<G>;
pub type CrossoverOperatorAlias<G> = dyn CrossoverOperator<G>;
pub type SelectOperatorAlias<G, F, C, A> = dyn SelectOperator<G, F, C, A>;
pub type ReinsertOperatorAlias<G, F, C> = dyn ReinsertOperator<G, F, C>;