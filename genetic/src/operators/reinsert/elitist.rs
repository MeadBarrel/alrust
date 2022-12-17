use crate::{error::*, op::*, population::*};
use rand::Rng;

#[derive(Default)]
pub struct ElitistReinserter {}

impl ReinsertOperator for ElitistReinserter {
    fn reinsert<R: Rng, P: Population>(
        &mut self,
        mut current: P,
        offspring: P,
        _: &mut R,
    ) -> Result<P> {
        let target_len = current.len();
        current.extend(offspring);
        current.sort();
        current.truncate(target_len);
        Ok(current)
    }
}
