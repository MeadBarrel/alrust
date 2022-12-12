use serde::Serialize;

use crate::mix::*;
use crate::types::Property;


#[derive(Serialize, Clone, Debug)]
pub struct AlchemyEffectTotal {
    direct: f64,
    over_time: f64,
    per_second: f64,
    length: f64,
}


#[derive(Serialize, Clone, Debug)]
pub struct AlchemyEffects {
    dh: f64,
    dp: f64,
    hot: f64,
    hl: f64,
    pot: f64,
    pl: f64,
    a: f64,
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
            per_second: effects.hot * volume.sqrt(),
            over_time: effects.hot * effects.hl * volume,
            length: effects.hl * volume.sqrt(),
        };

        let poison = AlchemyEffectTotal {
            direct: effects.dp,
            per_second: effects.pot * volume.sqrt(),
            over_time: effects.pot * effects.pl * volume,
            length: effects.hl * volume.sqrt(),
        };

        let ingredients = mix.ingredients.iter().map(|x| (x.0.name.clone(), x.1)).collect();
        Self {
            volume, effects, ingredients, healing, poison
        }
    }
}