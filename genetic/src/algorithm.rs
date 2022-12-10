use crate::alias::*;
use crate::error::*;
use crate::genetic::*;
use crate::population::*;


pub trait Types<G, F, C, A> {
    type FitnessFunction: FitnessFunction<Genotype = G, Fitness = F, Constraint = C>;
}

pub struct GeneticAlgorithm<G, F, C, A> {

    fitness_function: Box<FitnessFunctionAlias<G, F, C>>,
    advantage_function: Box<AdvantageFunctionAlias<F, A>>,
    mutate: Box<MutateOperatorAlias<G>>,
    crossover: Box<CrossoverOperatorAlias<G>>,
    select: Box<SelectOperatorAlias<G, F, C, A>>,
    reinsert: Box<ReinsertOperatorAlias<G, F, C, A>>,

    population: Individuals<G, F, C>,
}


impl<G, F, C, A> GeneticAlgorithm<G, F, C, A> 
    where
        G: Genotype,
        F: Fitness,
        C: Constraint,
        A: Advantage
{
    pub fn new(
        fitness_function: Box<FitnessFunctionAlias<G, F, C>>,
        advantage_function: Box<AdvantageFunctionAlias<F, A>>,
        mutate: Box<MutateOperatorAlias<G>>,
        crossover: Box<CrossoverOperatorAlias<G>>,
        select: Box<SelectOperatorAlias<G, F, C, A>>,
        reinsert: Box<ReinsertOperatorAlias<G, F, C, A>>,
        initial_pool: Vec<G>,
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

    pub fn advance(&mut self) -> Result<()> {
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
        future_individuals.extend(self.population.clone());

        self.population = self.reinsert
            .reinsert(future_individuals, self.advantage_function.as_ref())?;
        
        Ok(())
    }
}
