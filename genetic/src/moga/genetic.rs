use std::cmp::Ordering;

use ordered_float::NotNan;
use serde::Serialize;

use crate::{paretto::*, genetic::*};


pub type ParettoFitness = Features;
impl Fitness for ParettoFitness {}


#[derive(PartialEq, Eq, Clone, Debug, Serialize)]
pub struct ParettoAdvantage {
    rank: u64,
    #[serde(skip)]
    crowding_distance: NotNan<f64>,
}

#[derive(Default)]
pub struct ParettoAdvantageFunction {}

impl Advantage for ParettoAdvantage {}


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


impl AdvantageFunction for ParettoAdvantageFunction
{
    type Fitness = ParettoFitness;
    type Advantage = ParettoAdvantage;

    fn call(&self, fitnesses: Vec<&Self::Fitness>) -> Vec<Self::Advantage> {
        let fitnesses_owned: Vec<ParettoFitness> = fitnesses.into_iter().cloned().collect();
        let advantages = paretto_assess(&fitnesses_owned);
        advantages.into_iter().map(|(r, d)| ParettoAdvantage::new(r, d)).collect()
    }

}