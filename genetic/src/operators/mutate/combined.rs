use crate::op::*;
use crate::genetic::*;
use crate::error::*;


pub struct Mutators<G> {
    mutators: Vec<Box<dyn MutateOperator<G>>>
}


impl<G> Mutators<G> {
    pub fn new(mutators: Vec<Box<dyn MutateOperator<G>>>) -> Self {
        Self { mutators }
    }
}


impl<G> MutateOperator<G> for Mutators<G>
    where
        G: Genotype,
{
    fn mutate(&mut self, genome: &mut G) -> Result<()> {
        for mutator in self.mutators.iter_mut() {
            mutator.mutate(genome)?;
        };
        Ok(())
    }
}