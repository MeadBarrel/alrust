use std::str::FromStr;

use chrono::Utc;
use error_stack::{IntoReport, ResultExt};
use geneticalchemy::prelude::{AlchemyIndividual, OptimizedGrimoir, PotionSerializable};
use serde::Deserialize;

use super::error::{Result, OptimizationError};
use genetic::prelude::{Population, Individual, ParettoPopulation};

pub trait PopulationPrinter {
    type Population: Population;

    fn print(
        &mut self, 
        grimoire: &OptimizedGrimoir, 
        population: &Self::Population, 
        generation: usize
    ) -> Result<()>;
}


#[derive(Deserialize)]
#[serde(default)]
pub struct ToYaml {
    folder: String,
    subfolder: String,
}


impl Default for ToYaml {
    fn default() -> Self {
        Self { 
            folder: "output".to_string(),
            subfolder: Utc::now().format("%Y-%m-%d-%H%M%S").to_string(),
        }
    }
}


impl PopulationPrinter for ToYaml {
    type Population = ParettoPopulation<AlchemyIndividual>;

    fn print(
            &mut self, 
            grimoire: &OptimizedGrimoir, 
            population: &Self::Population, 
            generation: usize
        ) -> Result<()>  {
        use serde_yaml::to_writer;
        use std::fs::File;
        use std::path::PathBuf;
        use std::fs::create_dir_all;

        let output_folder = PathBuf::from_str(&self.folder)
            .into_report().change_context(OptimizationError::OutputError)?
            .join(&self.subfolder);
        create_dir_all(&output_folder)
            .into_report().change_context(OptimizationError::OutputError)?;

        let output_filename = output_folder.join(format!("{}.yaml", generation));

        let mut this_population = population.clone();
        this_population.sort();

        let printable: Vec<PotionSerializable> = this_population.individuals().iter()
            .map(|x| genome_to_serializable(grimoire, x)).collect();

        let file = File::create(&output_filename)
            .into_report().change_context(OptimizationError::OutputError)?;
        to_writer(file, &printable).into_report().change_context(OptimizationError::OutputError)?;

        Ok(())
    }
}


fn genome_to_serializable(grimoire: &OptimizedGrimoir, individual: &AlchemyIndividual) -> PotionSerializable {
    use grimoire::mix::Mix;

    let genome = individual.genotype();

    let mix = Mix {
        ingredients: genome.iter().map(|gene|(
            grimoire.ingredients[gene.ingredient_index].clone(),
            gene.amount
        )).collect(),
        advanced_potion_making_mod: grimoire.advanced_potion_making_mod,
        alvarin_clade: false,
    };

    PotionSerializable::from_mix(&mix)
}

