//! The PrecedencePreservativeCrossover struct is a type of genetic operator that is used to combine
//! the genetic material of multiple parent individuals to produce new offspring.
//! `num_children` specifies the number of children that will be produced by the crossover operation.
//! The algorithm ensures that the resulting child's genome will have non-repeating genes.
//!
//! This is accomplished by first creating a selection_table that specifies which parent's genetic
//! material should be used at each position in the child's genome.
//!
//! The selection_table is created by randomly selecting a parent for each position in the child's
//! genome. Once the selection_table has been created, the crossover method iterates over each
//! position in the child's genome and uses the select_gene method to select the genetic material
//! to be used at that position. The select_gene method uses the selection_table to determine which
//! parent's genetic material should be used at the current position, and then selects the gene from
//! that parent using a precedence-preserving crossover strategy. This means that the select_gene
//! method will prioritize selecting genetic material from the parent specified in the
//! selection_table for the current position, but if that parent does not have any genetic material
//! available at that position (because the gene has already been used in the child's genome), the
//! select_gene method will search to the left in the selection_table to find another parent that
//! does have genetic material available at that position. This process continues until a gene is
//! found, and that gene is then used in the child's genome. This process is repeated for each
//! position in the child's genome, resulting in a new child individual with a genome that is a
//! combination of the genetic material of the parent individuals, and where each gene in the
//! child's genome is unique and not repeated.
//!
//! Note: Each parent must have non-repeating genes, otherwise the algorithm will likely fail.

use std::cell::*;

use error_stack::*;
use rand::Rng;

use crate::{
    error::{Result, *},
    genetic::*,
    op::*,
};

pub struct PrecedencePreservativeCrossover {
    /// Number of children produced by the algorithm
    num_children: usize,
}

type SelectionTable = Vec<usize>;
type CrossoverStateRow<G> = Vec<Option<G>>;
type CrossoverState<G> = Vec<CrossoverStateRow<G>>;

trait CrossoverStateRowTrait {
    type Locus;

    fn search_left(&self, index: usize) -> Option<Self::Locus>;
    fn remove_gene(&mut self, gene: &Self::Locus);
    fn from_genes(src: impl IntoIterator<Item = Self::Locus>) -> Self;
}

impl<L: Locus + Eq> CrossoverStateRowTrait for CrossoverStateRow<L> {
    type Locus = L;

    fn search_left(&self, index: usize) -> Option<Self::Locus> {
        (0..=index).rev().find_map(|i| self[i].clone())
    }

    fn remove_gene(&mut self, gene: &Self::Locus) {
        self.iter_mut().for_each(|x| {
            if x.is_some() && x.as_ref().unwrap() == gene {
                *x = None
            }
        })
    }

    fn from_genes(src: impl IntoIterator<Item = Self::Locus>) -> Self {
        src.into_iter().map(Some).collect()
    }
}

trait CrossoverStateTrait {
    type Locus;
    type Row;

    fn remove_gene(&mut self, gene: &Self::Locus);
    fn from_genes(src: Vec<VectorEncoded<Self::Locus>>) -> Self;
    fn select_with_table(&self, matrix: &SelectionTable, index: usize) -> Option<Self::Locus>;
}

impl<L> CrossoverStateTrait for CrossoverState<L>
where
    L: Locus + Eq,
{
    type Locus = L;
    type Row = Vec<Option<L>>;

    fn remove_gene(&mut self, gene: &Self::Locus) {
        self.iter_mut().for_each(|x| x.remove_gene(gene));
    }

    fn from_genes(src: Vec<VectorEncoded<Self::Locus>>) -> Self {
        src.into_iter().map(Self::Row::from_genes).collect()
    }

    fn select_with_table(&self, table: &SelectionTable, index: usize) -> Option<Self::Locus> {
        self[table[index]].search_left(index)
    }
}

impl PrecedencePreservativeCrossover {
    pub fn new(num_children: usize) -> Self {
        Self { num_children }
    }

    fn crossover_with_table<L>(
        &mut self,
        parents: Vec<VectorEncoded<L>>,
        table: SelectionTable,
    ) -> Result<Vec<VectorEncoded<L>>>
    where
        L: Locus + Eq,
    {
        let mut children = Vec::new();

        for _ in 0..self.num_children {
            let child = self.create_child(&parents, &table)?;
            children.push(child);
        }

        Ok(children)
    }

    fn create_child<L>(
        &mut self,
        parents: &[VectorEncoded<L>],
        selection_table: &SelectionTable,
    ) -> Result<VectorEncoded<L>>
    where
        L: Locus + Eq,
    {
        let mut child: VectorEncoded<L> = VectorEncoded::default();

        let dna_size = parents[0].len();

        let mut crossover_state = CrossoverState::from_genes(parents.to_vec());

        for i in 0..dna_size {
            let maybe_gene = crossover_state.select_with_table(selection_table, i);

            let gene = match maybe_gene {
                Some(x) => x,
                None => {
                    return Err(Report::new(Error::GeneticError)).attach_printable_lazy(|| {
                        "PrecedencePreservativeCrossover received \
                        a parent that had non-unique genes"
                    })
                }
            };

            crossover_state.remove_gene(&gene);

            child.push(gene);
        }

        Ok(child)
    }
}

impl<L> CrossoverOperator<VectorEncoded<L>> for PrecedencePreservativeCrossover
where
    L: Locus + Clone + Eq,
{
    fn crossover<R: Rng>(
        &mut self,
        parents: Vec<VectorEncoded<L>>,
        rng: &mut R,
    ) -> Result<Vec<VectorEncoded<L>>> {
        let dna_size = parents[0].len();
        let num_parents = parents.len();

        let selection_table = selection_table(rng, dna_size, num_parents);
        self.crossover_with_table(parents, selection_table)
    }
}

fn selection_table<R: Rng>(rng: &mut R, num_cols: usize, num_rows: usize) -> SelectionTable
where
    R: Rng,
{
    (0..num_cols).map(|_| rng.gen_range(0..num_rows)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Locus for i32 {}

    #[test]
    fn test_precedence_preservative_crossover() {
        let rng = rand::thread_rng();
        let mut op = PrecedencePreservativeCrossover::new(1);

        // Create two parent sequences
        let parents = vec![vec![0, 5, 9, 2, 6, 3, 4], vec![0, 1, 2, 5, 4, 7, 6]];

        // Create a selection table indicating which parent element to choose at each index
        let selection_table = vec![0, 1, 1, 0, 1, 1, 0];

        // Calculate the expected output sequence
        let expected = vec![0, 1, 2, 9, 4, 7, 3];

        // Perform the crossover operation and retrieve the resulting sequence
        let actual = op
            .crossover_with_table(parents, selection_table)
            .unwrap()
            .remove(0);

        // Check that the expected and actual sequences match
        assert_eq!(expected, actual);
    }
}
