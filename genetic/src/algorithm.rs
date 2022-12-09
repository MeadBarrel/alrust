use crate::genetic::*;
use crate::alias::*;

pub trait Algorithm {}


pub struct GeneticAlgorithm<G, P, F, C, A> 
    where
        G: Genotype,
        P: Phenotype,
        F: Fitness,
        C: Constraint,
        A: Advantage,
{
    fitness_function: Box<FitnessFunctionAlias<F, P>>,
    constraint_function: Box<ConstraintFunctionAlias<C, P>>,
    advantage_function: Box<AdvantageFunctionAlias<F, A>>,

    mutate: Box<MutateOperatorAlias<G>>,
    crossover: Box<CrossoverOperatorAlias<G>>,
    select: Box<SelectOperatorAlias<F, G, C>>,
}