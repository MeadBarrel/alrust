use crate::alias::*;
use crate::genetic::Constraint;
use crate::genetic::Genotype;
use super::genetic::*;
use crate::algorithm::*;


pub fn create_paretto_algorithm<G, C>(
    fitness_function: Box<FitnessFunctionAlias<G, ParettoFitness, C>>,
    mutate: Box<MutateOperatorAlias<G>>,
    crossover: Box<CrossoverOperatorAlias<G>>,
    select: Box<SelectOperatorAlias<G, ParettoFitness, C, ParettoAdvantage>>,
    reinsert: Box<ReinsertOperatorAlias<G, ParettoFitness, C>>,
    initial_pool: Vec<G>,
) -> GeneticAlgorithm<G, ParettoFitness, C, ParettoAdvantage> 
    where
        G: Genotype,
        C: Constraint,
{
    GeneticAlgorithm::new(
        fitness_function,
        Box::new(ParettoAdvantageFunction::default()),
        mutate,
        crossover,
        select,
        reinsert,
        initial_pool,
    )
}