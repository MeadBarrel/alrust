use strum::IntoEnumIterator;

use crate::optimized::*;
use crate::types::*;


#[derive(Debug, Clone)]
pub struct Mix {
    pub ingredients: Vec<(Ingredient, u64)>,
    pub advanced_potion_making_mod: f64
}


pub fn mix_effects(mix: &Mix) -> EffectsMap {
    let mut result = EffectsMap::default();
    for property in Property::iter() {
        result[property as usize] = mix_effect(mix, property);
    };

    result
}


pub fn mix_volume(mix: &Mix) -> f64 {
    let w: u64 = mix.ingredients.iter().map(|(ingredient, count)| ingredient.alchemical_weight as u64 * count.to_owned() as u64).sum();
    (w as f64 - 1.) / 10.
}


pub fn mix_effect(mix: &Mix, property: Property) -> f64 {
    let total_count: u64 = mix.ingredients.iter().map(|(_, count)| count).sum();

    if total_count == 0 { return 0. }

    let multiplier: f64 = mix.ingredients.iter().map(
        |(ingredient, count)| 1. + ingredient.modifiers[property as usize].multiplier * (count.to_owned() as f64 / total_count as f64).sqrt()
    ).product();

    let sum: f64 = mix.ingredients.iter().map(
        |(ingredient, count)| ingredient.lore_multiplier * ingredient.modifiers[property as usize].modifier * (count.to_owned() as f64 / total_count as f64)
    ).sum();

    mix.advanced_potion_making_mod * sum * multiplier
}


#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use float_cmp::approx_eq;

    use super::*;
    use crate::sqlite::load_from_db;


    #[test]
    fn test_mix_dh() {
        let reference = load_data();
        let mix = Mix { advanced_potion_making_mod: 1.2, ingredients: reference.ingredients_from_names(ingredients()).unwrap() };

        let expected = 6.5;
        let actual = mix_effect(&mix, Property::DirectHealing);

        assert!( approx_eq!(f64, actual, expected, epsilon=0.01) );
    }

    #[test]
    fn test_mix_effects() {
        let reference = load_data();
        let mix = Mix { advanced_potion_making_mod: 1.2, ingredients: reference.ingredients_from_names(ingredients()).unwrap() };
        let actual = mix_effects(&mix);

        assert!( approx_eq!(f64, actual[Property::DirectHealing as usize], 6.5, epsilon=0.01) );
    }


    #[test
    
    
    
    
    
    
    
    ]
    fn test_mix_volume() {
        let reference = load_data();
        let mix = Mix { advanced_potion_making_mod: 1.2, ingredients: reference.ingredients_from_names(ingredients()).unwrap() };
        let actual = mix_volume(&mix);

        assert!( approx_eq!(f64, actual, 10., epsilon=0.01) );
    }

    fn load_data() -> OptimizedGrimoir {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("../testdb.sqlite");
        let compendium = load_from_db(d.to_str().unwrap()).unwrap();
        compendium.create_reference(&compendium.characters["default"])
    }

    fn ingredients() -> Vec<(String, u64)> {
        vec![
            ("Sea Dew Leaves".to_owned(), 67),
            ("Argus Sponge".to_owned(), 7),
            ("Skadite".to_owned(), 6),
            ("Gold".to_owned(), 4),
            ("Calxfish".to_owned(), 4),
            ("Nitre Queen Carcass".to_owned(), 4),
            ("Jadeite".to_owned(), 4),
            ("Great Horn".to_owned(), 4),
            ("Clothos Maiden Queen Carcass".to_owned(), 4),
            ("Green Jambura Juice".to_owned(), 3),
            ("Pirum Juice".to_owned(), 3),
            ("Electrum".to_owned(), 3),
            ("Muse Fruit".to_owned(), 3),
            ("Pirum".to_owned(), 2),
            ("Basileus".to_owned(), 2),
            ("White Bear Carcass".to_owned(), 2)
        ]
    }

}