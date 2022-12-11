use std::marker::PhantomData;
use serde::Serialize;

use crate::alias::*;
use crate::genetic::*;

// Individual --------------------------------------------------------------------------------------


#[derive(Debug, Clone)]
pub struct IndividualStruct<G, F, C> {
    pub genotype: G,
    pub fitness: F,
    pub constraints: C,
}

impl<G, F, C> IndividualStruct<G, F, C>
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

pub trait Individual: Clone {
    type Genotype: Genotype;
    type Fitness: Fitness;
    type Constraint: Constraint;

    fn genotype(&self) -> &Self::Genotype;

    fn fitness(&self) -> &Self::Fitness;

    fn constraint(&self) -> &Self::Constraint;

    fn from_genome(genome: Self::Genotype, fitness_function: &FitnessFunctionAlias<Self::Genotype, Self::Fitness, Self::Constraint>) -> Self;
}

impl<G, F, C> Individual for IndividualStruct<G, F, C>
    where
        G: Genotype,
        F: Fitness,
        C: Constraint
{
    type Genotype = G;
    type Fitness = F;
    type Constraint = C;

    fn genotype(&self) -> &Self::Genotype {
        &self.genotype
    }

    fn fitness(&self) -> &Self::Fitness {
        &self.fitness
    }

    fn constraint(&self) -> &Self::Constraint {
        &self.constraints
    }

    fn from_genome(genome: Self::Genotype, fitness_function: &FitnessFunctionAlias<Self::Genotype, Self::Fitness, Self::Constraint>) -> Self {
        IndividualStruct::<G, F, C>::from_genome(genome, fitness_function)
    }
}


// RankedIndividual --------------------------------------------------------------------------------


#[derive(Debug, Clone)]
pub struct RankedIndividualStruct<I, A>
{
    pub individual: I,
    pub advantage: A,
}

impl<I, A> RankedIndividualStruct<I, A>
    where
        I: Individual,
        A: Advantage,
{
    pub fn new(individual: I, advantage: A) -> Self {
        Self {
            individual,
            advantage,
        }
    }
}

pub trait RankedIndividual: Clone {
    type Individual: Individual<Genotype=Self::Genotype, Fitness=Self::Fitness, Constraint=Self::Constraint>;
    type Advantage: Advantage;
    type Genotype: Genotype;
    type Fitness: Fitness;
    type Constraint: Constraint;

    fn individual(&self) -> &Self::Individual;
    fn advantage(&self) -> &Self::Advantage;
    
    fn into_individual(self) -> Self::Individual;
    fn from_individual(individual: Self::Individual, advantage: Self::Advantage) -> Self;
}

impl<I, A> RankedIndividual for RankedIndividualStruct<I, A>
    where
        I: Individual,
        A: Advantage,
{
    type Individual = I;
    type Advantage = A;
    type Genotype = I::Genotype;
    type Fitness = I::Fitness;
    type Constraint = I::Constraint;

    fn individual(&self) -> &Self::Individual {
        &self.individual
    }

    fn advantage(&self) -> &Self::Advantage {
        &self.advantage
    }

    fn into_individual(self) -> Self::Individual {
        self.individual
    }

    fn from_individual(individual: Self::Individual, advantage: Self::Advantage) -> Self {
        Self {
            individual, advantage
        }
    }

}