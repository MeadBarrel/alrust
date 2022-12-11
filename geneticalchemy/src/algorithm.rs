use rand::prelude::*;
use crate::genetic::*;
use crate::mutate::*;
use genetic::op::CrossoverOperator;
use genetic::op::ReinsertOperator;
use genetic::op::SelectOperator;
use genetic::prelude::*;


//pub type AlchemyGA = GeneticAlgorithm<AlchemyGenome, ParettoFitness, AlchemyConstraint, ParettoAdvantage>;
pub type AlchemyIndividual = ParettoIndividual<AlchemyGenome, AlchemyConstraint>;
pub type AlchemyRankedIndividual = ParettoRankedIndividual<AlchemyGenome, AlchemyConstraint>;
pub type AlchemyGA<C, S, R, Rn> = ParettoGA<AlchemyRankedIndividual, AlchemyFitnessFunction, AlchemyMutator<Rn>, C, S, R>;

pub fn create_alchemy_ga<C, S, R, Rn>
(
    fitness_function: AlchemyFitnessFunction,
    mutate: AlchemyMutator<Rn>,
    crossover: C,
    select: S,
    reinsert: R,
    initial_pool: Vec<AlchemyGenome>,
) -> AlchemyGA<C, S, R, Rn>
    where
        C: CrossoverOperator<AlchemyGenome> + 'static,
        S: SelectOperator<AlchemyRankedIndividual> + 'static,
        R: ReinsertOperator<AlchemyIndividual> + 'static,
        Rn: Rng + 'static,
{
    create_paretto_algorithm(
        fitness_function,
        mutate,
        crossover,
        select,
        reinsert,
        initial_pool
    )
}