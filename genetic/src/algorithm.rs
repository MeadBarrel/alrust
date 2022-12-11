use std::path::Iter;
use crate::alias::*;
use crate::error::*;
use crate::genetic::*;
use crate::individual::{Individual, RankedIndividual};
use crate::op::{CrossoverOperator, MutateOperator, ReinsertOperator, SelectOperator};
use crate::population::*;


pub struct GeneticAlgorithm<I, F, A, M, C, S, R>
    where I: RankedIndividual
{

    fitness_function: F,
    advantage_function: A,
    mutate: M,
    crossover: C,
    select: S,
    reinsert: R,

    population: Individuals<I::Individual>,
}


impl<I, F, A, M, C, S, R> GeneticAlgorithm<I, F, A, M, C, S, R>
    where
        I: RankedIndividual,
        F: FitnessFunction<Genotype=I::Genotype, Fitness=I::Fitness, Constraint=I::Constraint> + 'static,
        A: AdvantageFunction<Fitness=I::Fitness, Advantage=I::Advantage> + 'static,
        M: MutateOperator<I::Genotype>,
        C: CrossoverOperator<I::Genotype>,
        S: SelectOperator<I>,
        R: ReinsertOperator<I::Individual>,
{
    pub fn new(
        fitness_function: F,
        advantage_function: A,
        mutate: M,
        crossover: C,
        select: S,
        reinsert: R,
        initial_pool: Vec<I::Genotype>,
    ) -> Self {
        let population = Individuals::from_genomes(initial_pool, &fitness_function);
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
            ::from_population(self.population.clone(), &self.advantage_function);
        let matings = self.select.select_from(ranked_individuals)?;

        let mut offspring = Vec::new();

        for mating_result in matings.into_iter().map(|p| self.crossover.crossover(p)) {
            offspring.extend(mating_result?)
        }

        for child in offspring.iter_mut() {
            self.mutate.mutate(child)?;
        }

        let mut future_individuals = 
            Individuals::from_genomes(offspring, &self.fitness_function);

        self.population = self.reinsert
            .reinsert(self.population.clone(), future_individuals)?;
        
        Ok(())
    }
}


pub trait Algorithm: Iterator {}


impl<I, F, A, M, C, S, R> Algorithm for GeneticAlgorithm<I, F, A, M, C, S, R>
    where
        I: RankedIndividual,
        F: FitnessFunction<Genotype=I::Genotype, Fitness=I::Fitness, Constraint=I::Constraint> + 'static,
        A: AdvantageFunction<Fitness=I::Fitness, Advantage=I::Advantage> + 'static,
        M: MutateOperator<I::Genotype>,
        C: CrossoverOperator<I::Genotype>,
        S: SelectOperator<I>,
        R: ReinsertOperator<I::Individual>,
{}


impl<I, F, A, M, C, S, R> Iterator for GeneticAlgorithm<I, F, A, M, C, S, R>
    where
        I: RankedIndividual,
        F: FitnessFunction<Genotype=I::Genotype, Fitness=I::Fitness, Constraint=I::Constraint> + 'static,
        A: AdvantageFunction<Fitness=I::Fitness, Advantage=I::Advantage> + 'static,
        M: MutateOperator<I::Genotype>,
        C: CrossoverOperator<I::Genotype>,
        S: SelectOperator<I>,
        R: ReinsertOperator<I::Individual>,
{
    type Item = Individuals<I::Individual>;

    fn next(&mut self) -> Option<Self::Item> {
        self.advance_evolution().unwrap();
        Some(self.population.clone())
    }
}
