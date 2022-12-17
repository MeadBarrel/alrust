use rand::prelude::{IteratorRandom, Rng};
use std::cmp::min;

use genetic::genetic::VectorEncoded;
use grimoire2::prelude::OptimizedGrimoire;

use crate::gene::AlchemyGene;

pub type AlchemyGenome = VectorEncoded<AlchemyGene>;

pub trait RandomizingGenome {
    fn create_random<R: Rng>(rng: &mut R, grimoire: &OptimizedGrimoire) -> Self;
}

impl RandomizingGenome for AlchemyGenome {
    fn create_random<R: Rng>(rng: &mut R, grimoire: &OptimizedGrimoire) -> Self {
        let genome_len = min(grimoire.ingredients.len(), 16);
        let grimoire_size = grimoire.ingredients.len();
        let selected_ingredients = (0..genome_len).choose_multiple(rng, grimoire_size);
        selected_ingredients
            .into_iter()
            .map(|x| AlchemyGene {
                amount: rng.gen_range(0..10),
                ingredient_index: x,
            })
            .collect()
    }
}
