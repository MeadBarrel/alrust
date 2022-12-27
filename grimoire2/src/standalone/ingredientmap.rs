use std::{ops::Index, collections::HashMap};

use super::StandaloneIngredient;
use crate::error::{Result, Error};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngredientMap {
    ingredients: Vec<StandaloneIngredient>,
    names: Vec<String>,
    names_map: HashMap<String, usize>,
}

impl IngredientMap {
    pub fn ingredients(&self) -> &Vec<StandaloneIngredient> {
        &self.ingredients
    }

    pub fn name(&self, i: usize) -> &str {
        &self.names[i]
    }

    pub fn len(&self) -> usize {
        self.ingredients.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ingredients.is_empty()
    }

    pub fn by_name(&self, name: &str) -> Result<usize> {
        match self.names_map.get(name) {
            Some(x) => Ok(*x),
            None => Err(Error::IngredientNotFound(name.to_string()))            
        }
    }
}

impl Index<usize> for IngredientMap {
    type Output = StandaloneIngredient;

    fn index(&self, index: usize) -> &Self::Output {
        &self.ingredients[index]
    }
}

impl<T> From<T> for IngredientMap
where
    T: Iterator<Item = (String, StandaloneIngredient)>,
{
    fn from(src: T) -> Self {
        let (names, ingredients): (Vec<String>, Vec<StandaloneIngredient>) = src.unzip();
        Self { 
            names_map: names.iter().enumerate().map(|(i, n)| (n.clone(), i)).collect(),
            names, 
            ingredients             
        }
    }
}
