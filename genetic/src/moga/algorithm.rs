use crate::alias::*;
use crate::genetic::Constraint;
use crate::genetic::Genotype;
use super::genetic::*;
use crate::algorithm::*;
use crate::individual::RankedIndividual;
use crate::op::{CrossoverOperator, MutateOperator, ReinsertOperator, SelectOperator};
use crate::prelude::{FitnessFunction, Individual, IndividualStruct, RankedIndividualStruct};


pub type ParettoIndividual<G, C> = IndividualStruct<G, ParettoFitness, C>;
pub type ParettoRankedIndividual<G, C> = RankedIndividualStruct<ParettoIndividual<G, C>, ParettoAdvantage>;
pub type ParettoGA<I, F, M, C, S, R> = GeneticAlgorithm<I, F, ParettoAdvantageFunction, M, C, S, R>;


pub fn create_paretto_algorithm<I, F, M, C, S, R>(
    fitness_function: F,
    mutate: M,
    crossover: C,
    select: S,
    reinsert: R,
    initial_pool: Vec<I::Genotype>,
) -> ParettoGA<I, F, M, C, S, R>
    where
        I: RankedIndividual<Fitness=ParettoFitness, Advantage=ParettoAdvantage>,
        F: FitnessFunction<Genotype=I::Genotype, Fitness=I::Fitness, Constraint=I::Constraint> + 'static,
        M: MutateOperator<I::Genotype>,
        C: CrossoverOperator<I::Genotype>,
        S: SelectOperator<I>,
        R: ReinsertOperator<I::Individual>

{
    GeneticAlgorithm::new(
        fitness_function,
        ParettoAdvantageFunction::default(),
        mutate,
        crossover,
        select,
        reinsert,
        initial_pool,
    )
}