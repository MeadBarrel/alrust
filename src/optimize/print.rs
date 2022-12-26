use chrono::Utc;
use error_stack::{IntoReport, ResultExt};
use serde::Deserialize;
use std::str::FromStr;

use genetic::prelude::{Individual, ParettoPopulation, Population};
use geneticalchemy::prelude::AlchemyIndividual;
use grimoire2::prelude::OptimizedGrimoire;

use super::error::{OptimizationError, Result};
use grimoire_serde::potion::{PotionSerializableConfig, PotionSerializable};

const TO_JSON_HELP: &str = "
    folder: <folder to write to - default is `output`>
    subfolder: <subfolder to write to - default is based on current date and time>
    config: <potion serialization config (see help for `mix`)>
";

#[derive(Deserialize)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct ToJson {
    folder: String,
    subfolder: String,
    config: PotionSerializableConfig,
}

pub trait PopulationPrinter {
    type Population: Population;

    fn print(
        &mut self,
        grimoire: &OptimizedGrimoire,
        population: &Self::Population,
        generation: usize,
    ) -> Result<()>;
}

impl Default for ToJson {
    fn default() -> Self {
        Self {
            folder: "output".to_string(),
            subfolder: Utc::now().format("%Y-%m-%d-%H%M%S").to_string(),
            config: PotionSerializableConfig::default(),
        }
    }
}

impl PopulationPrinter for ToJson {
    type Population = ParettoPopulation<AlchemyIndividual>;

    fn print(
        &mut self,
        grimoire: &OptimizedGrimoire,
        population: &Self::Population,
        generation: usize,
    ) -> Result<()> {
        use serde_yaml::to_writer;
        use std::{
            fs::{create_dir_all, File},
            path::PathBuf,
        };

        let output_folder = PathBuf::from_str(&self.folder)
            .into_report()
            .change_context(OptimizationError::OutputError)?
            .join(&self.subfolder);
        create_dir_all(&output_folder)
            .into_report()
            .change_context(OptimizationError::OutputError)?;

        let output_filename = output_folder.join(format!("{}.yaml", generation));

        let mut this_population = population.clone();
        this_population.sort();

        let printable: Vec<PotionSerializable> = this_population
            .individuals()
            .iter()
            .map(|x| genome_to_serializable(grimoire, &self.config, x))
            .collect();

        let file = File::create(output_filename)
            .into_report()
            .change_context(OptimizationError::OutputError)?;
        to_writer(file, &printable)
            .into_report()
            .change_context(OptimizationError::OutputError)?;

        Ok(())
    }
}

fn genome_to_serializable(
    grimoire: &OptimizedGrimoire,
    config: &PotionSerializableConfig,
    individual: &AlchemyIndividual,
) -> PotionSerializable {
    use grimoire2::prelude::Mix;

    let genome = individual.genotype();

    let mix = Mix::new(
        grimoire, 
        genome
            .iter()
            .map(|gene| gene.clone().into())
            .collect()
    );

    config.serialize_mix(&mix)
}