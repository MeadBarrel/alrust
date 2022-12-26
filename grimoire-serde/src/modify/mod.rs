pub mod character;
pub mod skill;
pub mod ingredient;


use std::collections::HashMap;

pub use character::CharacterUpdateSerializable;
pub use skill::SkillUpdateSerializable;
pub use ingredient::IngredientUpdateSerializable;


use serde::{Serialize, Deserialize};
use grimoire2::modify::GrimoireUpdate;


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
#[serde(deny_unknown_fields)]
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


}