use crate::genetic::*;
use crate::error::*;

pub trait MutateOperator<G> 
    where G: Genotype
{
    fn mutate(&mut self, genome: G) -> Result<G>;
}


pub trait SelectOperator<I>
    where I: RankedIndividual
{
    fn select_from(&mut self, individuals: Vec<I>) -> Result<Vec<I>>;
}


pub trait CrossoverOperator<G>
    where G: Genotype
{
    fn crossover(&mut self, genomes: Vec<G>) -> Result<Vec<G>>;
}


pub trait ReinsertOperator<I>
    where I: Individual
{
    fn reinsert(&mut self, individuals: Vec<I>) -> Result<Vec<I>>;
}
