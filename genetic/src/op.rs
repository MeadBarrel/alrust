use rand::prelude::Rng;

use crate::{alias::*, error::*, genetic::*, population::Population};

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
where
    G: Genotype,
{
    fn mutate<R: Rng>(&mut self, genome: &mut G, rng: &mut R) -> Result<()>;
}

pub trait SelectOperator {
    fn select_from<P: Population, R: Rng>(
        &mut self,
        population: P,
        rng: &mut R,
    ) -> Result<Matings<P::Genotype>>;
}

pub trait CrossoverOperator<G: Genotype> {
    fn crossover<R: Rng>(&mut self, genomes: Vec<G>, rng: &mut R) -> Result<Vec<G>>;
}

pub trait ReinsertOperator {
    fn reinsert<R, P: Population>(&mut self, current: P, offspring: P, rng: &mut R) -> Result<P>
    where
        R: Rng;
}
