use std::collections::HashMap;
use crate::types::*;
use std::{fmt, error::Error};
use error_stack::{Result};


#[derive(Debug)]
pub struct UnknownIngredientError {
    name: String
}


impl fmt::Display for UnknownIngredientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("Could not find the ingredient {}", self.name))
    }
}


impl Error for UnknownIngredientError {}


#[derive(Debug, Clone)]
pub struct Ingredient {
    pub name: String,
    pub alchemical_weight: u8,
    pub lore_multiplier: f64,
    pub modifiers: ModifierMap,
}


#[derive(Clone)]
pub struct OptimizedGrimoir {
    pub ingredients: Vec<Ingredient>,
    pub index: HashMap<String, usize>,
    pub advanced_potion_making_mod: f64,
}


impl OptimizedGrimoir {
    pub fn ingredients_from_names(&self, names: Vec<(String, u64)>) -> Result<Vec<(Ingredient, u64)>, UnknownIngredientError> {
        let mut result: Vec<(Ingredient, u64)> = Vec::default();

        for (name, count) in names {
            let index = self.index.get(&name).ok_or(UnknownIngredientError { name } )?.to_owned();
            let ingredient = self.ingredients[index].clone();
            result.push((ingredient, count))
        };

        Ok(result)
    }

}

