pub mod ingredient;
pub mod ingredientmap;
pub mod mix;

pub use ingredient::*;
pub use ingredientmap::*;
pub use mix::*;

use crate::grimoire::{Character, Grimoire};

#[derive(Debug, Clone)]
pub struct OptimizedGrimoire {
    pub alvarin_clade: bool,
    pub advanced_potion_making_mod: f64,
    pub ingredients: IngredientMap,
}

impl OptimizedGrimoire {
    pub fn new(
        alvarin_clade: bool,
        advanced_potion_making_mod: f64,
        ingredients: IngredientMap,
    ) -> Self {
        Self {
            alvarin_clade,
            advanced_potion_making_mod,
            ingredients,
        }
    }
}

impl From<(&Character, &Grimoire)> for OptimizedGrimoire {
    fn from((character, grimoire): (&Character, &Grimoire)) -> Self {
        let alvarin_clade = character.clades.contains("Alchemist");
        let advanced_potion_making_mod =
            1. + 0.2 * (character.skill(&grimoire.skills, "Advanced Potion Making") / 100) as f64;
        let ingredients = grimoire
            .ingredients
            .iter()
            .map(|(name, ingredient)| {
                (
                    name.clone(),
                    StandaloneIngredient::new(
                        ingredient.weight as u8,
                        character.lore_multiplier(
                            &grimoire.skills,
                            ingredient.skill.as_ref().unwrap_or(&"".to_string()),
                        ),
                        ingredient.modifiers.clone(),
                    ),
                )
            })
            .collect::<Vec<(String, StandaloneIngredient)>>()
            .into_iter()
            .into();
        Self::new(alvarin_clade, advanced_potion_making_mod, ingredients)
    }
}
