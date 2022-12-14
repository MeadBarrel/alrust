use serde::Serialize;

use crate::genetic::*;


pub trait Incubator {
    type Genotype: Genotype;
    type Phenotype: Serialize;

    fn grow(&self, genome: &Self::Genotype) -> Self::Phenotype;
}

