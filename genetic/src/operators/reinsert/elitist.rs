use crate::{
    op::*,
    genetic::*,
    error::*,
    population::*, alias::{Individuals, AdvantageFunctionAlias, RankedIndividuals},
};


pub struct ElitistReinserter<F, A> {
    advantage_function: Box<AdvantageFunctionAlias<F, A>>
}


impl<F, A> ElitistReinserter<F, A> {
    pub fn new(advantage_function: Box<AdvantageFunctionAlias<F, A>>) -> Self {
        Self {
            advantage_function
        }
    }
}


impl<G, F, C, A> ReinsertOperator<G, F, C> for ElitistReinserter<F, A>
    where
        G: Genotype,
        F: Fitness,
        C: Constraint,
        A: Advantage,
{

    fn reinsert(&mut self, individuals: Individuals<G, F, C>) -> Result<Individuals<G, F, C>> {
        let target_len = individuals.len();
        let mut individuals_ranked = RankedIndividuals::from_population(
            individuals, self.advantage_function.as_ref());
        individuals_ranked.sort_by_key(|x| x.advantage.clone());
        individuals_ranked.reverse();
        individuals_ranked.truncate(target_len);
        Ok(individuals_ranked.into_iter().map(|x| x.individual).collect())
    }

}