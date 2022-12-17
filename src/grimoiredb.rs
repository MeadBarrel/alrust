use std::{collections::HashMap, fmt::Display};

use error_stack::{Context, IntoReport, Result, ResultExt};
use grimoire2::prelude::{Character, Effect, Grimoire, Ingredient, Skill, Theoretical};
use serde::Deserialize;

use grimoire_sqlite::GrimoireSqlite;


#[derive(Debug, Default)]
pub struct LoadError {}

impl Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Could not load grimoire")
    }
}

impl Context for LoadError {}

#[derive(Debug, Default)]
pub struct SaveError {}

impl Display for SaveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Could not save grimoire")
    }
}

impl Context for SaveError {}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct GrimoireConfig {
    db: Option<String>,
    characters: HashMap<String, CharacterConfig>,
    lores: HashMap<String, LoreConfig>,
    ingredients: HashMap<String, IngredientConfig>,
}

impl GrimoireConfig {
    pub fn load(filename: &str) -> Result<Self, LoadError> {
        use serde_yaml::from_reader;
        use std::fs::File;

        let file = File::open(filename)
            .into_report()
            .change_context(LoadError::default())?;
        from_reader(file)
            .into_report()
            .change_context(LoadError::default())
    }

    pub fn build(&self) -> Result<Grimoire, LoadError> {
        let mut grimoire = match &self.db {
            Some(filename) => GrimoireSqlite::connect(filename)
                .into_report()
                .change_context(LoadError::default())?
                .load()
                .into_report()
                .change_context(LoadError::default())?,
            None => Grimoire::default(),
        };

        for (name, conf) in &self.lores {
            let mut lore = grimoire
                .skills
                .entry(name.clone())
                .or_insert_with(|| Skill::default());
            if let Some(x) = &conf.parent {
                lore.parent = Some(x.clone());
            }
            if let Some(x) = conf.effectiveness {
                lore.effectiveness = Theoretical::from(x)
            }
        }

        for (name, conf) in &self.characters {
            let mut c = Character::default();

            let character = grimoire
                .characters
                .entry(name.clone())
                .or_insert_with(|| Character::default());
                
            if let Some(x) = conf.advanced_potion_making {
                character.skills.insert("Advanced Potion Making".to_string(), x);
            };

            if let Some(x) = conf.alvarin_clade {
                match x {
                    true => c.clades.insert("Alchemist".to_string()),
                    false => c.clades.remove("Alchemist"),
                };
            }

            for (name, value) in &conf.lores {
                character.skills.insert(name.to_string(), *value);
            }
        }

        for (name, conf) in self.ingredients.iter() {
            let mut ingredient = grimoire
                .ingredients
                .entry(name.clone())
                .or_insert_with(|| Ingredient::default());

            if let Some(x) = conf.weight {
                ingredient.weight = x > 0
            };
            if let Some(x) = &conf.lore {
                ingredient.skill = Some(x.clone())
            };

            if let Some(x) = conf.dh {
                ingredient.modifiers[Effect::DirectHealing].term = Theoretical::from(x);
            }
            if let Some(x) = conf.mdh {
                ingredient.modifiers[Effect::DirectHealing].multiplier = Theoretical::from(x);
            }

            if let Some(x) = conf.dp {
                ingredient.modifiers[Effect::DirectPoison].term = Theoretical::from(x);
            }
            if let Some(x) = conf.mdp {
                ingredient.modifiers[Effect::DirectPoison].multiplier = Theoretical::from(x);
            }

            if let Some(x) = conf.hot {
                ingredient.modifiers[Effect::HealingOverTime].term = Theoretical::from(x);
            }
            if let Some(x) = conf.mhot {
                ingredient.modifiers[Effect::HealingOverTime].multiplier = Theoretical::from(x);
            }

            if let Some(x) = conf.pot {
                ingredient.modifiers[Effect::PoisonOverTime].term = Theoretical::from(x);
            }
            if let Some(x) = conf.pot {
                ingredient.modifiers[Effect::PoisonOverTime].multiplier = Theoretical::from(x);
            }

            if let Some(x) = conf.hl {
                ingredient.modifiers[Effect::HealingLength].term = Theoretical::from(x);
            }
            if let Some(x) = conf.mhl {
                ingredient.modifiers[Effect::HealingLength].multiplier = Theoretical::from(x);
            }

            if let Some(x) = conf.pl {
                ingredient.modifiers[Effect::PoisonLength].term = Theoretical::from(x);
            }
            if let Some(x) = conf.mpl {
                ingredient.modifiers[Effect::PoisonLength].multiplier = Theoretical::from(x);
            }

            if let Some(x) = conf.a {
                ingredient.modifiers[Effect::Alcohol].term = Theoretical::from(x);
            }
            if let Some(x) = conf.ma {
                ingredient.modifiers[Effect::Alcohol].multiplier = Theoretical::from(x);
            }
        }

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
            parent: None,
        }
    }
}

#[derive(Deserialize)]
#[serde(default)]
pub struct CharacterConfig {
    advanced_potion_making: Option<u8>,
    alvarin_clade: Option<bool>,
    lores: HashMap<String, u8>,
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
