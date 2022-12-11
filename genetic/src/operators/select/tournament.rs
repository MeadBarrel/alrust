use std::cell::RefCell;

use crate::{
    genetic::*,
    op::*,
    error::*,
    alias::*,
};

use rand::prelude::*;
use serde::Deserialize;
use crate::prelude::{Individual, RankedIndividual};


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IndexedRank<A: Advantage> {
    pub index: usize,
    pub advantage: A
}

#[derive(Deserialize, Clone)]
#[serde(default)]
pub struct TournamentConfig {
    tournament_size: usize,
    probability: f64,
    num_parents: usize,
    num_matings: usize,
    remove_selected: bool,
}


impl Default for TournamentConfig {
    fn default() -> Self {
        TournamentConfig {
            tournament_size: 25,
            probability: 0.1,
            num_parents: 2,
            num_matings: 25,
            remove_selected: false,
        }
    }
}


impl TournamentConfig {
    pub fn new(tournament_size: usize, probability: f64, num_parents: usize, num_matings: usize, remove_selected: bool) -> Self {
        Self {
            tournament_size,
            probability,
            num_parents,
            num_matings,
            remove_selected,
        }
    }
}


pub struct TournamentSelector {
    config: TournamentConfig,
}



impl TournamentSelector
{
    pub fn new(
        config: TournamentConfig,
    ) -> Self {
        Self {
            config,
        }
    }

    /// Given a Vector, randomly choose items based on their position in the
    /// vector. First item will have the highest probability, second a bit lower,
    /// and so until the last item which will have the lowest probability to be
    /// selected.    
    fn choose<T: Copy, R: Rng>(&mut self, vec: Vec<T>, amt: usize, rng: &mut R) -> Vec<T>
    {
        let p = self.config.probability;
        let weights: Vec<f64> = 
            (0..vec.len())
            .scan(1., |acc, _| {*acc*=1.-p; Some(1.**acc)})
            .collect();
        let indices = indices(&vec);
        let indices = indices.choose_multiple_weighted(rng, amt, |i| weights[*i]).unwrap();
        indices.into_iter().map(|i| vec[*i]).collect()
    }
}


impl<I> SelectOperator<I> for TournamentSelector
    where I: RankedIndividual,
{
    fn select_from<R: Rng>(&mut self, individuals: RankedIndividuals<I>, rng: &mut R) -> Result<Matings<I::Genotype>> {
        let mut result = Vec::with_capacity(self.config.num_matings);
        let mut indices_ranked: Vec<(usize, &I::Advantage, &I::Constraint)> = individuals
            .iter().enumerate().map(|(i, c)| (i, c.advantage(), c.individual().constraint())).collect();

        for _ in 0..self.config.num_matings {
            let mut sample: Vec<&(usize, &I::Advantage, &I::Constraint)> = indices_ranked
                .choose_multiple::<R>(rng, self.config.tournament_size)
                .collect();

            sample.sort_by_key(|&&x| (x.2, x.1));
            sample.reverse();

            let mut chosen_parents: Vec<usize> = sample.into_iter().map(|x| x.0).collect();

            chosen_parents = self.choose(chosen_parents, self.config.num_parents, rng);

            let parents = chosen_parents.into_iter().map(
                |i| {
                    if self.config.remove_selected { indices_ranked.retain(|x| x.0 != i); };
                    individuals[i].individual().genotype().clone()
                }
            ).collect();

            result.push(parents);
        }

        Ok(result)
    }

}


fn indices<T>(vec: &Vec<T>) -> Vec<usize>
{
    (0..vec.len()).collect()
}




#[cfg(test)]
pub mod tests {
    use rand::prelude::*;

    use crate::{genetic::*, individual::{RankedIndividual, Individual}};
    use crate::prelude::{IndividualStruct, RankedIndividualStruct};
    use super::*;

    impl Genotype for u64 {}
    impl Advantage for u64 {}
    impl Fitness for i32 {}
    impl Constraint for i32 {}

    #[test]
    fn test_choose()
    {
        let mut rng = SmallRng::seed_from_u64(0);
        let vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let tournament_config = TournamentConfig::new(0, 0.9, 0, 0, false);
        let mut tournament = TournamentSelector::new(tournament_config);

        let results = tournament.choose(vec, 4, &mut rng);
        
        let expected = [0, 2, 1, 6];

        assert_eq!(results, expected);
    }

    #[test]
    fn test_select_from_noremove()
    {
        let mut rng = SmallRng::seed_from_u64(0);
        let tournament_config = TournamentConfig::new(10, 0.5, 2, 2, false);
        let mut tournament = TournamentSelector::new(tournament_config);


        // let candidates = vec![
        //     438, 431, 963, 454, 883, 929, 580, 450, 390, 810, 238, 968, 640, 54, 
        //     916, 224, 231, 296, 787, 724];
        let candidates = vec![29, 71, 95, 87, 97, 0, 35, 25, 53, 91, 97, 43, 99, 100, 28, 92, 40, 45, 43, 5];
        

        let ranked_candidates = candidates
            .into_iter()
            .map(|x| RankedIndividualStruct::new(IndividualStruct::new(x, 0, 0), x))
            .collect();

        let actual = tournament.select_from(ranked_candidates, &mut rng).unwrap();
        let expected = vec![[100, 43], [100, 97]];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_select_from_remove()
    {
        let mut rng = SmallRng::seed_from_u64(0);
        let tournament_config = TournamentConfig::new(10, 0.5, 2, 2, true);
        let mut tournament = TournamentSelector::new(tournament_config);


        let candidates = vec![
            438, 431, 963, 454, 883, 929, 580, 450, 390, 810, 238, 968, 640, 54, 
            916, 224, 231, 296, 787, 724];

        let ranked_candidates = candidates
            .into_iter()
            .map(|x| RankedIndividualStruct::new(IndividualStruct::new(x, 0, 0), x))
            .collect();

        let actual = tournament.select_from(ranked_candidates, &mut rng).unwrap();
        let expected = vec![[968, 883], [929, 454]];

        assert_eq!(actual, expected);
    }

}