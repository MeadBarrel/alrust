pub mod ingredient;
pub mod ingredientmap;
pub mod mix;

pub use ingredient::*;
pub use ingredientmap::*;
pub use mix::*;


#[derive(Debug, Clone)]
pub struct OptimizedGrimoire {
    alvarin_clade: bool,
    advanced_potion_making_mod: f64,
    ingredients: IngredientMap,
}


impl OptimizedGrimoire {
    pub fn new(
        alvarin_clade: bool, 
        advanced_potion_making_mod: f64, 
        ingredients: IngredientMap
    ) -> Self {
        Self { alvarin_clade, advanced_potion_making_mod, ingredients }
    }
}
