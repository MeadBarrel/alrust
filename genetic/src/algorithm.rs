use crate::alias::*;
use crate::error::*;
use crate::genetic::*;
use crate::individual::{Individual, RankedIndividual};
use crate::population::*;


pub struct GeneticAlgorithm<I>
    where
        I: RankedIndividual,
{

    fitness_function: Box<FitnessFunctionAlias<I::Genotype, I::Fitness, I::Constraint>>,
    advantage_function: Box<AdvantageFunctionAlias<I::Fitness, I::Advantage>>,
    mutate: Box<MutateOperatorAlias<I::Genotype>>,
    crossover: Box<CrossoverOperatorAlias<I::Genotype>>,
    select: Box<SelectOperatorAlias<I>>,
    reinsert: Box<ReinsertOperatorAlias<I::Individual>>,

    population: Individuals<I::Individual>,
}


impl<I> GeneticAlgorithm<I>
    where
        I: RankedIndividual,
{
    pub fn new(
        fitness_function: Box<FitnessFunctionAlias<I::Genotype, I::Fitness, I::Constraint>>,
        advantage_function: Box<AdvantageFunctionAlias<I::Fitness, I::Advantage>>,
        mutate: Box<MutateOperatorAlias<I::Genotype>>,
        crossover: Box<CrossoverOperatorAlias<I::Genotype>>,
        select: Box<SelectOperatorAlias<I>>,
        reinsert: Box<ReinsertOperatorAlias<I::Individual>>,
        initial_pool: Vec<I::Genotype>,
    ) -> Self {
        let population = Individuals::from_genomes(initial_pool, fitness_function.as_ref());
        Self {
            fitness_function,
            advantage_function,
            mutate,
            crossover,
            select,
            reinsert,
            population,
        }
    }

    pub fn advance_evolution(&mut self) -> Result<()> {
        let ranked_individuals = RankedIndividuals
            ::from_population(self.population.clone(), self.advantage_function.as_ref());
        let matings = self.select.select_from(ranked_individuals)?;

        let mut offspring = Vec::new();

        for mating_result in matings.into_iter().map(|p| self.crossover.crossover(p)) {
            offspring.extend(mating_result?)
        }

        for child in offspring.iter_mut() {
            self.mutate.mutate(child)?;
        }

        let mut future_individuals = 
            Individuals::from_genomes(offspring, self.fitness_function.as_ref());

        self.population = self.reinsert
            .reinsert(self.population.clone(), future_individuals)?;
        
        Ok(())
    }
}


impl<I> Iterator for GeneticAlgorithm<I>
    where
        I: RankedIndividual,
{
    type Item = Individuals<I::Individual>;

    fn next(&mut self) -> Option<Self::Item> {
        self.advance_evolution().unwrap();
        Some(self.population.clone())
    }
}
