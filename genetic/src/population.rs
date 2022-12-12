use crate::genetic::*;
use crate::individual::*;
use crate::alias::*;

// Population --------------------------------------------------------------------------------------


/// The Population trait provides an interface for types that represent a population of individuals,
/// where an individual is defined by a Genotype, a Fitness value, and a Constraint value.
/// The Population trait is not intended to be used directly; instead, the Individuals type should
/// be used, which implements Population.
pub trait Population: IntoIterator<Item=Self::Individual> {
    type Individual: Individual<Genotype=Self::Genotype, Fitness=Self::Fitness, Constraint=Self::Constraint>;
    type Genotype;
    type Fitness;
    type Constraint;

    /// Creates a population of individuals from a vector of genomes and a fitness function.
    fn from_genomes(
        genomes: Vec<Self::Genotype>, 
        fitness_function: &FitnessFunctionAlias<Self::Genotype, Self::Fitness, Self::Constraint>
    ) -> Self;

    /// Returns a vector of references to the fitness values of the individuals in the population.
    fn fitnesses(&self) -> Vec<&Self::Fitness>;

    /// Ranks the individuals in the population based on a given advantage function.
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


/// The RankedPopulation trait provides an interface for types that represent a ranked population of
/// individuals, where an individual is defined by a Genotype, a Fitness value, a Constraint value,
/// and an Advantage value. The RankedPopulation trait is not intended to be used directly; instead,
/// the RankedIndividuals type should be used, which implements RankedPopulation.
pub trait RankedPopulation {
    type Item: RankedIndividual<Individual=Self::Individual>;
    type Individual: Individual<Genotype=Self::Genotype, Fitness=Self::Fitness, Constraint=Self::Constraint>;
    type Genotype: Genotype;
    type Fitness: Fitness;
    type Constraint: Constraint;
    type Advantage: Advantage;

    /// Creates a ranked population of individuals from a given population and an advantage function.
    ///
    /// This method creates a new ranked population by applying the given advantage function to
    /// the fitness values of the individuals in the given population. The resulting advantage
    /// values are then used to create new `RankedIndividual`s, which are added to the new ranked
    /// population.
    fn from_population<P>(
        population: P,
        advantage_function: &AdvantageFunctionAlias<Self::Fitness, Self::Advantage>) -> Self
    where P: Population<Individual=Self::Individual, Fitness=Self::Fitness>;

    /// Returns the best individual in the population, if one exists.
    ///
    /// The best individual is defined as the individual with the highest advantage value.
    fn best(&self) -> Option<&Self::Item>;

     /// Converts the ranked population into an unranked population of individuals.
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
