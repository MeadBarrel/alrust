use rand::prelude::Rng;

use genetic::{
    op::{CrossoverOperator, ReinsertOperator, SelectOperator},
    prelude::{create_paretto_algorithm, ParettoGA, ParettoIndividual},
};

use crate::{fitness::*, mutate::*};

pub type AlchemyIndividual = ParettoIndividual<AlchemyGenome>;
pub type AlchemyGA<C, S, R, RNG> =
    ParettoGA<AlchemyIndividual, AlchemyFitnessFunction, AlchemyMutator, C, S, R, RNG>;

pub fn create_alchemy_ga<C, S, R, RNG>(
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
    S: SelectOperator + 'static,
    R: ReinsertOperator + 'static,
    RNG: Rng + 'static,
{
    create_paretto_algorithm(
        fitness_function,
        mutate,
        crossover,
        select,
        reinsert,
        rng,
        initial_pool,
    )
}
