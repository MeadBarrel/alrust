use rand::prelude::*;
use crate::genetic::*;
use crate::mutate::*;
use genetic::op::CrossoverOperator;
use genetic::op::ReinsertOperator;
use genetic::op::SelectOperator;
use genetic::prelude::*;


//pub type AlchemyGA = GeneticAlgorithm<AlchemyGenome, ParettoFitness, AlchemyConstraint, ParettoAdvantage>;
pub type AlchemyGA = GeneticAlgorithm<ParettoRankedIndividual<AlchemyGenome, AlchemyConstraint>>;

pub fn create_alchemy_ga<C, S, R, Rn>
(
    fitness_function: AlchemyFitnessFunction,
    mutate: AlchemyMutator<Rn>,
    crossover: C,
    select: S,
    reinsert: R,
    initial_pool: Vec<AlchemyGenome>,
) -> AlchemyGA
    where
        C: CrossoverOperator<AlchemyGenome> + 'static,
        S: SelectOperator<ParettoRankedIndividual<AlchemyGenome, AlchemyConstraint>> + 'static,
        R: ReinsertOperator<ParettoIndividual<AlchemyGenome, AlchemyConstraint>> + 'static,
        Rn: Rng + 'static,
{
    create_paretto_algorithm(
        Box::new(fitness_function), 
        Box::new(mutate), 
        Box::new(crossover), 
        Box::new(select), 
        Box::new(reinsert), 
        initial_pool
    )
}