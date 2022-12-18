pub mod character;
pub mod skill;
pub mod ingredient;


use std::collections::HashMap;

pub use character::CharacterUpdateSerializable;
pub use skill::SkillUpdateSerializable;
pub use ingredient::IngredientUpdateSerializable;


use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct GrimoireUpdateSerializable {
    remove_characters: Vec<String>,
    remove_skills: Vec<String>,
    remove_ingredients: Vec<String>,

    characters: HashMap<String, CharacterUpdateSerializable>,
    skills: HashMap<String, SkillUpdateSerializable>,
    ingredients: HashMap<String, IngredientUpdateSerializable>
}