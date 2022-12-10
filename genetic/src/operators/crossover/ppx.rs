use rand::Rng;
use error_stack::*;

use crate::{
    genetic::*,
    op::*,
    error::*,
};
use crate::error::Result;


pub struct PrecedencePreservativeCrossover<R: Rng> {
    rng: R,
    num_children: usize,
}


type SelectionTable = Vec<usize>;
type CrossoverStateRow<G> = Vec<Option<G>>;
type CrossoverState<G> = Vec<CrossoverStateRow<G>>;



trait CrossoverStateRowTrait {
    type Locus;

    fn search_left(&self, index: usize) -> Option<Self::Locus>;
    fn remove_gene(&mut self, gene: &Self::Locus);
    fn from_genes(src: impl IntoIterator<Item=Self::Locus>) -> Self;
}


impl<L: Locus + Eq> CrossoverStateRowTrait for CrossoverStateRow<L>
{
    type Locus = L;

    fn search_left(&self, index: usize) -> Option<Self::Locus> {
        (0..=index).rev().find_map(|i| self[i].clone())
    }

    fn remove_gene(&mut self, gene: &Self::Locus) {
        self.iter_mut().for_each(|x| if x.is_some() && x.as_ref().unwrap() == gene { *x = None })
    }

    fn from_genes(src: impl IntoIterator<Item=Self::Locus>) -> Self {
        src.into_iter().map(Some).collect()
    }
}



trait CrossoverStateTrait
{
    type Locus;
    type Row;

    fn remove_gene(&mut self, gene: &Self::Locus);
    fn from_genes(src: Vec<VectorEncoded<Self::Locus>>) -> Self;
    fn select_with_table(&self, matrix: &SelectionTable, index: usize) -> Option<Self::Locus>;
}


impl<L> CrossoverStateTrait for CrossoverState<L>
    where L: Locus + Eq
{
    type Locus = L;
    type Row = Vec<Option<L>>;

    fn remove_gene(&mut self, gene: &Self::Locus) {
        self.iter_mut().for_each(|x| x.remove_gene(gene));
    }

    fn from_genes(src: Vec<VectorEncoded<Self::Locus>>) -> Self 
    {
        src.into_iter().map(Self::Row::from_genes).collect()
    }

    fn select_with_table(&self, table: &SelectionTable, index: usize) -> Option<Self::Locus>
    {
        self[table[index]].search_left(index)
    }
}



fn selection_table<R>(rng: &mut R, num_cols: usize, num_rows: usize) -> SelectionTable
    where R: Rng
{
    (0..num_cols).map(|_| rng.gen_range(0..num_rows)).collect()
}


impl<R: Rng> PrecedencePreservativeCrossover<R>  {
    pub fn new(num_children: usize, rng: R) -> Self {
        Self {
            num_children,
            rng,
        }
    }

    fn crossover_with_table<L>(&mut self, parents: Vec<VectorEncoded<L>>, table: SelectionTable) -> Result<Vec<VectorEncoded<L>>> 
        where L: Locus + Eq
    {
        let mut children = Vec::new();

        for _ in 0..self.num_children {
            let child = self.create_child(&parents, &table)?;
            children.push(child);
        };

        Ok(children)
    }

    fn create_child<L>(&mut self, parents: &[VectorEncoded<L>], selection_table: &SelectionTable) -> Result<VectorEncoded<L>>
        where L: Locus + Eq
    {
        let mut child: VectorEncoded<L> = VectorEncoded::default();

        let dna_size = parents[0].len();

        let mut crossover_state = CrossoverState::from_genes(parents.to_vec());        

        for i in 0..dna_size {
            let maybe_gene = crossover_state.select_with_table(selection_table, i);

            let gene = match maybe_gene {
                Some(x) => x,
                None => {
                    return Err(Report::new(Error::GeneticError))
                        .attach_printable_lazy(|| "PrecedencePreservativeCrossover received \
                        a parent that had non-unique genes")
                }
            };

            crossover_state.remove_gene(&gene);

            child.push(gene);
        };

        Ok(child)
    }


}


impl<L, R> CrossoverOperator<VectorEncoded<L>> for PrecedencePreservativeCrossover<R>
    where 
        L: Locus + Clone + Eq,
        R: Rng
{  
    fn crossover(&mut self, parents: Vec<VectorEncoded<L>>) -> Result<Vec<VectorEncoded<L>>> {
        let dna_size = parents[0].len();
        let num_parents = parents.len();

        let selection_table = selection_table(&mut self.rng, dna_size, num_parents);
        self.crossover_with_table(parents, selection_table)
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    impl Locus for i32 {}

    #[test]
    fn test_precedence_preservative_crossover() {
        let rng = rand::thread_rng();
        let mut op = PrecedencePreservativeCrossover::new(1, rng);

        let parents = vec![
            vec![0, 5, 9, 2, 6, 3, 4],
            vec![0, 1, 2, 5, 4, 7, 6],
        ];

        let selection_table = vec![0, 1, 1, 0, 1, 1, 0];

        let expected = vec![0, 1, 2, 9, 4, 7, 3];
        let actual = op.crossover_with_table(parents, selection_table).unwrap().remove(0);
        
        assert_eq!( expected, actual );
    }

}