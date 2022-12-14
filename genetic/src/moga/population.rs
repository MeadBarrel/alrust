use ordered_float::NotNan;
use std::ops::Index;
use std::cmp::Ordering;
use std::mem::take;


use crate::genetic::*;
use crate::paretto::paretto_assess;
use crate::prelude::IndividualStruct;
use crate::{population::*, prelude::Individual};


impl Fitness for Vec<NotNan<f64>> {}
pub type ParettoFitness = Vec<NotNan<f64>>;
pub type ParettoIndividual<G> = IndividualStruct<G, ParettoFitness>;



#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ParettoAdvantage {
    rank: u64,
    crowding_distance: NotNan<f64>,
}


impl Ord for ParettoAdvantage {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.rank.cmp(&other.rank) {
            Ordering::Equal => self.crowding_distance.cmp(&other.crowding_distance),
            Ordering::Greater => Ordering::Less,
            Ordering::Less => Ordering::Greater,
        }
    }
}


impl PartialOrd for ParettoAdvantage {
    fn partial_cmp(&self, other: &ParettoAdvantage) -> Option<Ordering> {
        match self.rank.partial_cmp(&other.rank) {
            Some(Ordering::Equal) => self.crowding_distance.partial_cmp(&other.crowding_distance),
            Some(Ordering::Greater) => Some(Ordering::Less),
            Some(Ordering::Less) => Some(Ordering::Greater),
            None => None,
        }
    }
}


impl ParettoAdvantage {
    pub fn new(rank: u64, crowding_distance: f64) -> Self
    {
        Self { 
            rank, 
            crowding_distance: NotNan::new(crowding_distance).unwrap(),
        }
    }

    pub fn rank(&self) -> u64 {
        self.rank
    }

    pub fn crowding_distance(&self) -> NotNan<f64> {
        self.crowding_distance
    }
}


#[derive(Clone)]
pub struct ParettoPopulation<I> {
    individuals: Vec<I>
}


impl<I> ParettoPopulation<I> 
    where I: Individual<Fitness = Vec<NotNan<f64>>>
{
    pub fn new(individuals: Vec<I>) -> Self {
        Self {
            individuals
        }
    }

    fn assess(&self) -> Vec<ParettoAdvantage> {
        let fitnesses_owned: Vec<Vec<f64>> = self.fitnesses()
            .into_iter()
            .cloned()
            .map(|v|
                v.into_iter().map(|x| x.into_inner()).collect()
            )
            .collect();
        let advantages = paretto_assess(&fitnesses_owned);
        advantages.into_iter().map(|(r, d)| ParettoAdvantage::new(r, d)).collect()        
    }        
}


impl<I> Index<usize> for ParettoPopulation<I> {
    type Output = I;

    fn index(&self, index: usize) -> &Self::Output {
        &self.individuals[index]
    }

}


impl<I> IntoIterator for ParettoPopulation<I> {
    type Item = I;
    type IntoIter = <Vec<I> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.individuals.into_iter()
    }
}


impl<I> Extend<I> for ParettoPopulation<I> 
    where I: Individual
{
    fn extend<T: IntoIterator<Item = I>>(&mut self, iter: T) {
        self.individuals.extend(iter)
    }

}


impl<I> Population for ParettoPopulation<I> 
    where I: Individual<Fitness = Vec<NotNan<f64>>>
{
    type Individual = I;
    type Genotype = I::Genotype;
    type Fitness = I::Fitness;

    fn derive(&self, individuals: Vec<Self::Individual>) -> Self {
        Self::new(individuals)       
    }

    fn len(&self) -> usize {
        self.individuals.len()
    }

    fn derive_ref(&self, individuals: Vec<&Self::Individual>) -> Self {
        Self::new(individuals.into_iter().cloned().collect())
    }

    fn from_genomes(
            genomes: Vec<Self::Genotype>, 
            fitness_function: &crate::alias::FitnessFunctionAlias<Self::Genotype, Self::Fitness>
        ) -> Self {
        Self::new(
            <Vec<I>>::from_genomes(genomes, fitness_function)
        )
    }

    fn fitnesses(&self) -> Vec<&Self::Fitness> {
        self.individuals.fitnesses()
    }

    fn individuals(&self) -> Vec<&Self::Individual> {
        self.individuals.individuals()
    }

    fn is_empty(&self) -> bool {
        self.individuals.is_empty()
    }

    fn remove(&mut self, index: usize) -> Self::Individual {
        self.individuals.remove(index)
    }

    fn truncate(&mut self, index: usize) {
        self.individuals.truncate(index)
    }

    fn sort(&mut self) {
        let advantages = self.assess();
        let individuals = take(&mut self.individuals);
        let mut ranked: Vec<(I, ParettoAdvantage)> = individuals.into_iter().zip(advantages.into_iter())
            .map(
                |(i, a)| (i, a)
            ).collect();
        ranked.sort_by_key(|(i, a)| (i.constraint(), a.clone()));
        ranked.reverse();
        self.individuals = ranked.into_iter().map(|(i, _)| i).collect();
    }

    fn best(&self) -> Option<&Self::Individual> {
        let advantages = self.assess();
        self.individuals.iter().enumerate()
            .max_by_key(|(i, ind)| (ind.constraint(), advantages[*i].clone())).map(|(_, i)| i)
    }

    fn n_best(&self, n: usize) -> Vec<&Self::Individual> {
        let advantages = self.assess();
        let mut ranked: Vec<(&I, ParettoAdvantage)> = self.individuals.iter().zip(advantages.into_iter())
            .map(
                |(i, a)| (i, a)
            ).collect();
        ranked.sort_by_key(|(i, a)| (i.constraint(), a.clone()));
        ranked.reverse();
        ranked.truncate(n);
        ranked.into_iter().map(|(i, _)| i).collect()
    }
}