use serde::{Serialize, Deserialize};

use grimoire2::modify::ingredient::IngredientUpdate;
use grimoire2::effect::Effect;

use crate::theoretical::TheoreticalWrapper;


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct IngredientUpdateSerializable {
    skill: Option<String>,
    remove_skill: bool,

    weight: Option<bool>,

    dh: Option<TheoreticalWrapper>,
    dp: Option<TheoreticalWrapper>,
    mdh: Option<TheoreticalWrapper>,
    mdp: Option<TheoreticalWrapper>,
    hot: Option<TheoreticalWrapper>,
    pot: Option<TheoreticalWrapper>,
    mhot: Option<TheoreticalWrapper>,
    mpot: Option<TheoreticalWrapper>,
    hl: Option<TheoreticalWrapper>,
    pl: Option<TheoreticalWrapper>,
    mhl: Option<TheoreticalWrapper>,
    mpl: Option<TheoreticalWrapper>,
    a: Option<TheoreticalWrapper>,
    ma: Option<TheoreticalWrapper>,
}


impl IngredientUpdateSerializable {
    pub fn to_update(&self) -> IngredientUpdate {
        let mut update = IngredientUpdate::default();
        if let Some(x) = self.skill.clone() { update.set_skill(&x); }
        if self.remove_skill { update.remove_skill(); }

        if let Some(x) = self.dh { update.set_term(Effect::DirectHealing, x.into()); }
        if let Some(x) = self.mdh { update.set_multiplier(Effect::DirectHealing, x.into()); }
        
        if let Some(x) = self.dp { update.set_term(Effect::DirectPoison, x.into()); }
        if let Some(x) = self.mdp { update.set_multiplier(Effect::DirectPoison, x.into()); }

        if let Some(x) = self.hot { update.set_term(Effect::HealingOverTime, x.into()); }
        if let Some(x) = self.mhot { update.set_multiplier(Effect::HealingOverTime, x.into()); }
        
        if let Some(x) = self.pot { update.set_term(Effect::PoisonOverTime, x.into()); }
        if let Some(x) = self.mpot { update.set_multiplier(Effect::PoisonOverTime, x.into()); }
        
        if let Some(x) = self.hl { update.set_term(Effect::HealingLength, x.into()); }
        if let Some(x) = self.mhl { update.set_multiplier(Effect::HealingLength, x.into()); }
        
        if let Some(x) = self.pl { update.set_term(Effect::PoisonLength, x.into()); }
        if let Some(x) = self.mpl { update.set_multiplier(Effect::PoisonLength, x.into()); }
        
        if let Some(x) = self.a { update.set_term(Effect::Alcohol, x.into()); }
        if let Some(x) = self.ma { update.set_multiplier(Effect::Alcohol, x.into()); }

        update
    }

    pub fn from_update(update: &IngredientUpdate) -> Self {
        let mut result = Self::default();

        match update.will_set_skill() {
            Some(None) => result.remove_skill = true,
            Some(Some(x)) => result.skill = Some(x),
            None => {},
        }

        result.weight = update.will_set_weight();

        result.dh = update.will_set_term(Effect::DirectHealing).map(|x| x.into());
        result.mdh = update.will_set_multiplier(Effect::DirectHealing).map(|x| x.into());        

        result.dp = update.will_set_term(Effect::DirectPoison).map(|x| x.into());
        result.mdp = update.will_set_multiplier(Effect::DirectPoison).map(|x| x.into());        

        result.hot = update.will_set_term(Effect::HealingOverTime).map(|x| x.into());
        result.mhot = update.will_set_multiplier(Effect::HealingOverTime).map(|x| x.into());        

        result.pot = update.will_set_term(Effect::PoisonOverTime).map(|x| x.into());
        result.mpot = update.will_set_multiplier(Effect::PoisonOverTime).map(|x| x.into());        

        result.hl = update.will_set_term(Effect::HealingLength).map(|x| x.into());
        result.mhl = update.will_set_multiplier(Effect::HealingLength).map(|x| x.into());        

        result.pl = update.will_set_term(Effect::PoisonLength).map(|x| x.into());
        result.mpl = update.will_set_multiplier(Effect::PoisonLength).map(|x| x.into());        

        result.a = update.will_set_term(Effect::Alcohol).map(|x| x.into());
        result.ma = update.will_set_multiplier(Effect::Alcohol).map(|x| x.into());        

        result
    }
}


impl From<IngredientUpdate> for IngredientUpdateSerializable {
    fn from(value: IngredientUpdate) -> Self {
        Self::from_update(&value)
    }
}


impl From<IngredientUpdateSerializable> for IngredientUpdate {
    fn from(value: IngredientUpdateSerializable) -> Self {
        value.to_update()
    }
}


#[cfg(test)]
mod tests {

    use grimoire2::grimoire::Ingredient;
    use grimoire2::modify::ingredient::IngredientUpdate;
    use grimoire2::effect::Effect;
    use grimoire2::theoretical::Theoretical;
    use crate::theoretical::TheoreticalWrapper;

    use rstest::rstest;

    use super::*;

    #[test]
    fn test_to_update_set_skill() {
        let ser_update = IngredientUpdateSerializable {
            skill: Some("a".to_string()),
            ..Default::default()
        };
        let character = ser_update.to_update().create();
        assert_eq!(character.skill, Some("a".to_string()));
    }

    #[test]
    fn test_to_update_remove_skill() {
        let ser_update = IngredientUpdateSerializable {
            remove_skill: true,
            ..Default::default()
        };        
        let mut ingredient = IngredientUpdate::default().set_skill("a").create();
        ser_update.to_update().update(&mut ingredient);
        assert!(ingredient.skill.is_none())
    }

    #[rstest]
    #[case(TheoreticalWrapper::Known(0.5))]
    #[case(TheoreticalWrapper::Theory(0.5))]
    #[case(TheoreticalWrapper::Unknown)]
    fn test_to_update_set_term(#[case] value: TheoreticalWrapper) {
        let ser_update = IngredientUpdateSerializable {
            a: Some(value),
            ..Default::default()
        };
        let mut ingredient = Ingredient::default();
        ser_update.to_update().update(&mut ingredient);
        assert_eq!(ingredient.modifiers[Effect::Alcohol].term, value.into());
    }

    #[rstest]
    #[case(TheoreticalWrapper::Known(0.5))]
    #[case(TheoreticalWrapper::Theory(0.5))]
    #[case(TheoreticalWrapper::Unknown)]
    fn test_to_update_set_multiplier(#[case] value: TheoreticalWrapper) {
        let ser_update = IngredientUpdateSerializable {
            ma: Some(value),
            ..Default::default()
        };
        let mut ingredient = Ingredient::default();
        ser_update.to_update().update(&mut ingredient);
        assert_eq!(ingredient.modifiers[Effect::Alcohol].multiplier, value.into());
    }

    #[test]
    fn test_from_update_set_skill() {
        let update = IngredientUpdate::default().set_skill("a").clone();
        let ser_update = IngredientUpdateSerializable::from_update(&update);
        assert_eq!(ser_update.skill, Some("a".to_string()));
        assert!(!ser_update.remove_skill);
    }

    #[test]
    fn test_from_update_remove_skill() {
        let update = IngredientUpdate::default().remove_skill().clone();
        let ser_update = IngredientUpdateSerializable::from_update(&update);
        assert!(ser_update.remove_skill);
    }

    #[rstest]
    #[case(TheoreticalWrapper::Known(0.5))]
    #[case(TheoreticalWrapper::Theory(0.5))]
    #[case(TheoreticalWrapper::Unknown)]
    fn test_from_update_set_term(#[case] value: TheoreticalWrapper) {
        let update = IngredientUpdate::default().set_term(Effect::Alcohol, value.into()).clone();
        let ser_update = IngredientUpdateSerializable::from_update(&update);
        assert_eq!(ser_update.a, Some(value));
    }

    #[rstest]
    #[case(TheoreticalWrapper::Known(0.5))]
    #[case(TheoreticalWrapper::Theory(0.5))]
    #[case(TheoreticalWrapper::Unknown)]
    fn test_from_update_set_multiplier(#[case] value: TheoreticalWrapper) {
        let update = IngredientUpdate::default().set_multiplier(Effect::Alcohol, value.into()).clone();
        let ser_update = IngredientUpdateSerializable::from_update(&update);
        assert_eq!(ser_update.ma, Some(value));
    }    
}