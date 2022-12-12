use serde::Serialize;

pub use crate::genetic::*;
pub use genetic::prelude::*;
pub use grimoire::prelude::*;


#[derive(Serialize, Clone, Debug)]
pub struct AlchemyEffectTotal {
    direct: f64,
    over_time: f64,
    per_second: f64,
    length: f64,
}


#[derive(Serialize, Clone, Debug)]
pub struct AlchemyPhenotypeEffects {
    volume: f64,
    dh: f64,
    dp: f64,
    hot: f64,
    hl: f64,
    pot: f64,
    pl: f64,
    a: f64,
}


#[derive(Serialize, Clone, Debug)]
pub struct AlchemyPhenotype {
    effects: AlchemyPhenotypeEffects,
    healing: AlchemyEffectTotal,
    poison: AlchemyEffectTotal,
    ingredients: Vec<(String, u64)>
}

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
    type Phenotype = AlchemyPhenotype;

    fn grow(&self, genome: &Self::Genotype) -> Self::Phenotype {
        let mix = Mix {
            ingredients: genome.iter().map(|gene|(
                self.grimoire.ingredients[gene.ingredient_index].clone(),
                gene.amount
            )).collect(),
            advanced_potion_making_mod: self.grimoire.advanced_potion_making_mod,
        };
        let effects = AlchemyPhenotypeEffects {
            volume: mix_volume(&mix),
            dh: mix_effect(&mix, Property::DirectHealing),
            dp: mix_effect(&mix, Property::DirectPoison),
            hot: mix_effect(&mix, Property::HealingOverTime),
            hl: mix_effect(&mix, Property::HealingLength),
            pot: mix_effect(&mix, Property::PoisonOverTime),
            pl: mix_effect(&mix, Property::PoisonLength),
            a: mix_effect(&mix, Property::Alcohol),
        };

        let healing = AlchemyEffectTotal {
            direct: effects.dh,
            per_second: effects.hot * effects.volume.sqrt(),
            over_time: effects.hot * effects.hl * effects.volume,
            length: effects.hl * effects.volume.sqrt(),
        };

        let poison = AlchemyEffectTotal {
            direct: effects.dp,
            per_second: effects.pot * effects.volume.sqrt(),
            over_time: effects.pot * effects.pl * effects.volume,
            length: effects.hl * effects.volume.sqrt(),
        };

        let ingredients = mix.ingredients.iter().map(|x| (x.0.name.clone(), x.1)).collect();
        AlchemyPhenotype {
            effects, ingredients, healing, poison
        }
    }
}