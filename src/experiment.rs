use std::fs::File;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_yaml::from_reader;
use anyhow::Result;
use thiserror::Error;


use grimoire::prelude::*;
use grimoire::optimized::Ingredient;
use crate::experiment::ExperimentError::IngredientNotFound;


#[derive(Error, Debug)]
pub enum ExperimentError {
    #[error("Ingredient {0} not found")]
    IngredientNotFound(String),
}


#[derive(Deserialize)]
#[serde(default)]
pub struct ExperimentConfig {
    db: String,
    character: CharacterConfig,
    mix: HashMap<String, u64>,
    assume: HashMap<String, IngredientAssume>
}

#[derive(Deserialize)]
#[serde(default)]
pub struct CharacterConfig {
    name: String,
    advanced_potion_making: Option<u8>,
    alvarin_clade: Option<bool>,
    lores: HashMap<String, u8>,
}


#[derive(Deserialize)]
pub struct IngredientAssume {
    dh: Option<f64>,
    mdh: Option<f64>,
    dp: Option<f64>,
    mdp: Option<f64>,
    hot: Option<f64>,
    mhot: Option<f64>,
    pot: Option<f64>,
    mpot: Option<f64>,
    hl: Option<f64>,
    mhl: Option<f64>,
    pl: Option<f64>,
    mpl: Option<f64>,
    a: Option<f64>,
    ma: Option<f64>,
}


impl ExperimentConfig {
    pub fn from_file(filename: &str) -> Result<Self> {
        let file = File::open(filename)?;
        Ok(from_reader::<File, Self>(file)?)
    }

    pub fn mix(&self) -> Result<Mix> {
        let mut default_character = Character::default();
        let mut grimoire_long = load_from_db(&self.db)?;

        
        let mut character = match grimoire_long.characters.get_mut(&self.character.name) {
            Some(c) =>  c,
            None => &mut default_character,
        };

        self.character.lores.iter().for_each(
            |(lore_name, lore_value)|
                {
                    character.lore_values
                        .insert(lore_name.to_string(), *lore_value);
                }
        );

        if let Some(x) = self.character.advanced_potion_making {
            character.advanced_potion_making=x;
        };

        if let Some(x) = self.character.alvarin_clade {
            character.alvarin_clade = x;
        }


        let character = character.clone();

        self.process_assumes(&mut grimoire_long);

        let grimoire = grimoire_long.create_reference(&character);

        let ingredients = self.mix.iter().map(
            |(name, value)| Ok((
                grimoire.ingredients[*grimoire.index.get(name)
                    .ok_or_else(|| IngredientNotFound(name.to_string()))?].clone(),
                *value,
            ))
        ).collect::<Result<Vec<(Ingredient, u64)>>>()?;        



        Ok(Mix::new(grimoire.advanced_potion_making_mod, character.alvarin_clade, ingredients))
    }

    fn process_assumes(&self, grimoire_long: &mut grimoire::data::Compendium) {
        grimoire_long.ingredients.iter_mut().for_each(
            |(name, ingredient)| {
                let maybe_assume = self.assume.get(name);
                if let Some(assumes) = maybe_assume {
                    if let Some(x) = assumes.dh { 
                        replace_modifier_mod(&mut ingredient.modifiers, Property::DirectHealing, x)
                    };
                    if let Some(x) = assumes.mdh {
                        replace_modifier_mul(&mut ingredient.modifiers, Property::DirectHealing, x)
                    }
                    if let Some(x) = assumes.dp { 
                        replace_modifier_mod(&mut ingredient.modifiers, Property::DirectPoison, x)
                    };
                    if let Some(x) = assumes.mdp {
                        replace_modifier_mul(&mut ingredient.modifiers, Property::DirectPoison, x)
                    }                        
                    if let Some(x) = assumes.hot { 
                        replace_modifier_mod(&mut ingredient.modifiers, Property::HealingOverTime, x)
                    };
                    if let Some(x) = assumes.mhot {
                        replace_modifier_mul(&mut ingredient.modifiers, Property::HealingOverTime, x)
                    }
                    if let Some(x) = assumes.pot { 
                        replace_modifier_mod(&mut ingredient.modifiers, Property::PoisonOverTime, x)
                    };
                    if let Some(x) = assumes.mpot {
                        replace_modifier_mul(&mut ingredient.modifiers, Property::PoisonOverTime, x)
                    }
                    if let Some(x) = assumes.hl { 
                        replace_modifier_mod(&mut ingredient.modifiers, Property::HealingLength, x)
                    };
                    if let Some(x) = assumes.mhl {
                        replace_modifier_mul(&mut ingredient.modifiers, Property::HealingLength, x)
                    }
                    if let Some(x) = assumes.pl { 
                        replace_modifier_mod(&mut ingredient.modifiers, Property::PoisonLength, x)
                    };
                    if let Some(x) = assumes.mpl {
                        replace_modifier_mul(&mut ingredient.modifiers, Property::PoisonLength, x)
                    }
                    if let Some(x) = assumes.a { 
                        replace_modifier_mod(&mut ingredient.modifiers, Property::Alcohol, x)
                    };
                    if let Some(x) = assumes.ma {
                        replace_modifier_mul(&mut ingredient.modifiers, Property::Alcohol, x)
                    }                      
                }
            }
        );        
    }
}


impl Default for ExperimentConfig {
    fn default() -> Self {
        Self {
            db: "db.sqlite".to_string(),
            character: CharacterConfig::default(),
            mix: HashMap::default(),
            assume: HashMap::default(),
        }
    }
}


impl Default for CharacterConfig {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            advanced_potion_making: Some(100),
            alvarin_clade: Some(true),
            lores: HashMap::new(),
        }
    }
}
