use serde::Serialize;

use crate::mix::*;
use crate::types::Property;


#[derive(Serialize, Clone, Debug)]
pub struct AlchemyEffectTotal {
    direct: EffectResult,
    over_time: EffectResult,
    per_second: EffectResult,
    length: EffectResult,
}


#[derive(Serialize, Clone, Debug)]
pub struct AlchemyEffects {
    dh: EffectResult,
    dp: EffectResult,
    hot: EffectResult,
    hl: EffectResult,
    pot: EffectResult,
    pl: EffectResult,
    a: EffectResult,
}



#[derive(Serialize, Clone, Debug)]
pub struct PotionSerializable {
    volume: f64,
    effects: AlchemyEffects,
    healing: AlchemyEffectTotal,
    poison: AlchemyEffectTotal,
    ingredients: Vec<(String, u64)>
}


impl PotionSerializable {
    pub fn from_mix(mix: &Mix) -> Self {
        let volume = mix_volume(mix);

        let effects = AlchemyEffects {
            dh: mix_effect(mix, Property::DirectHealing),
            dp: mix_effect(mix, Property::DirectPoison),
            hot: mix_effect(mix, Property::HealingOverTime),
            hl: mix_effect(mix, Property::HealingLength),
            pot: mix_effect(mix, Property::PoisonOverTime),
            pl: mix_effect(mix, Property::PoisonLength),
            a: mix_effect(mix, Property::Alcohol),
        };

        let healing = AlchemyEffectTotal {
            direct: effects.dh,
            per_second: effects.hot * EffectResult::from(volume.sqrt()),
            over_time: effects.hot * effects.hl * EffectResult::from(volume),
            length: effects.hl * EffectResult::from(volume.sqrt()),
        };

        let poison = AlchemyEffectTotal {
            direct: effects.dp,
            per_second: effects.pot * EffectResult::from(volume.sqrt()),
            over_time: effects.pot * effects.pl * EffectResult::from(volume),
            length: effects.hl * EffectResult::from(volume.sqrt()),
        };

        let ingredients = mix.ingredients.iter().map(|x| (x.0.name.clone(), x.1)).collect();
        Self {
            volume, effects, ingredients, healing, poison
        }
    }
}