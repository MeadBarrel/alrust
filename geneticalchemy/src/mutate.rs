use std::cell::RefCell;
use std::cmp::{max, min};
use rand::prelude::*;
use crate::genetic::*;
use genetic::{prelude::*, op::MutateOperator};


pub struct AlchemyMutator<R: Rng> {
    rng: RefCell<R>,
    grimoire_size: usize,
    amount_grow_ratio: f64,
    min_amount_grow: u64,
    num_mutations_amt: usize,
    num_mutations_ing: usize,
}


impl<R:Rng> AlchemyMutator<R> {
    pub fn new(
        rng: RefCell<R>, 
        grimoire_size: usize, 
        amount_grow_ratio: f64, 
        min_amount_grow: u64,
        num_mutations_amt: usize,
        num_mutations_ing: usize,
    ) -> Self 
    {
        Self {
            rng,
            grimoire_size,
            amount_grow_ratio,
            min_amount_grow,
            num_mutations_amt,
            num_mutations_ing
        }    
    }

    fn mutate_ingredients(&mut self, genome: &mut AlchemyGenome) {
        let indices_to_mutate = (0..genome.len()).choose_multiple(
            &mut *self.rng.borrow_mut(), self.num_mutations_ing);
        for index in indices_to_mutate {
            let new_ingredient = self.rng.borrow_mut().gen_range(0..self.grimoire_size);
            if genome.iter().any(|x| x.ingredient_index == new_ingredient) { continue; }
            genome[index].ingredient_index = new_ingredient;
        }
    }

    fn mutate_amounts(&mut self, genome: &mut AlchemyGenome) {
        let mut genes_to_mutate = genome.iter_mut().choose_multiple(
            &mut *self.rng.borrow_mut(), self.num_mutations_amt);
        for gene in genes_to_mutate {
            let current_amt = gene.amount;
            let delta: u64 = max(
                self.min_amount_grow as u64, 
                ((self.rng.borrow_mut().gen::<f64>() * current_amt as f64) * self.amount_grow_ratio) as u64
            );
            let reverse = self.rng.borrow_mut().gen_bool(0.5);
            if reverse { gene.amount = gene.amount - min(gene.amount, delta) }
            else { gene.amount += delta }
        }        
    }
}


impl<R:Rng> MutateOperator<AlchemyGenome> for AlchemyMutator<R> {
    fn mutate(&mut self, genome: &mut AlchemyGenome) -> Result<()> {
        self.mutate_ingredients(genome);
        self.mutate_amounts(genome);
        Ok(())
    }
}