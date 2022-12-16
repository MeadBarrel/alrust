use crate::modifiermap::ModifierMap;
use crate::theoretical::Theoretical;


#[derive(Default, Debug, Clone)]
pub struct StandaloneIngredient {
    pub weight: usize,
    pub lore_multiplier: Theoretical<f64>,
    pub modifiers: ModifierMap,
}


impl StandaloneIngredient {
    pub fn new(weight: usize, lore_multiplier: Theoretical<f64>, modifiers: ModifierMap) -> Self {
        Self { weight, lore_multiplier, modifiers }
    }
}
