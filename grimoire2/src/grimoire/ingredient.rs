use serde::{Serialize, Deserialize};

use crate::modifiermap::ModifierMap;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Ingredient {
    pub skill: Option<String>,
    pub weight: bool,
    pub modifiers: ModifierMap,
}

impl Ingredient {
    pub fn new(skill: &str, weight: bool, modifiers: ModifierMap) -> Self {
        Self {
            skill: Some(skill.to_string()),
            weight,
            modifiers,
        }
    }
}



pub mod versioned {
    use serde::{Serialize, Deserialize};

    use super::Ingredient;
    use crate::modifiermap::versioned::ModifierMapVersioned;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum IngredientVersioned {
        #[serde(rename="0")]
        V0(IngredientV0)
    }


    impl From<Ingredient> for IngredientVersioned {
        fn from(value: Ingredient) -> Self {
            Self::V0(value.into())
        }
    }


    impl From<IngredientVersioned> for Ingredient {
        fn from(value: IngredientVersioned) -> Self {
            match value {
                IngredientVersioned::V0(x) => x.into()
            }
        }
    }


    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct IngredientV0 {
        pub skill: Option<String>,
        pub weight: bool,
        pub modifiers: ModifierMapVersioned,           
    }

    impl From<Ingredient> for IngredientV0 {
        fn from(value: Ingredient) -> Self {
            Self {
                skill: value.skill.clone(),
                weight: value.weight,
                modifiers: value.modifiers.into()
            }
        }
    }


    impl From<IngredientV0> for Ingredient {
        fn from(value: IngredientV0) -> Self {
            Self {
                skill: value.skill.clone(),
                weight: value.weight,
                modifiers: value.modifiers.into()
            }
        }
    }
}