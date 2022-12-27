use crate::{modifiermap::ModifierMap, theoretical::Theoretical};
use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct StandaloneIngredient {
    pub weight: u8,
    pub lore_multiplier: Theoretical<f64>,
    pub modifiers: ModifierMap,
}

impl StandaloneIngredient {
    pub fn new(weight: u8, lore_multiplier: Theoretical<f64>, modifiers: ModifierMap) -> Self {
        Self {
            weight,
            lore_multiplier,
            modifiers,
        }
    }
}
