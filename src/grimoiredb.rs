use std::{fmt::Display, collections::HashMap};

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use diesel::Connection;
use diesel::sqlite::SqliteConnection;
use error_stack::{Context, Result, IntoReport, ResultExt, Report};
use serde::Deserialize;

use geneticalchemy::prelude::Compendium;

use crate::models;
use grimoire::data;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");


#[derive(Debug, Default)]
pub struct LoadError {}

impl Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Could not load grimoire")
    }
}

impl Context for LoadError {}


#[derive(Deserialize, Default)]
#[serde(default)]
pub struct GrimoireConfig {
    db: Option<String>,
    characters: HashMap<String, CharacterConfig>,
    lores: HashMap<String, LoreConfig>,
    ingredients: HashMap<String, IngredientConfig>,
}

impl GrimoireConfig {
    pub fn build(&self) -> Result<Compendium, LoadError> {
        use grimoire::types::{replace_modifier_mod, replace_modifier_mul, Property};

        let mut grimoire = match &self.db {
            Some(filename) => load_grimoire_from_db(filename)?,
            None => Compendium::default(),
        };

        for (name, conf) in &self.lores {
            let mut lore = grimoire.lores.entry(name.clone())
                .or_insert_with(|| data::Lore::named_default(name));
            if let Some(x) = &conf.parent { lore.parent_name = Some(x.clone()); }
            if let Some(x) = conf.effectiveness { lore.effectiveness = x }
        }

        for (name, conf) in &self.characters {
            let mut character = grimoire.characters.entry(name.clone())
                .or_insert_with(|| data::Character::named_default(name));
            if let Some(x) = conf.advanced_potion_making {
                character.advanced_potion_making = x;
            }
            if let Some(x) = conf.alvarin_clade { character.alvarin_clade = x }
            for (name, value) in &conf.lores {
                character.lore_values.insert(name.clone(), *value);
            }
        };

        for (name, conf) in self.ingredients.iter() {
            let mut ingredient = grimoire.ingredients.entry(name.clone())
                .or_insert_with(|| data::Ingredient::named_default(name));
            
            if let Some(x) = conf.weight { ingredient.alchemical_weight = x };
            if let Some(x) = &conf.lore { ingredient.lore_name = x.clone() };

            if let Some(x) = conf.dh {
                replace_modifier_mod(&mut ingredient.modifiers, Property::DirectHealing, Some(x))
            }
            if let Some(x) = conf.mdh {
                replace_modifier_mul(&mut ingredient.modifiers, Property::DirectHealing, Some(x))
            }

            if let Some(x) = conf.dp {
                replace_modifier_mod(&mut ingredient.modifiers, Property::DirectPoison, Some(x))
            }
            if let Some(x) = conf.mdp {
                replace_modifier_mul(&mut ingredient.modifiers, Property::DirectPoison, Some(x))
            }

            if let Some(x) = conf.hot {
                replace_modifier_mod(&mut ingredient.modifiers, Property::HealingOverTime, Some(x))
            }
            if let Some(x) = conf.mhot {
                replace_modifier_mul(&mut ingredient.modifiers, Property::HealingOverTime, Some(x))
            }

            if let Some(x) = conf.pot {
                replace_modifier_mod(&mut ingredient.modifiers, Property::PoisonOverTime, Some(x))
            }
            if let Some(x) = conf.mpot {
                replace_modifier_mul(&mut ingredient.modifiers, Property::PoisonOverTime, Some(x))
            }
            
            if let Some(x) = conf.hl {
                replace_modifier_mod(&mut ingredient.modifiers, Property::HealingLength, Some(x))
            }
            if let Some(x) = conf.mhl {
                replace_modifier_mul(&mut ingredient.modifiers, Property::HealingLength, Some(x))
            }

            if let Some(x) = conf.pl {
                replace_modifier_mod(&mut ingredient.modifiers, Property::PoisonLength, Some(x))
            }
            if let Some(x) = conf.mpl {
                replace_modifier_mul(&mut ingredient.modifiers, Property::PoisonLength, Some(x))
            }
            
            if let Some(x) = conf.a {
                replace_modifier_mod(&mut ingredient.modifiers, Property::Alcohol, Some(x))
            }
            if let Some(x) = conf.ma {
                replace_modifier_mul(&mut ingredient.modifiers, Property::Alcohol, Some(x))
            }            

        };

        Ok(grimoire)
    }
}


#[derive(Deserialize)]
#[serde(default)]
pub struct LoreConfig {
    effectiveness: Option<f64>,
    parent: Option<String>,
}


impl Default for LoreConfig {
    fn default() -> Self {
        Self {
            effectiveness: Some(0.666666666),
            parent: None
        }
    }
}


#[derive(Deserialize)]
#[serde(default)]
pub struct CharacterConfig {
    advanced_potion_making: Option<u8>,
    alvarin_clade: Option<bool>,
    lores: HashMap<String, u8>
}


impl Default for CharacterConfig {
    fn default() -> Self {
        Self {
            advanced_potion_making: Some(100),
            alvarin_clade: Some(true),
            lores: HashMap::default(),
        }    
    }
}


#[derive(Deserialize, Default)]
#[serde(default)]
pub struct IngredientConfig {
    weight: Option<u8>,
    lore: Option<String>,
    dh: Option<f64>,
    dp: Option<f64>,
    mdh: Option<f64>,
    mdp: Option<f64>,
    hot: Option<f64>,
    pot: Option<f64>,
    mhot: Option<f64>,
    mpot: Option<f64>,
    hl: Option<f64>,
    pl: Option<f64>,
    mhl: Option<f64>,
    mpl: Option<f64>,
    a: Option<f64>,
    ma: Option<f64>,
}


fn run_migrations(connection: &mut SqliteConnection) -> Result<(), LoadError> {
    let result = connection.run_pending_migrations(MIGRATIONS);
    match result {
        Ok(_) => Ok(()),
        Err(_) => Err(
            Report::new(LoadError::default())
            .attach_printable("Failed to run migrations")
        )
    }
}

pub fn load_grimoire_from_db(filename: &str) -> Result<Compendium, LoadError> {
    let mut connection = SqliteConnection::establish(filename)
        .into_report()
        .change_context(LoadError::default())
        .attach_printable_lazy(|| format!("Could not load {}", filename))?;
    
    run_migrations(&mut connection)?;

    let ingredients_db = models::ingredient::Ingredient::load(&mut connection)
        .into_report()
        .change_context(LoadError::default())
        .attach_printable("Could not load ingredients")?;

    let lores_db = models::lore::Lore::load(&mut connection)
        .into_report()
        .change_context(LoadError::default())
        .attach_printable("Could not load lores")?;

    let player_characters_db = models::player_character::PlayerCharacter::load(&mut connection)
        .into_report()
        .change_context(LoadError::default())
        .attach_printable("Could not load characters")?;

    let player_character_lores_db = 
        models::player_character_lore::PlayerCharacterLore::load(&mut connection)
        .into_report()
        .change_context(LoadError::default())
        .attach_printable("Could not load player lores")?;

    let ingredients: Vec<data::Ingredient> = 
        ingredients_db.iter().map(|x| x.to_grimoire()).collect();

    let lores: Vec<data::Lore> =
        lores_db.iter().map(|x| x.to_grimoire()).collect();

    let player_characters: Vec<data::Character> =
        player_characters_db.iter().map(|x| x.to_grimoire(&player_character_lores_db)).collect();

    let grimoire = Compendium::create_from_vecs(player_characters, lores, ingredients);

    Ok(grimoire)
}