use grimoire::prelude::mix_volume;
use ordered_float::NotNan;

use genetic::prelude::*;
use grimoire::mix::Mix;
use grimoire::optimized::OptimizedGrimoir;


pub type AlchemyConstraint = Vec<NotNan<f64>>;


#[derive(Eq, Clone, Debug)]
pub struct AlchemyGene {
    pub ingredient_index: usize,
    pub amount: u64,
}


impl Locus for AlchemyGene {}


impl PartialEq for AlchemyGene {
    fn eq(&self, other: &AlchemyGene) -> bool {
        self.ingredient_index == other.ingredient_index
    }
}


pub type AlchemyGenome = VectorEncoded<AlchemyGene>;
pub type AlchemyFitness = ParettoFitness;


pub trait AlchemyFitnessElement {
    fn fitness(&self, mix: &Mix) -> f64;
}



pub struct AlchemyFitnessFunction {
    elements: Vec<Box<dyn AlchemyFitnessElement>>,
    desired_volume: f64,
    grimoire: OptimizedGrimoir,
}


impl AlchemyFitnessFunction {
    pub fn new(
        grimoire: OptimizedGrimoir,
        elements: Vec<Box<dyn AlchemyFitnessElement>>,
        desired_volume: f64,
    ) -> Self {
        Self { grimoire, elements, desired_volume }
    }

    fn get_mix(&self, genome: &AlchemyGenome) -> Mix {
        Mix {
            ingredients: genome.iter().map(|gene|(
                self.grimoire.ingredients[gene.ingredient_index].clone(),
                gene.amount
            )).collect(),
            advanced_potion_making_mod: self.grimoire.advanced_potion_making_mod,
            alvarin_clade: false,
        }
    }    
}


impl FitnessFunction for AlchemyFitnessFunction {
    type Genotype = AlchemyGenome;
    type Fitness = AlchemyFitness;

    fn fitness(&self, genome: &Self::Genotype) -> Self::Fitness {
        let mix = self.get_mix(genome);
        self.elements.iter().map(
            |element| NotNan::new(element.fitness(&mix)).unwrap()
        ).collect()
    }

    fn constraint(&self, genome: &Self::Genotype) -> Constraint {
        let mix = self.get_mix(genome);
        NotNan::new(-(mix_volume(&mix) - self.desired_volume).abs()).unwrap()
    }
}
