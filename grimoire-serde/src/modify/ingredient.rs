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
}

impl From<IngredientUpdateSerializable> for IngredientUpdate {
    fn from(value: IngredientUpdateSerializable) -> Self {
        value.to_update()
    }
}
