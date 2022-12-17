use rand::{prelude::{IteratorRandom, Rng}, seq::index::sample};
use std::cmp::min;

use genetic::genetic::VectorEncoded;
use grimoire2::prelude::OptimizedGrimoire;


use crate::gene::AlchemyGene;

pub type AlchemyGenome = VectorEncoded<AlchemyGene>;

pub trait RandomizingGenome {
    fn create_random<R: Rng>(rng: &mut R, num_ingredients: usize) -> Self;
}

impl RandomizingGenome for AlchemyGenome {
    fn create_random<R: Rng>(rng: &mut R, num_ingredients: usize) -> Self {
        let genome_len = min(num_ingredients, 16);
        let grimoire_size = num_ingredients;
        let selected_ingredients = (0..grimoire_size).choose_multiple(rng, genome_len);
        selected_ingredients
            .into_iter()
            .map(|x| AlchemyGene {
                amount: rng.gen_range(0..10),
                ingredient_index: x,
            })
            .collect()
    }
}


#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use rand::{prelude::SmallRng, SeedableRng, thread_rng};

    use super::{AlchemyGenome, RandomizingGenome};

    #[test]
    fn test_create_random_uniques() {
        for _ in 0..20 {
            let mut rng = thread_rng();
            let genome = AlchemyGenome::create_random(&mut rng, 20);
            let ingredients: HashSet<usize> = genome.iter().map(|x| x.ingredient_index).collect();
            assert!( ingredients.len() == genome.len(), "Genome contains non-unique elements" );
        }
    }

    #[test]
    fn test_create_random() {
        let mut rng = SmallRng::seed_from_u64(0);
        let genome = AlchemyGenome::create_random(&mut rng, 100);
        let expected_ingredients = vec![36, 33, 60, 3, 48, 22, 27, 61, 92, 50, 10, 11, 95, 93, 39, 99];
        let expected_amounts = vec![2, 6, 9, 9, 5, 6, 7, 9, 9, 2, 0, 7, 0, 9, 1, 5];

        let (actual_ingredients, actual_amounts): (Vec<usize>, Vec<u64>) = genome
            .iter().map(|x| (x.ingredient_index, x.amount)).unzip();

        assert_eq!(
            actual_ingredients, 
            expected_ingredients, 
            "Ingredient indices dont match"
        );

        assert_eq!(
            actual_amounts, 
            expected_amounts, 
            "Ingredient amounts dont match"
        );       
    }
}