use std::ops::Index;

use super::StandaloneIngredient;


#[derive(Debug, Clone)]
pub struct IngredientMap {
    ingredients: Vec<StandaloneIngredient>,
    names: Vec<String>
}


impl IngredientMap {
    pub fn ingredients(&self) -> &Vec<StandaloneIngredient> {
        &self.ingredients
    }

    pub fn name(&self, i: usize) -> String {
        self.names[i].to_string()
    }
}


impl Index<usize> for IngredientMap {
    type Output = StandaloneIngredient;

    fn index(&self, index: usize) -> &Self::Output {
        &self.ingredients[index]
    }
}


impl<T> From<T> for IngredientMap 
    where T: Iterator<Item=(String, StandaloneIngredient)>
{
    fn from(src: T) -> Self {
        let (names, ingredients) = src.unzip();
        Self {
            names, ingredients
        }
    }
}
