use std::cell::RefCell;
use std::cmp::{max, min};
use rand::prelude::*;
use crate::genetic::*;
use genetic::{prelude::*, op::MutateOperator};


pub struct AlchemyMutator {
    grimoire_size: usize,
    amount_grow_ratio: f64,
    min_amount_grow: u64,
    num_mutations_amt: usize,
    num_mutations_ing: usize,
}


impl AlchemyMutator {
    pub fn new(
        grimoire_size: usize,
        amount_grow_ratio: f64, 
        min_amount_grow: u64,
        num_mutations_amt: usize,
        num_mutations_ing: usize,
    ) -> Self 
    {
        Self {
            grimoire_size,
            amount_grow_ratio,
            min_amount_grow,
            num_mutations_amt,
            num_mutations_ing
        }    
    }

    fn mutate_ingredients<R: Rng>(&mut self, genome: &mut AlchemyGenome, rng: &mut R) {
        let indices_to_mutate = (0..genome.len()).choose_multiple(
            rng, self.num_mutations_ing);
        for index in indices_to_mutate {
            let new_ingredient = rng.gen_range(0..self.grimoire_size);
            if genome.iter().any(|x| x.ingredient_index == new_ingredient) { continue; }
            genome[index].ingredient_index = new_ingredient;
        }
    }

    fn mutate_amounts<R: Rng>(&mut self, genome: &mut AlchemyGenome, rng: &mut R) {
        let mut genes_to_mutate = genome.iter_mut().choose_multiple(
            rng, self.num_mutations_amt);
        for gene in genes_to_mutate {
            let current_amt = gene.amount;
            let delta: u64 = max(
                self.min_amount_grow as u64, 
                ((rng.gen::<f64>() * current_amt as f64) * self.amount_grow_ratio) as u64
            );
            let reverse = rng.gen_bool(0.5);
            if reverse { gene.amount = gene.amount - min(gene.amount, delta) }
            else { gene.amount += delta }
        }        
    }
}


impl MutateOperator<AlchemyGenome> for AlchemyMutator {
    fn mutate<R: Rng>(&mut self, genome: &mut AlchemyGenome, rng: &mut R) -> Result<()> {
        self.mutate_ingredients(genome, rng);
        self.mutate_amounts(genome, rng);
        Ok(())
    }
}