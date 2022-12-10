use std::marker::PhantomData;


use serde::Serialize;

use crate::alias::*;
use crate::genetic::*;
use crate::error::*;


pub trait Incubator {
    type Genotype: Genotype;
    type Phenotype: Serialize;

    fn grow(&self, genome: &Self::Genotype) -> Self::Phenotype;
}


#[derive(Serialize)]
pub struct PrintableIndividual<A, S> {
    advantage: A,
    phenotype: S,
}


impl<A, S> PrintableIndividual<A, S> {
    pub fn new(phenotype: S, advantage: A) -> Self {
        PrintableIndividual {
            advantage,
            phenotype
        }
    }
}


pub trait PopulationSerializer<A, S> 
    where A: Advantage,
          S: Serialize,
{
    fn serialize(&self, population: Vec<PrintableIndividual<A, S>>) -> Result<String>;
    fn filename_extension(&self) -> &'static str;
}


pub trait PopulationHandler<G, F, C> {
    fn handle(&mut self, population: Individuals<G, F, C>, generation: usize) -> Result<()>;
}
