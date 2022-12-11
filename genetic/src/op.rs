use crate::genetic::*;
use crate::error::*;
use crate::alias::*;
use crate::individual::{Individual, RankedIndividual};

pub trait MutateOperator<G> 
    where G: Genotype
{
    fn mutate(&mut self, genome: &mut G) -> Result<()>;
}


pub trait SelectOperator<I>
    where
        I: RankedIndividual,
{
    fn select_from(&mut self, individuals: RankedIndividuals<I>) -> Result<Matings<I::Genotype>>;
}


pub trait CrossoverOperator<G>
    where G: Genotype
{
    fn crossover(&mut self, genomes: Vec<G>) -> Result<Vec<G>>;
}


pub trait ReinsertOperator<I>
    where
        I: Individual
{
    fn reinsert(
        &mut self, 
        current: Individuals<I>,
        offspring: Individuals<I>) -> Result<Individuals<I>>;
}
