use std::collections::HashMap;

use strum::IntoEnumIterator;

use crate::grimoire::Ingredient;
use crate::theoretical::Theoretical;
use crate::effect::{Effect, self};



#[derive(Default, Clone, Debug)]
pub struct IngredientUpdate {
    multiplier_actions: HashMap<Effect, Theoretical<f64>>,
    term_actions: HashMap<Effect, Theoretical<f64>>,
    skill: Option<Option<String>>,
    weight: Option<bool>,
}


impl IngredientUpdate {
    pub fn create(&self) -> Ingredient {
        let mut ingredient = Ingredient::default();
        self.update(&mut ingredient);
        ingredient
    }

    pub fn from_ingredient(ingredient: &Ingredient) -> Self {
        let mut result = Self::default();
        ingredient.modifiers.iter().for_each(
            |(effect, modifier)|
            { result.set_modifier(effect, modifier.term, modifier.multiplier); }
        );
        match &ingredient.skill {
            Some(x) => result.set_skill(x),
            None => result.remove_skill(),
        };

        result.set_weight(ingredient.weight);
        result
    }

    pub fn update(&self, ingredient: &mut Ingredient) {
        if let Some(x) = &self.skill {
            ingredient.skill = x.clone();
        }

        if let Some(x) = self.weight {
            ingredient.weight = x;
        }

        self.term_actions.iter().for_each(
            |(effect, value)|
            { ingredient.modifiers[*effect].term = *value }
        );

        self.multiplier_actions.iter().for_each(
            |(effect, value)|
            { ingredient.modifiers[*effect].multiplier = *value }
        );       

        // self.term_actions.iter().for_each(|(effect, action)|
        //     match action {
        //         ModifierUpdate::To(to) => ingredient.modifiers[*effect].term = *to,
        //         ModifierUpdate::ToKnown => ingredient.modifiers[*effect].term = 
        //             ingredient.modifiers[*effect].term.to_known(),
        //         ModifierUpdate::ToUnknown => ingredient.modifiers[*effect].term =
        //             ingredient.modifiers[*effect].term.to_unknown(),
        //     }
        // );

        // self.multiplier_actions.iter().for_each(|(effect, action)|
        //     match action {
        //         ModifierUpdate::To(to) => ingredient.modifiers[*effect].multiplier = *to,
        //         ModifierUpdate::ToKnown => ingredient.modifiers[*effect].multiplier = 
        //             ingredient.modifiers[*effect].multiplier.to_known(),
        //         ModifierUpdate::ToUnknown => ingredient.modifiers[*effect].multiplier =
        //             ingredient.modifiers[*effect].multiplier.to_unknown(),
        //     }
        // );

    }

    pub fn set_skill(&mut self, skill: &str) -> &mut Self {
        self.skill = Some(Some(skill.to_string()));
        self
    }

    pub fn remove_skill(&mut self) -> &mut Self {
        self.skill = Some(None);
        self
    }

    pub fn set_weight(&mut self, weight: bool) -> &mut Self {
        self.weight = Some(weight);
        self
    }

    pub fn set_modifier(
        &mut self, 
        effect: Effect, 
        term: Theoretical<f64>, 
        multiplier: Theoretical<f64>
    ) -> &mut Self {
        self.set_term(effect, term);
        self.set_multiplier(effect, multiplier);
        self
    }

    pub fn set_term(&mut self, effect: Effect, value: Theoretical<f64>) -> &mut Self {
        self.term_actions.insert(effect, value);
        self
    }

    pub fn set_multiplier(&mut self, effect: Effect, value: Theoretical<f64>) -> &mut Self {
        self.multiplier_actions.insert(effect, value);        
        self
    }    

    pub fn will_set_skill(&self) -> Option<Option<String>> {
        self.skill.clone()
    }

    pub fn will_set_weight(&self) -> Option<bool> {
        self.weight
    }

    pub fn will_set_term(&self, effect: Effect) -> Option<Theoretical<f64>> {
        self.term_actions.get(&effect).copied()
    }

    pub fn will_set_multiplier(&self, effect: Effect) -> Option<Theoretical<f64>> {
        self.multiplier_actions.get(&effect).copied()
    }

}


impl From<Ingredient> for IngredientUpdate {
    fn from(ingredient: Ingredient) -> Self {
        Self::from_ingredient(&ingredient)
    }
}


impl From<IngredientUpdate> for Ingredient {
    fn from(value: IngredientUpdate) -> Self {
        value.create()
    }
}


#[cfg(test)]
mod tests {
    use crate::grimoire::Ingredient;
    use crate::effect::Effect;
    use crate::theoretical::Theoretical;

    use super::IngredientUpdate;

    #[test]
    fn test_from_ingredient() {
        let ingredient = ingredient_updater().create();
        let update = IngredientUpdate::from_ingredient(&ingredient);
        let new_ingredient = update.create();

        assert_eq!( new_ingredient.modifiers[Effect::DirectHealing].term, (1.0).into() );
        assert_eq!( new_ingredient.modifiers[Effect::DirectHealing].multiplier, (1.0).into() );

        assert_eq!( new_ingredient.modifiers[Effect::DirectPoison].term, (2.0).into() );
        assert_eq!( new_ingredient.modifiers[Effect::DirectPoison].multiplier, Theoretical::Unknown );

        assert_eq!( new_ingredient.modifiers[Effect::HealingOverTime].term, Theoretical::Unknown );
        assert_eq!( new_ingredient.modifiers[Effect::HealingOverTime].multiplier, (3.0).into() );

        assert_eq!( new_ingredient.modifiers[Effect::PoisonOverTime].term, Theoretical::Theory(4.0) );
        assert_eq!( new_ingredient.modifiers[Effect::PoisonOverTime].multiplier, Theoretical::Theory(4.0) );

        assert_eq!( new_ingredient.skill, Some("skill".to_string()) );
        assert!( new_ingredient.weight );

    }  

    fn ingredient_updater() -> IngredientUpdate {
        IngredientUpdate::default()
            .set_term(Effect::DirectHealing, (1.0).into())
            .set_multiplier(Effect::DirectHealing, (1.0).into())

            .set_term(Effect::DirectPoison, (2.0).into())

            .set_multiplier(Effect::HealingOverTime, (3.0).into())

            .set_term(Effect::PoisonOverTime, Theoretical::Theory(4.0))
            .set_multiplier(Effect::PoisonOverTime, Theoretical::Theory(4.0))

            .set_skill("skill")
            .set_weight(true)
            .clone()
    }

    #[test]
    fn test_set_term() {
        let mut ingredient = Ingredient::default();
        IngredientUpdate::default()
            .set_term(Effect::Alcohol, Theoretical::Known(0.5)).update(&mut ingredient);
        assert!( ingredient.modifiers[Effect::Alcohol].term.is_known() );
        assert_eq!( ingredient.modifiers[Effect::Alcohol].term.inner(), 0.5 );
    }

    #[test]
    fn test_set_multiplier() {
        let mut ingredient = Ingredient::default();
        IngredientUpdate::default()
            .set_multiplier(Effect::Alcohol, Theoretical::Known(0.5)).update(&mut ingredient);
        assert!( ingredient.modifiers[Effect::Alcohol].multiplier.is_known() );
        assert_eq!( ingredient.modifiers[Effect::Alcohol].multiplier.inner(), 0.5 );        
    }

    #[test]
    fn test_set_weight() {
        let mut ingredient = Ingredient::default();
        IngredientUpdate::default().set_weight(true).update(&mut ingredient);
        assert!( ingredient.weight );
    }

    #[test]
    fn test_set_skill() {
        let mut ingredient = Ingredient::default();
        IngredientUpdate::default()
            .set_skill("some skill").update(&mut ingredient);
        assert_eq!( Some("some skill".to_string()), ingredient.skill )
    }

    #[test]
    fn test_remove_skill() {
        let mut ingredient = IngredientUpdate::default()
            .set_skill("some_skill").create();
        IngredientUpdate::default().remove_skill().update(&mut ingredient);
        assert!( ingredient.skill.is_none() )
    }
}