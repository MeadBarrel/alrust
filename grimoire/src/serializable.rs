use serde::Serialize;

use crate::{mix::*, theoretical::*, types::Property};

#[derive(Serialize, Clone, Debug)]
pub struct AlchemyEffectTotal {
    direct: Theoretical<f64>,
    over_time: Theoretical<f64>,
    per_second: Theoretical<f64>,
    length: Theoretical<f64>,
}

#[derive(Serialize, Clone, Debug)]
pub struct AlchemyEffects {
    dh: Theoretical<f64>,
    dp: Theoretical<f64>,
    hot: Theoretical<f64>,
    hl: Theoretical<f64>,
    pot: Theoretical<f64>,
    pl: Theoretical<f64>,
    a: Theoretical<f64>,
}

#[derive(Serialize, Clone, Debug)]
pub struct PotionSerializable {
    volume: f64,
    effects: AlchemyEffects,
    healing: AlchemyEffectTotal,
    poison: AlchemyEffectTotal,
    ingredients: Vec<(String, u64)>,
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
            per_second: effects.hot * Theoretical::from(volume.sqrt()),
            over_time: effects.hot * effects.hl * Theoretical::from(volume),
            length: effects.hl * Theoretical::from(volume.sqrt()),
        };

        let poison = AlchemyEffectTotal {
            direct: effects.dp,
            per_second: effects.pot * Theoretical::from(volume.sqrt()),
            over_time: effects.pot * effects.pl * Theoretical::from(volume),
            length: effects.hl * Theoretical::from(volume.sqrt()),
        };

        let ingredients = mix
            .ingredients
            .iter()
            .map(|x| (x.0.name.clone(), x.1))
            .collect();
        Self {
            volume,
            effects,
            ingredients,
            healing,
            poison,
        }
    }
}
