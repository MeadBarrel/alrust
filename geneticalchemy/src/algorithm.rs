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
pub type AlchemyGA<C, S, R, RNG> = ParettoGA<AlchemyRankedIndividual, AlchemyFitnessFunction, AlchemyMutator, C, S, R, RNG>;

pub fn create_alchemy_ga<C, S, R, RNG>
(
    fitness_function: AlchemyFitnessFunction,
    mutate: AlchemyMutator,
    crossover: C,
    select: S,
    reinsert: R,
    rng: RNG,
    initial_pool: Vec<AlchemyGenome>,
) -> AlchemyGA<C, S, R, RNG>
    where
        C: CrossoverOperator<AlchemyGenome> + 'static,
        S: SelectOperator<AlchemyRankedIndividual> + 'static,
        R: ReinsertOperator<AlchemyIndividual> + 'static,
        RNG: Rng + 'static,
{
    create_paretto_algorithm(
        fitness_function,
        mutate,
        crossover,
        select,
        reinsert,
        rng,
        initial_pool
    )
}