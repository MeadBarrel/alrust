use serde::Serialize;


use crate::error::*;


pub trait Genotype {}
pub trait Phenotype: Serialize {}
pub trait Fitness {}
pub trait Constraint: Ord {}
pub trait Advantage: Ord + Eq {}

pub trait Population<G> 
{
    type Individual: Individual;
    type IntoIterator: IntoIterator<Item = Self::Individual>;

    fn individuals(&self) -> Self::IntoIterator;
    fn derive(&self, individuals: impl Iterator<Item=Self::Individual>) -> Self;
    fn best(&self) -> Option<Self::Individual>;
}

pub trait RankedPopulation {
    type Individual: RankedIndividual;
    type IntoIterator: IntoIterator<Item = Self::Individual>;

    fn derive(&self, individuals: impl Iterator<Item = Self::Individual>) -> Self;
    fn individuals(&self) -> Self::IntoIterator;
    fn best(&self) -> Option<Self::Individual>;
}


pub trait Incubator {
    type Genotype: Genotype;
    type Phenotype: Phenotype;

    fn grow(&self, genome: &Self::Genotype) -> Self::Phenotype;
}

pub trait FitnessFunction {
    type Phenotype: Phenotype;
    type Fitness: Fitness;

    fn call(&self, phenotype: Self::Phenotype) -> Result<Self::Fitness>;
}

pub trait ConstraintFunction {
    type Phenotype: Phenotype;
    type Constraint: Constraint;

    fn call(&self, phenotype: Self::Phenotype) -> Result<Self::Constraint>;
}

pub trait AdvantageFunction {
    type Fitness: Fitness;
    type Advantage: Advantage;

    fn call(&self, fitnesses: Vec<&Self::Fitness>) -> Vec<Self::Advantage>;
}

pub trait Individual {
    type Fitness: Fitness;
    type Genotype: Genotype;
    type Constraint: Constraint;

    fn fitness(&self) -> &Self::Fitness;
    fn genome(&self) -> &Self::Genotype;
    fn constraint(&self) -> &Self::Constraint;
}


pub trait RankedIndividual {
    type Individual: Individual;
    type Advantage: Advantage;

    fn individual(&self) -> &Self::Individual;
    fn advantage(&self) -> &Self::Advantage;
}


pub struct IndividualStruct<G, F, C> {
    genotype: G,
    fitness: F,
    constraints: C,
}


pub struct RankedIndividualStruct<I, A>
{
    individual: I,
    advantage: A,
} 


impl<G, F, C> IndividualStruct<G, F, C>
{
    pub fn new(genotype: G, fitness: F, constraints: C) -> Self {
        Self {
            genotype,
            fitness,
            constraints,
        }
    }
}


impl<I, A> RankedIndividualStruct<I, A> {
    pub fn new(individual: I, advantage: A) -> Self {
        Self {
            individual,
            advantage,
        }
    }
}


impl<G, F, C> Individual for IndividualStruct<G, F, C>
    where
        G: Genotype,
        F: Fitness,
        C: Constraint,
{
    type Fitness = F;
    type Genotype = G;
    type Constraint = C;

    fn fitness(&self) -> &Self::Fitness {
        &self.fitness
    }

    fn genome(&self) -> &Self::Genotype {
        &self.genotype
    }

    fn constraint(&self) -> &Self::Constraint {
        &self.constraints
    }
}


impl<I, A> RankedIndividual for RankedIndividualStruct<I, A>
    where
        I: Individual,
        A: Advantage,
{
    type Individual = I;
    type Advantage = A;

    fn individual(&self) -> &Self::Individual {
        &self.individual
    }

    fn advantage(&self) -> &Self::Advantage {
        &self.advantage
    }
}