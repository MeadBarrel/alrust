use std::collections::HashMap;

use error_stack::{IntoReport, Result, ResultExt};
use serde::Deserialize;
use thiserror::Error;

use grimoire2::prelude::{Mix, OptimizedGrimoire, Ingredient};

use crate::grimoiredb::GrimoireConfig;


#[derive(Error, Debug)]
pub enum ExperimentError {
    #[error("Experiment failed")]
    ExperimentFailed,
    #[error("Ingredient not found: {0}")]
    IngredientNotFound(String),
    #[error("Character not found: {0}")]
    CharacterNotFound(String),
    #[error("Could not load configuration file")]
    ConfigFileError,
}

#[derive(Deserialize)]
#[serde(default)]
pub struct ExperimentConfig {
    grimoire: GrimoireConfig,
    character: String,
    mix: HashMap<String, u64>,
}

impl Default for ExperimentConfig {
    fn default() -> Self {
        Self {
            grimoire: GrimoireConfig::default(),
            character: "default".to_string(),
            mix: HashMap::default(),
        }
    }
}

impl ExperimentConfig {
    pub fn load(filename: &str) -> Result<Self, ExperimentError> {
        use serde_yaml::from_reader;
        use std::fs::File;

        let file = File::open(filename)
            .into_report()
            .change_context(ExperimentError::ConfigFileError)?;

        from_reader(file)
            .into_report()
            .change_context(ExperimentError::ConfigFileError)
    }

    pub fn grimoire(&self) -> Result<OptimizedGrimoire, ExperimentError> {
        let grimoire = self.grimoire.build().change_context(ExperimentError::ExperimentFailed)?;
        let character = grimoire
            .characters
            .get(&self.character)
            .ok_or_else(|| ExperimentError::CharacterNotFound(self.character.clone()))?;
        Ok((character, &grimoire).into())

    }

    pub fn mix<'a>(&'a self, grimoire: &'a OptimizedGrimoire) -> Result<Mix<'a>, ExperimentError> {
        let mut ingredients: Vec<(usize, u64)> = Vec::default();

        for (name, value) in &self.mix {
            let index = grimoire
                .ingredients
                .by_name(name)
                .into_report()
                .change_context(ExperimentError::IngredientNotFound(name.to_string()))?;

            ingredients.push((index, *value));
        }

        Ok(
            Mix::new(grimoire, ingredients)
        )
    }
}
