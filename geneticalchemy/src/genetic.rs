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
    constraints: Vec<Box<dyn AlchemyFitnessElement>>,
    grimoire: OptimizedGrimoir,
}


impl AlchemyFitnessFunction {
    pub fn new(
        grimoire: OptimizedGrimoir,
        elements: Vec<Box<dyn AlchemyFitnessElement>>,
        constraints: Vec<Box<dyn AlchemyFitnessElement>>,
    ) -> Self {
        Self { grimoire, elements, constraints }
    }

    fn get_mix(&self, genome: &AlchemyGenome) -> Mix {
        Mix {
            ingredients: genome.iter().map(|gene|(
                self.grimoire.ingredients[gene.ingredient_index].clone(),
                gene.amount
            )).collect(),
            advanced_potion_making_mod: self.grimoire.advanced_potion_making_mod,
        }
    }    
}


impl FitnessFunction for AlchemyFitnessFunction {
    type Genotype = AlchemyGenome;
    type Fitness = AlchemyFitness;
    type Constraint = AlchemyConstraint;

    fn fitness(&self, genome: &Self::Genotype) -> Self::Fitness {
        let mix = self.get_mix(genome);
        self.elements.iter().map(|element| element.fitness(&mix)).collect()
    }

    fn constraint(&self, genome: &Self::Genotype) -> Self::Constraint {
        let mix = self.get_mix(genome);
        self.constraints.iter().map(
            |element| element.fitness(&mix)).map(|x| NotNan::new(x).unwrap()).collect()
    }
}
