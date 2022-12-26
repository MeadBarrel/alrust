use serde::{Serialize, Deserialize};

use crate::modifiermap::ModifierMap;

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
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


#[cfg(test)]
pub mod tests {
    use proptest::strategy::Strategy;
    use proptest::sample::select;
    use super::*;
    use crate::modifiermap::tests::modifier_map_strategy;
    
    pub fn ingredient_strategy() -> impl Strategy<Value = Ingredient> {
        let skill = select(vec![
            Some("a".to_string()),
            Some("b".to_string()),
            Some("c".to_string()),
            None,
        ]);
        let weight = select(vec![true, false]);
        let modifiers = modifier_map_strategy();

        (skill, weight, modifiers).prop_map(|(s, w, m)| Ingredient {
            skill: s,
            weight: w,
            modifiers: m
        })
    }
}