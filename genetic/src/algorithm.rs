use rand::Rng;

use crate::error::*;
use crate::op::CrossoverOperator;
use crate::op::MutateOperator;
use crate::op::ReinsertOperator;
use crate::op::SelectOperator;
use crate::population::*;
use crate::genetic::*;


pub struct GeneticAlgorithm<P, F, M, C, S, R, RNG> 
{
    fitness: F,
    mutate: M,
    crossover: C,
    select: S,
    reinsert: R,
    rng: RNG,
    population: P
}


impl<P, F, M, C, S, R, RNG> GeneticAlgorithm<P, F, M, C, S, R, RNG> 
    where 
        P:Population,
        F:FitnessFunction<Genotype = P::Genotype, Fitness = P::Fitness> + 'static
{
    pub fn new(
        fitness: F,
        mutate: M,
        crossover: C,
        select: S,
        reinsert: R,
        rng: RNG,
        initial_population: Vec<P::Genotype>
    ) -> Self {
        let population = P::from_genomes(initial_population, &fitness);
        Self {
            fitness,
            mutate,
            crossover,
            select,
            reinsert,
            rng,
            population
        }
    }
}


impl<P, F, M, C, S, R, RNG> Algorithm for GeneticAlgorithm<P, F, M, C, S, R, RNG> 
    where
        P:Population,
        F:FitnessFunction<Genotype = P::Genotype, Fitness = P::Fitness> + 'static,
        M:MutateOperator<P::Genotype>,
        C:CrossoverOperator<P::Genotype>,
        S:SelectOperator,
        R:ReinsertOperator,
        RNG: Rng,

{
    type Population = P;

    fn advance_evolution(&mut self) -> Result<()> {
        let matings = self.select.select_from(self.population.clone(), &mut self.rng)?;
        let mut offspring = Vec::new();

        for mating_result in matings.into_iter().map(|p| self.crossover.crossover(p, &mut self.rng)) {
            offspring.extend(mating_result?)
        };

        for child in offspring.iter_mut() {
            self.mutate.mutate(child, &mut self.rng)?;
        };

        let future_individuals = P::from_genomes(offspring, &self.fitness);        

        self.population = self.reinsert
            .reinsert(self.population.clone(), future_individuals, &mut self.rng)?; 

        Ok(())

    }

    fn last_population(&self) -> Self::Population {
        self.population.clone()
    }
}


pub struct AlgorithmIterator<'a, A: Algorithm> {
    algorithm: &'a mut A
}


impl<'a, A: Algorithm> AlgorithmIterator<'a, A> {
    pub fn new(algorithm: &'a mut A) -> Self {
        Self {
            algorithm
        }
    }
}


impl<'a, A:Algorithm> Iterator for AlgorithmIterator<'a, A> {
    type Item = Result<A::Population>;

    fn next(&mut self) -> Option<Self::Item> {
        let evolution_result = self.algorithm.advance_evolution();
        match evolution_result {
            Ok(()) => Some(Ok(self.algorithm.last_population())),
            Err(e) => Some(Err(e)),
        }
    }
}


pub trait Algorithm {
    type Population: Population;

    fn advance_evolution(&mut self) -> Result<()>;
    fn last_population(&self) -> Self::Population;
}


// pub trait Algorithm: Iterator {
//     type Individual: Individual;

//     fn advance_evolution(&mut self) -> 
// }


// // pub struct GeneticAlgorithm<I, F, M, C, S, R, RNG>
//     where I: Individual
// {

//     fitness_function: F,
//     mutate: M,
//     crossover: C,
//     select: S,
//     reinsert: R,
//     rng: RNG,

//     population: Individuals<I>,
// }


// impl<I, F, A, M, C, S, R, RNG> GeneticAlgorithm<I, F, A, M, C, S, R, RNG>
//     where
//         I: RankedIndividual,
//         F: FitnessFunction<Genotype=I::Genotype, Fitness=I::Fitness, Constraint=I::Constraint> + 'static,
//         A: AdvantageFunction<Fitness=I::Fitness, Advantage=I::Advantage> + 'static,
//         M: MutateOperator<I::Genotype>,
//         C: CrossoverOperator<I::Genotype>,
//         S: SelectOperator<I>,
//         R: ReinsertOperator<I::Individual>,
//         RNG: Rng,
// {
//     pub fn new(
//         fitness_function: F,
//         advantage_function: A,
//         mutate: M,
//         crossover: C,
//         select: S,
//         reinsert: R,
//         rng: RNG,
//         initial_pool: Vec<I::Genotype>,
//     ) -> Self {
//         let population = Individuals::from_genomes(initial_pool, &fitness_function);
//         Self {
//             fitness_function,
//             advantage_function,
//             mutate,
//             crossover,
//             select,
//             reinsert,
//             rng,
//             population,
//         }
//     }

//     pub fn advance_evolution(&mut self) -> Result<()> {
//         let ranked_individuals = RankedIndividuals
//             ::from_population(self.population.clone(), &self.advantage_function);
//         let matings = self.select.select_from(ranked_individuals, &mut self.rng)?;

//         let mut offspring = Vec::new();

//         for mating_result in matings.into_iter().map(|p| self.crossover.crossover(p, &mut self.rng)) {
//             offspring.extend(mating_result?)
//         }

//         for child in offspring.iter_mut() {
//             self.mutate.mutate(child, &mut self.rng)?;
//         }

//         let mut future_individuals = 
//             Individuals::from_genomes(offspring, &self.fitness_function);

//         self.population = self.reinsert
//             .reinsert(self.population.clone(), future_individuals, &mut self.rng)?;
        
//         Ok(())
//     }
// }


// pub trait Algorithm: Iterator {}


// impl<I, F, M, C, S, R, RNG> Algorithm for GeneticAlgorithm<I, F, M, C, S, R, RNG>
//     where
//         I: RankedIndividual,
//         F: FitnessFunction<Genotype=I::Genotype, Fitness=I::Fitness, Constraint=I::Constraint> + 'static,
//         M: MutateOperator<I::Genotype>,
//         C: CrossoverOperator<I::Genotype>,
//         S: SelectOperator<I>,
//         R: ReinsertOperator<I::Individual>,
//         RNG: Rng,
// {}


// impl<I, F, M, C, S, R, RNG> Iterator for GeneticAlgorithm<I, F, M, C, S, R, RNG>
//     where
//         I: RankedIndividual,
//         F: FitnessFunction<Genotype=I::Genotype, Fitness=I::Fitness, Constraint=I::Constraint> + 'static,
//         M: MutateOperator<I::Genotype>,
//         C: CrossoverOperator<I::Genotype>,
//         S: SelectOperator<I>,
//         R: ReinsertOperator<I::Individual>,
//         RNG: Rng,
// {
//     type Item = Individuals<I::Individual>;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.advance_evolution().unwrap();
//         Some(self.population.clone())
//     }
// }
