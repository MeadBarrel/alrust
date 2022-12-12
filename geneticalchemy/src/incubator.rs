use serde::Serialize;

pub use crate::genetic::*;
pub use genetic::prelude::*;
pub use grimoire::prelude::*;
pub struct AlchemyIncubator {
    grimoire: OptimizedGrimoir,
}


impl AlchemyIncubator {
    pub fn new(grimoire: OptimizedGrimoir) -> Self {
        Self { grimoire }
    }
}


impl Incubator for AlchemyIncubator {
    type Genotype = AlchemyGenome;
    type Phenotype = PotionSerializable;

    fn grow(&self, genome: &Self::Genotype) -> Self::Phenotype {
        let mix = Mix {
            ingredients: genome.iter().map(|gene|(
                self.grimoire.ingredients[gene.ingredient_index].clone(),
                gene.amount
            )).collect(),
            advanced_potion_making_mod: self.grimoire.advanced_potion_making_mod,
            alvarin_clade: false,
        };

        PotionSerializable::from_mix(&mix)
    }
}