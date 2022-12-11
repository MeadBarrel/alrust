use std::path::Iter;
use rand::Rng;
use crate::alias::*;
use crate::error::*;
use crate::genetic::*;
use crate::individual::{Individual, RankedIndividual};
use crate::op::{CrossoverOperator, MutateOperator, ReinsertOperator, SelectOperator};
use crate::population::*;


pub struct GeneticAlgorithm<I, F, A, M, C, S, R, RNG>
    where I: RankedIndividual
{

    fitness_function: F,
    advantage_function: A,
    mutate: M,
    crossover: C,
    select: S,
    reinsert: R,
    rng: RNG,

    population: Individuals<I::Individual>,
}


impl<I, F, A, M, C, S, R, RNG> GeneticAlgorithm<I, F, A, M, C, S, R, RNG>
    where
        I: RankedIndividual,
        F: FitnessFunction<Genotype=I::Genotype, Fitness=I::Fitness, Constraint=I::Constraint> + 'static,
        A: AdvantageFunction<Fitness=I::Fitness, Advantage=I::Advantage> + 'static,
        M: MutateOperator<I::Genotype>,
        C: CrossoverOperator<I::Genotype>,
        S: SelectOperator<I>,
        R: ReinsertOperator<I::Individual>,
        RNG: Rng,
{
    pub fn new(
        fitness_function: F,
        advantage_function: A,
        mutate: M,
        crossover: C,
        select: S,
        reinsert: R,
        rng: RNG,
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
            rng,
            population,
        }
    }

    pub fn advance_evolution(&mut self) -> Result<()> {
        let ranked_individuals = RankedIndividuals
            ::from_population(self.population.clone(), &self.advantage_function);
        let matings = self.select.select_from(ranked_individuals, &mut self.rng)?;

        let mut offspring = Vec::new();

        for mating_result in matings.into_iter().map(|p| self.crossover.crossover(p, &mut self.rng)) {
            offspring.extend(mating_result?)
        }

        for child in offspring.iter_mut() {
            self.mutate.mutate(child, &mut self.rng)?;
        }

        let mut future_individuals = 
            Individuals::from_genomes(offspring, &self.fitness_function);

        self.population = self.reinsert
            .reinsert(self.population.clone(), future_individuals, &mut self.rng)?;
        
        Ok(())
    }
}


pub trait Algorithm: Iterator {}


impl<I, F, A, M, C, S, R, RNG> Algorithm for GeneticAlgorithm<I, F, A, M, C, S, R, RNG>
    where
        I: RankedIndividual,
        F: FitnessFunction<Genotype=I::Genotype, Fitness=I::Fitness, Constraint=I::Constraint> + 'static,
        A: AdvantageFunction<Fitness=I::Fitness, Advantage=I::Advantage> + 'static,
        M: MutateOperator<I::Genotype>,
        C: CrossoverOperator<I::Genotype>,
        S: SelectOperator<I>,
        R: ReinsertOperator<I::Individual>,
        RNG: Rng,
{}


impl<I, F, A, M, C, S, R, RNG> Iterator for GeneticAlgorithm<I, F, A, M, C, S, R, RNG>
    where
        I: RankedIndividual,
        F: FitnessFunction<Genotype=I::Genotype, Fitness=I::Fitness, Constraint=I::Constraint> + 'static,
        A: AdvantageFunction<Fitness=I::Fitness, Advantage=I::Advantage> + 'static,
        M: MutateOperator<I::Genotype>,
        C: CrossoverOperator<I::Genotype>,
        S: SelectOperator<I>,
        R: ReinsertOperator<I::Individual>,
        RNG: Rng,
{
    type Item = Individuals<I::Individual>;

    fn next(&mut self) -> Option<Self::Item> {
        self.advance_evolution().unwrap();
        Some(self.population.clone())
    }
}
