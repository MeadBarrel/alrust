use ordered_float::NotNan;

pub use genetic::prelude::{Constraint, FitnessFunction, ParettoFitness};
pub use grimoire2::prelude::{Mix, OptimizedGrimoire};

pub use crate::genome::AlchemyGenome;

pub type AlchemyConstraint = Vec<NotNan<f64>>;
pub type AlchemyFitness = ParettoFitness;

pub trait AlchemyFitnessElement {
    fn fitness(&self, mix: &Mix) -> f64;
}

pub struct AlchemyFitnessFunction {
    elements: Vec<Box<dyn AlchemyFitnessElement>>,
    desired_volume: f64,
    grimoire: OptimizedGrimoire,
}

impl AlchemyFitnessFunction {
    pub fn new(
        grimoire: OptimizedGrimoire,
        elements: Vec<Box<dyn AlchemyFitnessElement>>,
        desired_volume: f64,
    ) -> Self {
        Self {
            grimoire,
            elements,
            desired_volume,
        }
    }

    fn get_mix(&self, genome: &AlchemyGenome) -> Mix {
        let ingredients = genome.iter().cloned().map(|x| x.into()).collect();
        Mix::new(&self.grimoire, ingredients)
    }
}

impl FitnessFunction for AlchemyFitnessFunction {
    type Genotype = AlchemyGenome;
    type Fitness = AlchemyFitness;

    fn fitness(&self, genome: &Self::Genotype) -> Self::Fitness {
        let mix = self.get_mix(genome);
        self.elements
            .iter()
            .map(|element| NotNan::new(element.fitness(&mix)).unwrap())
            .collect()
    }

    fn constraint(&self, genome: &Self::Genotype) -> Constraint {
        let mix = self.get_mix(genome);
        NotNan::new(-(mix.volume() - self.desired_volume).abs()).unwrap()
    }
}
