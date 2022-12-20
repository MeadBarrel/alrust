use std::ops::Index;

use serde::{Serialize, Deserialize};

use super::command::Commands;
use crate::grimoire::Ingredient;
use crate::theoretical::Theoretical;
use crate::effect::Effect;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum IngredientUpdateCommand {
    ChangeMultiplier(Effect, Theoretical<f64>),
    ChangeTerm(Effect, Theoretical<f64>),
    SetSkill(Option<String>),
    SetWeight(bool)
}


#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IngredientUpdate {
    commands: Vec<IngredientUpdateCommand>
}


impl IngredientUpdate {
    pub fn set_skill(&mut self, skill: &str) -> &mut Self {
        self.commands.push(IngredientUpdateCommand::SetSkill(Some(skill.to_string())));
        self
    }

    pub fn remove_skill(&mut self) -> &mut Self {
        self.commands.push(IngredientUpdateCommand::SetSkill(None));
        self
    }

    pub fn set_weight(&mut self, weight: bool) -> &mut Self {
        self.commands.push(IngredientUpdateCommand::SetWeight(weight));
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
        self.commands.push(IngredientUpdateCommand::ChangeTerm(effect, value));
        self
    }

    pub fn set_multiplier(&mut self, effect: Effect, value: Theoretical<f64>) -> &mut Self {
        self.commands.push(IngredientUpdateCommand::ChangeMultiplier(effect, value));
        self
    }    

}


impl Index<usize> for IngredientUpdate {
    type Output = IngredientUpdateCommand;

    fn index(&self, index: usize) -> &Self::Output {
        &self.commands[index]
    }
}


impl Commands<Ingredient, IngredientUpdateCommand> for IngredientUpdate {
    fn create_from(ingredient: &Ingredient) -> Self {
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

    fn create(&self) -> Ingredient {
        let mut ingredient = Ingredient::default();
        self.update(&mut ingredient);
        ingredient
    }

    fn update(&self, ingredient: &mut Ingredient) {
        for command in &self.commands {
            match command {
                IngredientUpdateCommand::ChangeMultiplier(effect, value) => {
                    ingredient.modifiers[*effect].multiplier = *value;
                },
                IngredientUpdateCommand::ChangeTerm(effect, value) => {
                    ingredient.modifiers[*effect].term = *value;
                },
                IngredientUpdateCommand::SetSkill(value) => {
                    ingredient.skill = value.clone()
                },
                IngredientUpdateCommand::SetWeight(value) => {
                    ingredient.weight = *value
                }
            }
        }
    }

    fn add(&mut self, command: IngredientUpdateCommand) -> &mut Self {
        self.commands.push(command);
        self
    }

    fn len(&self) -> usize {
        self.commands.len()
    }

    fn combine_last(&mut self) -> &mut Self {
        use IngredientUpdateCommand::*;

        if self.len() < 2 { return  self; }

        let prev = &self.commands[self.len()-2];
        let last = &self.commands[self.len()-1];

        match (prev, last) {
            (ChangeTerm(a, _), ChangeTerm(b, _)) => if a == b {
                self._replace_last_two_with(last.clone());
            },
            (ChangeMultiplier(a, _), ChangeMultiplier(b, _)) => if a == b {
                self._replace_last_two_with(last.clone())
            },
            (SetWeight(_), SetWeight(_)) => {
                self._replace_last_two_with(last.clone())
            },
            (SetSkill(_), SetSkill(_)) => {
                self._replace_last_two_with(last.clone())
            },
            (_, _) => {},
        }

        self
    }

    fn truncate(&mut self, index: usize) -> &mut Self {
        self.commands.truncate(index);
        self
    }        

}


impl From<Ingredient> for IngredientUpdate {
    fn from(ingredient: Ingredient) -> Self {
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
    use super::Commands;

    #[test]
    fn test_from_ingredient() {
        let ingredient = ingredient_updater().create();
        let update = IngredientUpdate::create_from(&ingredient);
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

    #[test]
    fn test_combine_last_set_remove_skill() {
        let update = IngredientUpdate::default()
            .set_skill("a")
            .remove_skill()
            .combine_last()
            .clone();
        let ingredient = &mut IngredientUpdate::default().set_skill("b").create();
        update.update(ingredient);
        assert_eq!(update.len(), 1);
        assert!(ingredient.skill.is_none());
    }

    #[test]
    fn test_combine_last_remove_set_skill() {
        let update = IngredientUpdate::default()
            .remove_skill()
            .set_skill("a")
            .combine_last()
            .clone();
        let ingredient = &mut IngredientUpdate::default().set_skill("b").create();
        update.update(ingredient);
        assert_eq!(update.len(), 1);
        assert_eq!(ingredient.skill, Some("a".to_string()));        
    }

    #[test]
    fn test_combine_last_set_term() {
        let update = IngredientUpdate::default()
            .set_term(Effect::Alcohol, Theoretical::Known(1.))
            .set_term(Effect::Alcohol, Theoretical::Known(2.))
            .combine_last()
            .clone();
        let ingredient = &mut IngredientUpdate::default().set_skill("b").create();
        update.update(ingredient);
        assert_eq!(update.len(), 1);
        assert_eq!(ingredient.modifiers[Effect::Alcohol].term, Theoretical::Known(2.));        
    }

    #[test]
    fn test_combine_last_set_multiplier() {
        let update = IngredientUpdate::default()
            .set_multiplier(Effect::Alcohol, Theoretical::Known(1.))
            .set_multiplier(Effect::Alcohol, Theoretical::Known(2.))
            .combine_last()
            .clone();
        let ingredient = &mut IngredientUpdate::default().set_skill("b").create();
        update.update(ingredient);
        assert_eq!(update.len(), 1);
        assert_eq!(ingredient.modifiers[Effect::Alcohol].multiplier, Theoretical::Known(2.));        
    }    

    #[test]
    fn test_combine_last_set_term_diff_effects() {
        let update = IngredientUpdate::default()
            .set_term(Effect::Alcohol, Theoretical::Known(1.))
            .set_term(Effect::DirectHealing, Theoretical::Known(2.))
            .combine_last()
            .clone();
        let ingredient = &mut IngredientUpdate::default().set_skill("b").create();
        update.update(ingredient);
        assert_eq!(update.len(), 2);
        assert_eq!(ingredient.modifiers[Effect::Alcohol].term, Theoretical::Known(1.));
        assert_eq!(ingredient.modifiers[Effect::DirectHealing].term, Theoretical::Known(2.));
    }

    #[test]
    fn test_combine_last_set_multiplier_diff_effects() {
        let update = IngredientUpdate::default()
            .set_multiplier(Effect::Alcohol, Theoretical::Known(1.))
            .set_multiplier(Effect::DirectHealing, Theoretical::Known(2.))
            .combine_last()
            .clone();
        let ingredient = &mut IngredientUpdate::default().set_skill("b").create();
        update.update(ingredient);
        assert_eq!(update.len(), 2);
        assert_eq!(ingredient.modifiers[Effect::Alcohol].multiplier, Theoretical::Known(1.));
        assert_eq!(ingredient.modifiers[Effect::DirectHealing].multiplier, Theoretical::Known(2.));
    }        
}



pub mod versioned {
    use serde::{Serialize, Deserialize};
    use super::{IngredientUpdate, IngredientUpdateCommand};

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub enum IngredientUpdateVersioned {
        #[serde(rename="0")]
        V0(v0::IngredientUpdateV0)
    }

    impl From<IngredientUpdate> for IngredientUpdateVersioned {
        fn from(value: IngredientUpdate) -> Self {
            Self::V0(value.into())
        }
    }

    impl From<IngredientUpdateVersioned> for IngredientUpdate {
        fn from(value: IngredientUpdateVersioned) -> Self {
            match value {
                IngredientUpdateVersioned::V0(x) => x.into()
            }
        }
    }

    pub mod v0 {
        use super::*;

        use crate::theoretical::versioned::TheoreticalVersioned;
        use crate::effect::Effect;

        #[derive(Clone, Debug, Serialize, Deserialize)]
        pub struct IngredientUpdateV0 {
            commands: Vec<IngredientUpdateCommandV0>
        }

        impl From<IngredientUpdate> for IngredientUpdateV0 {
            fn from(value: IngredientUpdate) -> Self {
                Self {
                    commands: value.commands.into_iter().map(|x| x.into()).collect(),
                }
            }
        }

        impl From<IngredientUpdateV0> for IngredientUpdate {
            fn from(value: IngredientUpdateV0) -> Self {
                Self {
                    commands: value.commands.into_iter().map(|x| x.into()).collect(),
                }                
            }
        }

        #[derive(Clone, Debug, Serialize, Deserialize)]
        pub enum IngredientUpdateCommandV0 {
            ChangeMultiplier(Effect, TheoreticalVersioned<f64>),
            ChangeTerm(Effect, TheoreticalVersioned<f64>),
            SetSkill(Option<String>),
            SetWeight(bool)
        }

        impl From<IngredientUpdateCommand> for IngredientUpdateCommandV0 {
            fn from(value: IngredientUpdateCommand) -> Self {
                match value {
                    IngredientUpdateCommand::ChangeMultiplier(n, v) => 
                        IngredientUpdateCommandV0::ChangeMultiplier(n, v.into()),
                    IngredientUpdateCommand::ChangeTerm(n, v) =>
                        IngredientUpdateCommandV0::ChangeTerm(n, v.into()),
                    IngredientUpdateCommand::SetSkill(n) => 
                        IngredientUpdateCommandV0::SetSkill(n),
                    IngredientUpdateCommand::SetWeight(n) =>
                        IngredientUpdateCommandV0::SetWeight(n)
                }
            }
        }

        impl From<IngredientUpdateCommandV0> for IngredientUpdateCommand {
            fn from(value: IngredientUpdateCommandV0) -> Self {
                match value {
                    IngredientUpdateCommandV0::ChangeMultiplier(n, v) => 
                        IngredientUpdateCommand::ChangeMultiplier(n, v.into()),
                    IngredientUpdateCommandV0::ChangeTerm(n, v) =>
                        IngredientUpdateCommand::ChangeTerm(n, v.into()),
                    IngredientUpdateCommandV0::SetSkill(n) => 
                        IngredientUpdateCommand::SetSkill(n),
                    IngredientUpdateCommandV0::SetWeight(n) =>
                        IngredientUpdateCommand::SetWeight(n)
                }                
            }
        }
    }
}
