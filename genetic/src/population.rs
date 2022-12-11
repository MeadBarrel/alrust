use std::marker::PhantomData;
use std::path::Iter;
use std::process::Output;
use crate::genetic::*;
use crate::individual::*;
use crate::alias::*;

// Population --------------------------------------------------------------------------------------

pub trait Population: IntoIterator<Item=Self::Individual> {
    type Individual: Individual<Genotype=Self::Genotype, Fitness=Self::Fitness, Constraint=Self::Constraint>;
    type Genotype;
    type Fitness;
    type Constraint;

    fn from_genomes(
        genomes: Vec<Self::Genotype>, 
        fitness_function: &FitnessFunctionAlias<Self::Genotype, Self::Fitness, Self::Constraint>
    ) -> Self;

    fn fitnesses(&self) -> Vec<&Self::Fitness>;
    fn ranked<A>(
        self,
        advantage_function: &A) -> RankedIndividuals<RankedIndividualStruct<Self::Individual, A::Advantage>>
        where A: AdvantageFunction<Fitness=Self::Fitness> + 'static;

}


impl<I> Population for Individuals<I>
    where
        I: Individual
{
    type Individual = I;
    type Genotype = I::Genotype;
    type Fitness = I::Fitness;
    type Constraint = I::Constraint;

    fn from_genomes(
        genomes: Vec<Self::Genotype>,
        fitness_function: &FitnessFunctionAlias<Self::Genotype, Self::Fitness, Self::Constraint>
    ) -> Self {
        genomes.into_iter().map(|genome| I::from_genome(genome, fitness_function)).collect()
    }

    fn fitnesses(&self) -> Vec<&Self::Fitness> {
        self.iter().map(|x| x.fitness()).collect()
    }

    fn ranked<A>(
        self,
        advantage_function: &A
    ) -> RankedIndividuals<RankedIndividualStruct<Self::Individual, A::Advantage>>
        where A: AdvantageFunction<Fitness=Self::Fitness> + 'static
    {
        RankedIndividuals::<RankedIndividualStruct<Self::Individual, A::Advantage>>
            ::from_population(self, advantage_function)
    }
}



// RankedPopulation --------------------------------------------------------------------------------

pub trait RankedPopulation {
    type Item: RankedIndividual<Individual=Self::Individual>;
    type Individual: Individual<Genotype=Self::Genotype, Fitness=Self::Fitness, Constraint=Self::Constraint>;
    type Genotype: Genotype;
    type Fitness: Fitness;
    type Constraint: Constraint;
    type Advantage: Advantage;

    fn from_population<P>(
        population: P,
        advantage_function: &AdvantageFunctionAlias<Self::Fitness, Self::Advantage>) -> Self
    where P: Population<Individual=Self::Individual, Fitness=Self::Fitness>;

    fn best(&self) -> Option<&Self::Item>;
    fn to_individuals(self) -> Individuals<Self::Individual>;
}


impl<I> RankedPopulation for RankedIndividuals<I>
    where
        I: RankedIndividual
{
    type Item = I;
    type Individual = I::Individual;
    type Genotype = I::Genotype;
    type Fitness = I::Fitness;
    type Constraint = I::Constraint;
    type Advantage = I::Advantage;

    fn best(&self) -> Option<&Self::Item> {
        self.iter().max_by_key(|x| x.advantage())
    }

    fn from_population<P>(
        population: P,
        advantage_function: &AdvantageFunctionAlias<Self::Fitness, Self::Advantage>) -> Self
    where P: Population<Individual=Self::Individual, Fitness=Self::Fitness> {
        let fitnesses = population.fitnesses();
        let advantages = advantage_function.call(fitnesses);
        population.into_iter().zip(advantages.into_iter())
            .map(
                |(individual, advantage)|
                RankedIndividual::from_individual(individual, advantage)
            ).collect()
    }

    fn to_individuals(self) -> Individuals<Self::Individual> {
        self.into_iter().map(|x| x.into_individual()).collect()
    }
}
