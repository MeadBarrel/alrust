use std::cmp::min;
use std::collections::HashMap;
use crate::types::*;
use crate::optimized;

#[derive(Debug, Clone)]
pub struct Lore {
    pub name: String,
    pub effectiveness: Option<f64>,
    pub parent_name: Option<String>
}


impl Lore {
    pub fn new(name: &str, effectiveness: Option<f64>, parent_name: Option<String>) -> Self {
        Self {
            name: name.to_string(),
            effectiveness,
            parent_name
        }
    }

    pub fn named_default(name: &str) -> Self {
        Self {
            name: name.to_string(),
            effectiveness: None,
            parent_name: None
        }
    }
}


#[derive(Debug, Clone)]
pub struct Ingredient {
    pub name: String,
    pub alchemical_weight: u8,
    pub lore_name: String,
    pub modifiers: HashMap<Property, Modifier>,
}


impl Ingredient {
    pub fn new(name: &str, alchemical_weight: u8, lore_name: &str, modifiers: HashMap<Property, Modifier>) -> Self{
        Self {
            name: name.to_string(),
            lore_name: lore_name.to_string(),
            alchemical_weight,
            modifiers
        }
    }

    pub fn named_default(name: &str) -> Self {
        Self {
            name: name.to_string(),
            alchemical_weight: 0,
            lore_name: "default".to_string(),
            modifiers: HashMap::default(),
        }
    }

    pub fn get_modifier(&self, property: Property) -> Modifier {
        self.modifiers.get(&property).cloned().unwrap_or_default()
    }
}


#[derive(Debug, Clone)]
pub struct Character {
    pub name: String,
    pub lore_values: HashMap<String, u8>,
    pub advanced_potion_making: u8,
    pub alvarin_clade: bool,
}


impl Default for Character {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            lore_values: HashMap::default(),
            advanced_potion_making: 100,
            alvarin_clade: true,
        }
    }
}


impl Character {
    pub fn new(name: &str, lore_values: HashMap<String, u8>, advanced_potion_making: u8, alvarin_clade: bool) -> Self {
        Self {
            name: name.to_string(),
            lore_values,
            advanced_potion_making,
            alvarin_clade,
        }
    }

    pub fn named_default(name: &str) -> Self {
        Self {
            name: name.to_string(),
            lore_values: HashMap::default(),
            advanced_potion_making: 100,
            alvarin_clade: true,            
        }
    }

    pub fn get_lore_value(&self, name: &str) -> u8 {
        let default = 0;
        *self.lore_values.get(name).unwrap_or(&default)
    }
}


#[derive(Debug, Default)]
pub struct Compendium {
    pub characters: HashMap<String, Character>,
    pub lores: HashMap<String, Lore>,
    pub ingredients: HashMap<String, Ingredient>
}


impl Compendium {
    pub fn create_from_vecs(characters: Vec<Character>, lores: Vec<Lore>, ingredients: Vec<Ingredient>) -> Self {
        Self {
            characters: characters.iter().map(|x| (x.name.clone(), x.clone())).collect::<HashMap<String, Character>>(),
            lores: lores.iter().map(|x| (x.name.clone(), x.clone())).collect::<HashMap<String, Lore>>(),
            ingredients: ingredients.iter().map(|x| (x.name.clone(), x.clone())).collect::<HashMap<String, Ingredient>>(),
        }
    }

    /// Create an optimized reference for the specified character
    pub fn create_reference(&self, character: &Character) -> optimized::OptimizedGrimoir {
        let ingredients: Vec<optimized::Ingredient> = self.ingredients.iter().map(
            |(_, ingredient)| {
                optimized::Ingredient {
                    name: ingredient.name.clone(),
                    alchemical_weight: ingredient.alchemical_weight,
                    lore_multiplier: self.get_lore_multiplier(character, &ingredient.lore_name),
                    modifiers: create_modifier_map(&ingredient.modifiers),
                }
            }
        ).collect();

        let index: HashMap<String, usize> =
            ingredients.iter().enumerate().map(|(i, ing)| (ing.name.clone(), i)).collect();

        let advanced_potion_making_mod = 1.0 + 0.2 * (character.advanced_potion_making as f64 / 100.);

        optimized::OptimizedGrimoir {
            ingredients, index, advanced_potion_making_mod
        }
    }


    /// Get caracter's effective lore multiplier for the specified lore
    pub fn get_lore_multiplier(&self, character: &Character, lore: &str) -> f64 {
        let lore_effectiveness = match self.lores.get(lore) {
            Some(x) => x.effectiveness,
            None => Some(0.66666),
        };
        1. + lore_effectiveness.unwrap_or(0.66666) * self.get_lore_value(character, &lore) as f64 / 100.
    }

    /// Return character's effective lore value.
    /// 
    /// If parent lore value is lower, that parent lore value will be returned.
    /// If lore value is not set, return 0.
    pub fn get_lore_value(&self, character: &Character, lore_name: &str) -> u8 {
        let default_lore = Lore::named_default(lore_name);
        let lore = self.lores.get(lore_name).unwrap_or_else(|| &default_lore);
        let lore_value = character.get_lore_value(lore_name);

        match &lore.parent_name {
            Some(parent_name) => min(lore_value, self.get_lore_value(character, &parent_name)),
            None => lore_value
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_lore_value() {
        let compendium = create_test_data();

        let expected = 50_u8;
        let actual = compendium.get_lore_value(&compendium.characters.get("Tashka").unwrap(), "Steel Lore");

        assert_eq!(actual, expected);
    }

    fn create_test_data() -> Compendium {
        let lores = vec![
            Lore {name: "Steel Lore".to_owned(), effectiveness: Some(0.66666), parent_name: Some("Iron-based Alloys".to_owned())},
            Lore {name: "Iron-based Alloys".to_owned(), effectiveness: Some(0.66666), parent_name: Some("Metallurgy".to_owned())},
            Lore {name: "Metallurgy".to_owned(), effectiveness: Some(0.66666), parent_name: None},
        ];
        let ingredients = Vec::<Ingredient>::default();
        let lore_values = vec![
            ("Steel Lore".to_owned(), 90_u8),
            ("Iron-based Alloys".to_owned(), 100_u8),
            ("Metallurgy".to_owned(), 50_u8)
        ].into_iter().collect::<HashMap<String, u8>>();
        let characters = vec![
            Character {name: "Tashka".to_owned(), lore_values: lore_values, advanced_potion_making: 100_u8, alvarin_clade: false}
        ];

        Compendium::create_from_vecs(characters, lores, ingredients)
    }
}