use thiserror::Error;
use serde::Deserialize;
use error_stack::{IntoReport, Result, ResultExt, Report};

use grimoire_sqlite::GrimoireSqlite;

use grimoire_serde::mix::MixIngredients;
use grimoire_serde::modify::GrimoireUpdateSerializable;
use grimoire_serde::potion::{PotionSerializableConfig, PotionSerializable};

use grimoire2::standalone::{OptimizedGrimoire, Mix};


#[derive(Deserialize, Debug, Default)]
#[serde(default)]
pub struct ExperimentConfig {
    db: String,
    character: String,
    potion: PotionSerializableConfig,
    grimoire: GrimoireUpdateSerializable,
    mix: MixIngredients,
}


#[derive(Error, Debug)]
pub enum ExperimentError {
    #[error("Ingredient not found: {0}")]
    IngredientNotFound(String),
    #[error("Character not found: {0}")]
    CharacterNotFound(String),
    #[error("Could not load grimoire")]
    GrimoireLoadFailed,    
}


impl ExperimentConfig {
    pub fn run(&self) -> Result<PotionSerializable, ExperimentError> {
        let mut grimoire = GrimoireSqlite::connect(&self.db)
            .into_report()
            .change_context(ExperimentError::GrimoireLoadFailed)?
            .load()
            .into_report()
            .change_context(ExperimentError::GrimoireLoadFailed)?;
        
        self.grimoire.to_update().update(&mut grimoire);
        let character = grimoire.characters.get(&self.character)
            .ok_or(
                Report::new(ExperimentError::CharacterNotFound(self.character.clone()))
            )?;
        let optimized_grimoire = OptimizedGrimoire::from((character, &grimoire));

        let mut ingredients: Vec<(usize, u64)> = Vec::default();

        for (name, value) in &self.mix {
            let index = optimized_grimoire
                .ingredients
                .by_name(name)
                .into_report()
                .change_context(ExperimentError::IngredientNotFound(name.to_string()))?;

            ingredients.push((index, *value));
        }

        let mix = Mix::new(&optimized_grimoire, ingredients);

        Ok(self.potion.serialize_mix(&mix))
    }
}
