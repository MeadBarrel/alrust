use rand::Rng;

use crate::genetic::*;
use crate::error::*;
use crate::alias::*;

pub trait MutateOperator<G> 
    where G: Genotype
{
    fn mutate(&mut self, genome: &mut G) -> Result<G>;
}


pub trait SelectOperator<G, F, C, A>
{
    fn select_from(&mut self, individuals: RankedIndividuals<G, F, C, A>) -> Result<Matings<G>>;
}


pub trait CrossoverOperator<G>
    where G: Genotype
{
    fn crossover(&mut self, genomes: Vec<G>) -> Result<Vec<G>>;
}


pub trait ReinsertOperator<G, F, C, A>
{
    fn reinsert(&mut self, individuals: Individuals<G, F, C>, advantage_function: &AdvantageFunctionAlias<F, A>) -> Result<Individuals<G, F, C>>;
}
