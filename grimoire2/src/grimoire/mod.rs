pub mod character;
pub mod ingredient;
pub mod skill;

pub use character::*;
pub use ingredient::*;
pub use skill::*;

use std::collections::HashMap;

type Skills = HashMap<String, Skill>;
type Ingredients = HashMap<String, Ingredient>;
type Characters = HashMap<String, Character>;

#[derive(Default)]
pub struct Grimoire {
    pub skills: Skills,
    pub ingredients: Ingredients,
    pub characters: Characters,
}

impl Grimoire {
    pub fn new(skills: Skills, ingredients: Ingredients, characters: Characters) -> Self {
        Self {
            skills,
            ingredients,
            characters,
        }
    }
}