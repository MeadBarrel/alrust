use crate::algorithm::*;
use crate::genetic::*;
use crate::op::*;
use crate::prelude::Individual;
use super::population::*;
use ordered_float::NotNan;
use rand::Rng;


pub type ParettoGA<I, F, M, C, S, R, RNG> = GeneticAlgorithm<ParettoPopulation<I>, F, M, C, S, R, RNG>;


pub fn create_paretto_algorithm<I, F, M, C, S, R, RNG>(
    fitness: F,
    mutate: M,
    crossover: C,
    select: S,
    reinsert: R,
    rng: RNG,
    initial_population: Vec<I::Genotype>,
) -> ParettoGA<I, F, M, C, S, R, RNG>
    where
        I: Individual<Fitness = Vec<NotNan<f64>>>,
        F: FitnessFunction<Genotype = I::Genotype, Fitness = I::Fitness> + 'static,
        M: MutateOperator<I::Genotype>, 
        C: CrossoverOperator<I::Genotype>,
        S: SelectOperator,
        R: ReinsertOperator,
        RNG: Rng,
{
    GeneticAlgorithm::new(fitness, mutate, crossover, select, reinsert, rng, initial_population)
}
