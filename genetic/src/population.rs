use std::ops::Index;
use rand::seq::index::sample;
use rand::Rng;

use crate::genetic::*;
use crate::individual::*;
use crate::alias::*;

// Population --------------------------------------------------------------------------------------


/// The Population trait provides an interface for types that represent a population of individuals,
/// where an individual is defined by a Genotype, a Fitness value, and a Constraint value.
/// The Population trait is not intended to be used directly; instead, the Individuals type should
/// be used, which implements Population.
pub trait Population: 
    Clone
    + IntoIterator<Item=Self::Individual> 
    + Index<usize, Output=Self::Individual> 
    + Sized 
    + Extend<Self::Individual>
{
    type Individual: Individual<Genotype=Self::Genotype, Fitness=Self::Fitness>;
    type Genotype: Genotype;
    type Fitness: Fitness;

    /// Get the number of individuals in this population
    fn len(&self) -> usize;

    /// Creates a population of individuals from a vector of genomes and a fitness function.
    fn from_genomes(
        genomes: Vec<Self::Genotype>, 
        fitness_function: &FitnessFunctionAlias<Self::Genotype, Self::Fitness>
    ) -> Self;

    /// Derive a new population from this one, with new individuals. It can be used, for example,
    /// to create a new population that may have additional parameters to its constructor method
    /// without calling the constructor.
    fn derive(&self, individuals: Vec<Self::Individual>) -> Self;  

    /// Same as derive, but takes a vector of references
    fn derive_ref(&self, individuals: Vec<&Self::Individual>) -> Self;

    /// Return a vector of references to individuals in this population
    fn individuals(&self) -> Vec<&Self::Individual>;

    /// Return a vector of references to the fitness values of the individuals in the population.
    fn fitnesses(&self) -> Vec<&Self::Fitness>;

    /// Sort this population in place by fitness
    fn sort(&mut self);

    /// Truncate this population in place
    fn truncate(&mut self, index: usize);

    /// Return a new population sorted by it's fitness, with fittest individuals at the top.
    fn sorted(&self) -> Self {
        let mut result = self.derive_ref(self.individuals());
        result.sort();
        result
    }

    /// Return n best individuals.
    fn n_best(&self, n: usize) -> Vec<&Self::Individual>;

    /// Return best individual in the population.
    fn best(&self) -> Option<&Self::Individual>;

    /// Return a sample of n individuals while preserving their order
    fn sample<R:Rng>(&self, rng: &mut R, amount: usize) -> Vec<&Self::Individual> {
        let mut indices: Vec<usize> = sample(rng, self.len(), amount).into_iter().collect();
        indices.sort();
        indices.into_iter().map(|i| &self[i]).collect()
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Remove an individual at `index` from the populatio, and return the removed individual.
    fn remove(&mut self, index: usize) -> Self::Individual;
}


impl<I> Population for Individuals<I>
    where
        I: Individual
{
    type Individual = I;
    type Genotype = I::Genotype;
    type Fitness = I::Fitness;

    fn from_genomes(
        genomes: Vec<Self::Genotype>,
        fitness_function: &FitnessFunctionAlias<Self::Genotype, Self::Fitness>
    ) -> Self {
        genomes.into_iter().map(|genome| I::from_genome(genome, fitness_function)).collect()
    }

    fn fitnesses(&self) -> Vec<&Self::Fitness> {
        self.iter().map(|x| x.fitness()).collect()
    }

    fn derive(&self, individuals: Vec<Self::Individual>) -> Self {
        individuals
    }

    fn derive_ref(&self, individuals: Vec<&Self::Individual>) -> Self {
        individuals.into_iter().cloned().collect()
    }

    fn individuals(&self) -> Vec<&Self::Individual> {
        self.iter().collect()
    }

    fn sort(&mut self) {
        self.sort_by_key(|x| (x.constraint(), x.fitness().clone()));
        self.reverse()
    }

    #[inline(always)]
    fn truncate(&mut self, index: usize) {
        self.truncate(index);
    }

    fn best(&self) -> Option<&Self::Individual> {
        self.iter().max_by_key(|x| (x.constraint(), x.fitness()))
    }

    #[inline(always)]
    fn len(&self) -> usize {
        self.len()
    }

    fn n_best(&self, n: usize) -> Vec<&Self::Individual> {
        let mut references = self.individuals();
        references.sort_by_key(|x| (x.constraint(), x.fitness()));
        references.reverse();
        references.truncate(n);
        references
    }

    #[inline(always)]
    fn remove(&mut self, index: usize) -> Self::Individual {
        self.remove(index)
    }

}


