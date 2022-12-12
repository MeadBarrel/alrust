use rand::prelude::Rng;

use crate::genetic::*;
use crate::error::*;
use crate::alias::*;
use crate::individual::{Individual, RankedIndividual};


/// The trait defines a type of interface for types that can mutate a `Genotype`.
///
/// ```
/// use rand::Rng;
/// use genetic::op::MutateOperator;
/// use genetic::error::Result;
///
/// struct MyMutationOperator;
///
/// impl MutateOperator<MyGenotype> for MyMutationOperator {
///     fn mutate<R: Rng>(&mut self, genome: &mut G, rng: &mut R) -> Result<()> {
///         // Mutate the genome in some way
///         // ...
///         Ok(())
///     }
/// }
/// ```
pub trait MutateOperator<G> 
    where G: Genotype
{
    fn mutate<R: Rng>(&mut self, genome: &mut G, rng: &mut R) -> Result<()>;
}


pub trait SelectOperator<I>
    where
        I: RankedIndividual,
{
    fn select_from<R: Rng>(
        &mut self, individuals: RankedIndividuals<I>, rng: &mut R) -> Result<Matings<I::Genotype>>;
}


pub trait CrossoverOperator<G>
    where G: Genotype
{
    fn crossover<R: Rng>(&mut self, genomes: Vec<G>, rng: &mut R) -> Result<Vec<G>>;
}


pub trait ReinsertOperator<I>
    where
        I: Individual
{
    fn reinsert<R>(
        &mut self, 
        current: Individuals<I>,
        offspring: Individuals<I>,
        rng: &mut R,
    ) -> Result<Individuals<I>>
    where
        R: Rng,
    ;
}
