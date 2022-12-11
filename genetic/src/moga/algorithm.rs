use crate::alias::*;
use crate::genetic::Constraint;
use crate::genetic::Genotype;
use super::genetic::*;
use crate::algorithm::*;
use crate::individual::RankedIndividual;
use crate::prelude::{Individual, IndividualStruct, RankedIndividualStruct};


pub type ParettoIndividual<G, C> = IndividualStruct<G, ParettoFitness, C>;
pub type ParettoRankedIndividual<G, C> = RankedIndividualStruct<ParettoIndividual<G, C>, ParettoAdvantage>;


pub fn create_paretto_algorithm<G, C>(
    fitness_function: Box<FitnessFunctionAlias<G, ParettoFitness, C>>,
    mutate: Box<MutateOperatorAlias<G>>,
    crossover: Box<CrossoverOperatorAlias<G>>,
    select: Box<SelectOperatorAlias<ParettoRankedIndividual<G, C>>>,
    reinsert: Box<ReinsertOperatorAlias<ParettoIndividual<G, C>>>,
    initial_pool: Vec<G>,
) -> GeneticAlgorithm<ParettoRankedIndividual<G, C>>
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