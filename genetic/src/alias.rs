use crate::genetic::*;
use crate::op::*;


pub type FitnessFunctionAlias<F, P> = dyn FitnessFunction<Fitness = F, Phenotype = P>;
pub type ConstraintFunctionAlias<C, P> = dyn ConstraintFunction<Constraint = C, Phenotype = P>;
pub type AdvantageFunctionAlias<F, A> = dyn AdvantageFunction<Fitness = F, Advantage = A>;

pub type IndividualAlias<F, G, C> = dyn Individual<Fitness = F, Genotype = G, Constraint = C>;
pub type RankedIndividualAlias<I, A> = dyn RankedIndividual<Individual = I, Advantage = A>;

pub type IncubatorAlias<G, P> = dyn Incubator<Genotype = G, Phenotype = P>;
pub type MutateOperatorAlias<G> = dyn MutateOperator<G>;
pub type CrossoverOperatorAlias<G> = dyn CrossoverOperator<G>;
pub type SelectOperatorAlias<F, G, C> = IndividualAlias<F, G, C>;
