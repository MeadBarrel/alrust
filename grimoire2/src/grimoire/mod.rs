pub mod character;
pub mod ingredient;
pub mod skill;

pub use character::*;
pub use ingredient::*;
pub use skill::*;

use indexmap::IndexMap;

type Skills = IndexMap<String, Skill>;
type Ingredients = IndexMap<String, Ingredient>;
type Characters = IndexMap<String, Character>;

#[derive(Default, Debug, Clone, PartialEq)]
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


pub mod versioned {
    use std::collections::HashMap;

    use serde::{Serialize, Deserialize};
    
    use super::Grimoire;
    use super::character::versioned::CharacterVersioned;
    use super::skill::versioned::SkillVersioned;
    use super::ingredient::versioned::IngredientVersioned;

    type SkillsVersioned = HashMap<String, SkillVersioned>;
    type IngredientsVersioned = HashMap<String, IngredientVersioned>;
    type CharactersVersioned = HashMap<String, CharacterVersioned>;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum GrimoireVersioned {
        #[serde(rename="0")]
        V0(GrimoireV0)
    }

    impl From<Grimoire> for GrimoireVersioned {
        fn from(value: Grimoire) -> Self {
            Self::V0(value.into())
        }
    }

    impl From<GrimoireVersioned> for Grimoire {
        fn from(value: GrimoireVersioned) -> Self {
            match value {
                GrimoireVersioned::V0(x) => x.into()
            }
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct GrimoireV0 {
        pub skills: SkillsVersioned,
        pub ingredients: IngredientsVersioned,
        pub characters: CharactersVersioned,           
    }

    impl From<Grimoire> for GrimoireV0 {
        fn from(value: Grimoire) -> Self {
            Self {
                skills: value.skills.into_iter().map(|(n, x)| (n, x.into())).collect(),
                ingredients: value.ingredients.into_iter().map(|(n, x)| (n, x.into())).collect(),
                characters: value.characters.into_iter().map(|(n, x)| (n, x.into())).collect(),
            }
        }
    }

    impl From<GrimoireV0> for Grimoire {
        fn from(value: GrimoireV0) -> Self {
            Self {
                skills: value.skills.into_iter().map(|(n, x)| (n, x.into())).collect(),
                ingredients: value.ingredients.into_iter().map(|(n, x)| (n, x.into())).collect(),
                characters: value.characters.into_iter().map(|(n, x)| (n, x.into())).collect(),
            }            
        }
    }
}


#[cfg(test)]
pub mod tests {
    use proptest::strategy::Strategy;
    use proptest::sample::select;
    use proptest::collection::hash_map;
    use super::*;
    use crate::grimoire::character::tests::character_strategy;
    use crate::grimoire::skill::tests::skill_strategy;
    use crate::grimoire::ingredient::tests::ingredient_strategy;
    
    pub fn grimoire_strategy() -> impl Strategy<Value=Grimoire> {
        let name = select(vec!["a", "b", "c"]);
        let characters = hash_map(name.clone(), character_strategy(), 3);
        let skills = hash_map(name.clone(), skill_strategy(), 3);
        let ingredients = hash_map(name, ingredient_strategy(), 3);

        (characters, skills, ingredients).prop_map(|(c, s, i)| Grimoire {
            characters: c.into_iter().map(|(n, v)| (n.to_string(), v)).collect(),
            skills: s.into_iter().map(|(n, v)| (n.to_string(), v)).collect(),
            ingredients: i.into_iter().map(|(n, v)| (n.to_string(), v)).collect(),
        } )
    }
}