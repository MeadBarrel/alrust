use rand::Rng;
use crate::{
    op::*,
    genetic::*,
    error::*,
    population::*, alias::{Individuals, AdvantageFunctionAlias, RankedIndividuals},
};
use crate::individual::{Individual, RankedIndividual, RankedIndividualStruct};
use crate::prelude::IndividualStruct;


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


impl<I, A> ReinsertOperator<I> for ElitistReinserter<I::Fitness, A>
    where
        I: Individual,
        A: Advantage,
{
    fn reinsert<R: Rng>(
            &mut self, 
            mut current: Individuals<I>,
            offspring: Individuals<I>,
            _: &mut R
    ) -> Result<Individuals<I>> {
        let target_len = current.len();
        current.extend(offspring);
        let mut individuals_ranked= RankedIndividuals::<RankedIndividualStruct<I, A>>::from_population(
            current, self.advantage_function.as_ref());
        individuals_ranked.sort_by_key(|x| (x.individual().constraint().clone(), x.advantage.clone()));
        individuals_ranked.reverse();
        individuals_ranked.truncate(target_len);
        Ok(individuals_ranked.into_iter().map(|x| x.individual).collect())
                
    }

}