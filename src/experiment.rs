use std::collections::HashMap;

use serde::Deserialize;
use thiserror::Error;
use error_stack::{Result, ResultExt, IntoReport};

use crate::grimoiredb::GrimoireConfig;
use grimoire::mix::Mix;
use grimoire::optimized::Ingredient;


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
        use std::fs::File;
        use serde_yaml::from_reader;

        let file = File::open(filename)
            .into_report().change_context(ExperimentError::ConfigFileError)?;

        from_reader(file).into_report().change_context(ExperimentError::ConfigFileError)
    }


    pub fn mix(&self) -> Result<Mix, ExperimentError> {
        let grimoire = self.grimoire.build().change_context(ExperimentError::ExperimentFailed)?;
        let character = grimoire.characters.get(&self.character).ok_or_else(
            || ExperimentError::CharacterNotFound(self.character.clone())
        )?;
        let optimized_grimoire = grimoire.create_reference(character);

        let mut ingredients: Vec<(Ingredient, u64)> = Vec::default();

        for (name, value) in &self.mix {
            let index = optimized_grimoire.index.get(name).ok_or_else(
                || ExperimentError::IngredientNotFound(name.to_string())
            )?;

            let ingredient = optimized_grimoire.ingredients[*index].clone();

            ingredients.push((ingredient, *value));
        }

        let mix = Mix::new(
            optimized_grimoire.advanced_potion_making_mod, 
            character.alvarin_clade, 
            ingredients);
        Ok(mix)
    }
}
