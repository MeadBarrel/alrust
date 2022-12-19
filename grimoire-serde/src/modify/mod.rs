pub mod character;
pub mod skill;
pub mod ingredient;


use std::collections::HashMap;

pub use character::CharacterUpdateSerializable;
pub use skill::SkillUpdateSerializable;
pub use ingredient::IngredientUpdateSerializable;


use serde::{Serialize, Deserialize};
use grimoire2::{modify::GrimoireUpdate, prelude::Grimoire};
use grimoire2::modify::{
    character::CharacterUpdate,
    skill::SkillUpdate,
    ingredient::IngredientUpdate
};


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


impl GrimoireUpdateSerializable {
    pub fn to_update(&self) -> GrimoireUpdate {
        let mut update = GrimoireUpdate::default();

        self.remove_characters.iter().for_each(|name| { update.remove_character(name); });
        self.remove_skills.iter().for_each(|name| { update.remove_skill(name); } );
        self.remove_ingredients.iter().for_each(|name| { update.remove_ingredient(name); } );

        self.characters.iter().for_each(
            |(name, ser_update)| {
                update.character(name, ser_update.clone().into());
            }
        );

        self.skills.iter().for_each(
            |(name, ser_update)| {
                update.skill(name, ser_update.clone().into());
            }
        );

        self.ingredients.iter().for_each(
            |(name, ser_update)| {
                update.ingredient(name, ser_update.clone().into());
            }
        );

        update
    }

    pub fn from_grimoire(grimoire: &Grimoire) -> Self {
        let mut result = Self::default();

        grimoire.characters.iter().for_each(|(name, character)| {
            result.characters.insert(
                name.clone(), CharacterUpdate::from_character(character).into()
            );
        });

        grimoire.skills.iter().for_each(|(name, skill)| {
            result.skills.insert(
                name.clone(), SkillUpdate::from_skill(skill).into()
            );
        });

        grimoire.ingredients.iter().for_each(|(name, ingredient)| {
            result.ingredients.insert(
                name.clone(), IngredientUpdate::from_ingredient(ingredient).into()
            );
        });

        result
    }
}