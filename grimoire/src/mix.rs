use strum::IntoEnumIterator;

use crate::optimized::*;
use crate::types::*;
use crate::theoretical::*;


#[derive(Debug, Clone)]
pub struct Mix {
    pub ingredients: Vec<(Ingredient, u64)>,
    pub advanced_potion_making_mod: f64,
    pub alvarin_clade: bool,
}


impl Mix {
    pub fn new(
        advanced_potion_making_mod: f64,
        alvarin_clade: bool,
        ingredients: Vec<(Ingredient, u64)>
    ) -> Self {
        Self {
            advanced_potion_making_mod,
            alvarin_clade,
            ingredients
        }
    }
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


pub fn mix_effect(mix: &Mix, property: Property) -> Theoretical {
    let total_count: u64 = mix.ingredients.iter().map(|(_, count)| count).sum();

    if total_count == 0 { return Theoretical::from(0.) }

    let mut multiplier = Theoretical::from(1.);

    for (ingredient, count) in &mix.ingredients {
        multiplier = multiplier *
            (Theoretical::from(1.) + Theoretical::from(ingredient.modifiers[property as usize].multiplier) *
            Theoretical::from((count.to_owned() as f64 / total_count as f64).sqrt()))
    }

    let mut sum = Theoretical::from(0.);

    for (ingredient, count) in &mix.ingredients {
        sum = sum +
            Theoretical::from(ingredient.lore_multiplier) *
            Theoretical::from(ingredient.modifiers[property as usize].modifier) *
            Theoretical::from(count.to_owned() as f64 / total_count as f64)
    };

    Theoretical::from(mix.advanced_potion_making_mod) * sum * multiplier
}


#[cfg(test)]
mod tests {
    use float_cmp::approx_eq;

    use super::*;
    use crate::data;


    fn create_compendium() -> data::Compendium {
        let ingredients = vec![
            data::Ingredient::new("Sea Dew Leaves", 1, "Herbology", vec![
                (Property::DirectHealing, Modifier::new(Some(1.2), Some(0.0)))
            ].into_iter().collect()),
            data::Ingredient::new("Argus Sponge", 1, "Herbology", vec![
                (Property::DirectHealing, Modifier::new(Some(0.), Some(0.96))),
                (Property::DirectPoison, Modifier::new(Some(0.979), Some(-0.75)))
            ].into_iter().collect()),
            data::Ingredient::new("Skadite", 0, "Petrology", vec![
                (Property::DirectHealing, Modifier::new(Some(0.), Some(0.96)))
            ].into_iter().collect()),
            data::Ingredient::new("Unknownium", 0, "Petrology", vec![
                (Property::DirectHealing, Modifier::new(None, None))
            ].into_iter().collect())
        ];
        let lores = vec![
            data::Lore::new("Herbology", Some(0.66666), None, None),
            data::Lore::new("Petrology", Some(0.66666), None, None),
        ];
        let characters = vec![
            data::Character::new(
                "default", 
                vec![("Herbology".to_string(), 100u8), ("Petrology".to_string(), 100u8)].into_iter().collect(), 
                100, 
                true,
            )
        ];
        data::Compendium::create_from_vecs(characters, lores, ingredients)
    }


    #[test]
    fn test_mix_dh() {
        let reference = load_data();
        let mix = Mix { alvarin_clade:true, advanced_potion_making_mod: 1.2, ingredients: reference.ingredients_from_names(ingredients()).unwrap() };

        let expected = 3.25;
        let actual = mix_effect(&mix, Property::DirectHealing);

        assert!(actual.is_known());
        assert!( approx_eq!(f64, actual.inner(), expected, epsilon=0.01), "actual: {}", actual.inner() );
    }

    #[test]
    fn test_mix_effects() {
        let reference = load_data();
        let mix = Mix { alvarin_clade: true, advanced_potion_making_mod: 1.2, ingredients: reference.ingredients_from_names(ingredients()).unwrap() };
        let actual = mix_effects(&mix);

        assert!( approx_eq!(f64, actual[Property::DirectHealing as usize].inner(), 3.25, epsilon=0.01) );
    }


    #[test]
    fn test_mix_volume() {
        let reference = load_data();
        let mix = Mix { alvarin_clade: true, advanced_potion_making_mod: 1.2, ingredients: reference.ingredients_from_names(ingredients()).unwrap() };
        let actual = mix_volume(&mix);

        assert!( approx_eq!(f64, actual, 7.3, epsilon=0.01) );
    }

    fn load_data() -> OptimizedGrimoir {
        let compendium = create_compendium();
        let character = &compendium.characters["default"];
        compendium.create_reference(character)
    }

    fn ingredients() -> Vec<(String, u64)> {
        vec![
            ("Sea Dew Leaves".to_owned(), 67),
            ("Argus Sponge".to_owned(), 7),
            ("Skadite".to_owned(), 6),
        ]
    }

}