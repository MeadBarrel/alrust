use geneticalchemy::prelude::AlchemyIndividual;
use serde::{Serialize, Deserialize};
use grimoire2::standalone::OptimizedGrimoire;
use genetic::prelude::ParettoPopulation;

#[derive(Serialize, Deserialize)]
pub struct IndividualSerializable {
    fitness: Vec<f64>,
    genome: Vec<(usize, u64)>
}

#[derive(Serialize, Deserialize)]
pub struct PopulationSerializable {
    generation: usize,
    individuals: Vec<IndividualSerializable>
}

#[derive(Serialize, Deserialize)]
pub struct PopulationsSerializable {
    grimoire: OptimizedGrimoire,
    populations: Vec<PopulationSerializable>,
}

impl PopulationsSerializable {
    pub fn new(grimoire: OptimizedGrimoire) -> Self {
        Self {
            grimoire,
            populations: Vec::default(),
        }
    }

    pub fn add_population(
        &mut self, 
        population: ParettoPopulation<AlchemyIndividual>, generation: usize
    ) {
        let population = PopulationSerializable {
            generation,
            individuals: population.into_iter().map(|x| x.into()).collect()
        };

        self.populations.push(population);
    }
}

impl From<AlchemyIndividual> for IndividualSerializable {
    fn from(value: AlchemyIndividual) -> Self {
        Self {
            fitness: value.fitness.into_iter().map(|x| x.into_inner()).collect(),
            genome: value.genotype.into_iter().map(|x| (x.ingredient_index, x.amount)).collect(),
        }
    }
}
