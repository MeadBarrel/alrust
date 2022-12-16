use crate::prelude::{Effect, Theoretical};

use super::{OptimizedGrimoire, StandaloneIngredient};


type MixedIngredients = Vec<(usize, u64)>;


#[derive(Debug, Clone)]
pub struct Mix<'a> {
    grimoire: &'a OptimizedGrimoire,
    ingredients: MixedIngredients,
}


impl<'a> Mix<'a> {
    pub fn new(
        grimoire: &'a OptimizedGrimoire,
        ingredients: MixedIngredients,
    ) -> Self {
        Self {
            grimoire, ingredients
        }
    }

    pub fn ingredients_iter(&self) -> impl Iterator<Item=(&StandaloneIngredient, u64)> {
        self.ingredients.iter().map(|(i, a)| (&self.grimoire.ingredients[*i], *a))
    }

    pub fn volume(&self) -> f64 {
        let without_clade = (
            self.ingredients_iter().map(|(i, a)| i.weight as u64 * a).sum::<u64>() - 1
        ) as f64 / 10.;

        if !self.grimoire.alvarin_clade { return without_clade };

        without_clade * 1.1
    }

    pub fn effect(&self, effect: Effect) -> Theoretical<f64> {
        let total_count: u64 = self.ingredients.iter().map(|(_, c)| c).sum();

        if total_count == 0 { return Theoretical::from(0.) }

        let mut multiplier = Theoretical::from(1.);

        for (ingredient, count) in self.ingredients_iter() {
            multiplier = multiplier *
                (Theoretical::from(1.) + ingredient.modifiers[effect].multiplier *
                Theoretical::from((count as f64 / total_count as f64).sqrt()))
        }
    
        let mut sum = Theoretical::from(0.);
    
        for (ingredient, count) in self.ingredients_iter() {
            sum = sum +
                ingredient.lore_multiplier *
                ingredient.modifiers[effect].term *
                Theoretical::from(count as f64 / total_count as f64)
        };
    
        Theoretical::from(self.grimoire.advanced_potion_making_mod) * sum * multiplier
    }
}


#[cfg(test)]
mod tests {
    use float_cmp::approx_eq;

    use crate::prelude::{StandaloneIngredient, Theoretical, ModifierMap, Effect};
    use super::*;

    #[test]
    fn test_mix_dh_noapm() {
        let grimoire = create_grimoire(true, 1.0);
        let mix = Mix::new(&grimoire, vec![(0, 11), (1, 11), (2, 11)]);
        let expected = 2.704;
        let actual = mix.effect(Effect::DirectHealing);
        assert!( actual.is_known() );
        assert!(
            approx_eq!( f64, actual.inner(), expected, epsilon=0.01 ),
            "Expected {}, got {}", expected, actual.inner()
        );
    }

    #[test]
    fn test_mix_dh_apm() {
        let grimoire = create_grimoire(true, 1.2);
        let mix = Mix::new(&grimoire, vec![(0, 11), (1, 11), (2, 11)]);
        let expected = 3.245;
        let actual = mix.effect(Effect::DirectHealing);
        assert!( actual.is_known() );
        assert!(
            approx_eq!( f64, actual.inner(), expected, epsilon=0.01 ),
            "Expected {}, got {}", expected, actual.inner()
        );
    }

    fn create_ingredients() -> Vec<StandaloneIngredient> {
        vec![
            StandaloneIngredient::new(
                1, Theoretical::Known(1.66666), 
                ModifierMap::from(vec![(Effect::DirectHealing, 2.4, 0.)])
            ), 
            StandaloneIngredient::new(
                1, Theoretical::Known(1.66666), 
                ModifierMap::from(vec![(Effect::DirectHealing, 0., 0.64)])
            ), 
            StandaloneIngredient::new(
                1, Theoretical::Known(1.99999), 
                ModifierMap::from(vec![(Effect::DirectHealing, 0.5, 0.32)])
            ), 
            StandaloneIngredient::new(0, Theoretical::default(), ModifierMap::default()),
            StandaloneIngredient::new(0, Theoretical::default(), ModifierMap::default()),
            StandaloneIngredient::new(1, Theoretical::default(), ModifierMap::default()),
            StandaloneIngredient::new(1, Theoretical::default(), ModifierMap::default()),
        ]
    }


    fn create_grimoire(alvarin_clade: bool,  advanced_potion_making_mod: f64) -> OptimizedGrimoire {
        let ingredients = create_ingredients();
        let ingredients_map = ingredients.into_iter().map(|x| ("...".to_string(), x)).into();
        OptimizedGrimoire::new(alvarin_clade, advanced_potion_making_mod, ingredients_map)
    }

    #[test]
    fn test_mix_volume_wo_clade() {
        let expected = 24. / 10.;

        let grimoire = create_grimoire(false, 1.0);
        let mix = Mix::new(&grimoire, vec![(3, 10), (4, 15), (5, 10), (6, 15)]);

        let actual = mix.volume();

        assert!( 
            approx_eq!(f64, actual, expected, epsilon=0.01), 
            "Volume expected {}, but got {}", expected, actual 
        )
    }

    #[test]
    fn test_mix_volume_w_clade() {
        let expected = (24. / 10.) * 1.1;

        let grimoire = create_grimoire(true, 1.0);
        let mix = Mix::new(&grimoire, vec![(3, 10), (4, 15), (5, 10), (6, 15)]);

        let actual = mix.volume();

        assert!( 
            approx_eq!(f64, actual, expected, epsilon=0.01), 
            "Volume expected {}, but got {}", expected, actual 
        )
    }    
}