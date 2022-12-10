use crate::alias::*;
use crate::genetic::*;

#[derive(Debug, Clone)]
pub struct Individual<G, F, C> {
    pub genotype: G,
    pub fitness: F,
    pub constraints: C,
}


#[derive(Debug, Clone)]
pub struct RankedIndividual<G, F, C, A>
{
    pub individual: Individual<G, F, C>,
    pub advantage: A,
} 


impl<G, F, C> Individual<G, F, C>
    where
        G: Genotype,
        F: Fitness,
        C: Constraint,
{
    pub fn new(genotype: G, fitness: F, constraints: C) -> Self {
        Self {
            genotype,
            fitness,
            constraints,
        }
    }

    pub fn from_genome(
        genome: G, 
        fitness_function: &FitnessFunctionAlias<G, F,  C>
    ) -> Self {
        let fitness = fitness_function.fitness(&genome);
        let constraint = fitness_function.constraint(&genome);
        Self::new(genome, fitness, constraint )
    }
}


impl<G, F, C, A> RankedIndividual<G, F, C, A> {
    pub fn new(individual: Individual<G, F, C>, advantage: A) -> Self {
        Self {
            individual,
            advantage,
        }
    }

}
