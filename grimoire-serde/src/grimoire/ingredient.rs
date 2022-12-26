use serde::Serialize;
use crate::theoretical::TheoreticalWrapper;
use grimoire2::grimoire::Ingredient;

#[derive(Serialize)]
pub struct IngredientHumanReadable {
    weight: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    skill: Option<String>,
    #[serde(skip_serializing_if = "TheoreticalWrapper::is_unknown")]
    dh: TheoreticalWrapper,
    #[serde(skip_serializing_if = "TheoreticalWrapper::is_unknown")]
    dp: TheoreticalWrapper,
    #[serde(skip_serializing_if = "TheoreticalWrapper::is_unknown")]
    mdh: TheoreticalWrapper,
    #[serde(skip_serializing_if = "TheoreticalWrapper::is_unknown")]
    mdp: TheoreticalWrapper,
    #[serde(skip_serializing_if = "TheoreticalWrapper::is_unknown")]
    hot: TheoreticalWrapper,
    #[serde(skip_serializing_if = "TheoreticalWrapper::is_unknown")]
    pot: TheoreticalWrapper,
    #[serde(skip_serializing_if = "TheoreticalWrapper::is_unknown")]
    mhot: TheoreticalWrapper,
    #[serde(skip_serializing_if = "TheoreticalWrapper::is_unknown")]
    mpot: TheoreticalWrapper,
    #[serde(skip_serializing_if = "TheoreticalWrapper::is_unknown")]
    hl: TheoreticalWrapper,
    #[serde(skip_serializing_if = "TheoreticalWrapper::is_unknown")]
    pl: TheoreticalWrapper,
    #[serde(skip_serializing_if = "TheoreticalWrapper::is_unknown")]
    mhl: TheoreticalWrapper,
    #[serde(skip_serializing_if = "TheoreticalWrapper::is_unknown")]
    mpl: TheoreticalWrapper,
    #[serde(skip_serializing_if = "TheoreticalWrapper::is_unknown")]
    a: TheoreticalWrapper,
    #[serde(skip_serializing_if = "TheoreticalWrapper::is_unknown")]
    ma: TheoreticalWrapper,    
}

impl From<Ingredient> for IngredientHumanReadable {
    fn from(value: Ingredient) -> Self {
        use grimoire2::effect::Effect;

        Self {
            weight: value.weight,
            skill: value.skill,
            dh: value.modifiers[Effect::DirectHealing].term.into(),
            mdh: value.modifiers[Effect::DirectHealing].multiplier.into(),

            dp: value.modifiers[Effect::DirectPoison].term.into(),
            mdp: value.modifiers[Effect::DirectPoison].multiplier.into(),

            hot: value.modifiers[Effect::HealingOverTime].term.into(),
            mhot: value.modifiers[Effect::HealingOverTime].multiplier.into(),

            pot: value.modifiers[Effect::PoisonOverTime].term.into(),
            mpot: value.modifiers[Effect::PoisonOverTime].multiplier.into(),
            
            hl: value.modifiers[Effect::HealingLength].term.into(),
            mhl: value.modifiers[Effect::HealingLength].multiplier.into(),
            
            pl: value.modifiers[Effect::PoisonLength].term.into(),
            mpl: value.modifiers[Effect::PoisonLength].multiplier.into(),
            
            a: value.modifiers[Effect::Alcohol].term.into(),
            ma: value.modifiers[Effect::Alcohol].multiplier.into(),
        }
    }
}