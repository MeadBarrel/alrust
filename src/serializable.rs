use serde::Serialize;

use grimoire2::prelude::{Theoretical, Mix, Effect};

use crate::theoretical::TheoreticalWrapper;


#[derive(Serialize, Clone, Debug)]
pub struct PotionSerializable {
    volume: f64,
    effects: AlchemyEffects,
    healing: AlchemyEffectTotal,
    poison: AlchemyEffectTotal,
    ingredients: Vec<(String, u64)>,
}


#[derive(Serialize, Clone, Debug)]
pub struct AlchemyEffectTotal {
    direct: TheoreticalWrapper<f64>,
    over_time: TheoreticalWrapper<f64>,
    per_second: TheoreticalWrapper<f64>,
    length: TheoreticalWrapper<f64>,
}

#[derive(Serialize, Clone, Debug)]
pub struct AlchemyEffects {
    dh: TheoreticalWrapper<f64>,
    dp: TheoreticalWrapper<f64>,
    hot: TheoreticalWrapper<f64>,
    hl: TheoreticalWrapper<f64>,
    pot: TheoreticalWrapper<f64>,
    pl: TheoreticalWrapper<f64>,
    a: TheoreticalWrapper<f64>,
}

impl PotionSerializable {
    pub fn from_mix(mix: &Mix) -> Self {
        let volume = mix.volume();

        let effects = AlchemyEffects {
            dh: TheoreticalWrapper(mix.effect(Effect::DirectHealing)),
            dp: TheoreticalWrapper(mix.effect(Effect::DirectPoison)),
            hot: TheoreticalWrapper(mix.effect(Effect::HealingOverTime)),
            pot: TheoreticalWrapper(mix.effect(Effect::PoisonOverTime)),
            hl: TheoreticalWrapper(mix.effect(Effect::HealingLength)),
            pl: TheoreticalWrapper(mix.effect(Effect::PoisonLength)),
            a: TheoreticalWrapper(mix.effect(Effect::Alcohol)),
        };

        let healing = AlchemyEffectTotal {
            direct: effects.dh,
            per_second: TheoreticalWrapper(effects.hot.0 * Theoretical::from(volume.sqrt())),
            over_time: TheoreticalWrapper(effects.hot.0 * effects.hl.0 * Theoretical::from(volume)),
            length: TheoreticalWrapper(effects.hl.0 * Theoretical::from(volume.sqrt())),
        };

        let poison = AlchemyEffectTotal {
            direct: effects.dp,
            per_second: TheoreticalWrapper(effects.pot.0 * Theoretical::from(volume.sqrt())),
            over_time: TheoreticalWrapper(effects.pot.0 * effects.pl.0 * Theoretical::from(volume)),
            length: TheoreticalWrapper(effects.hl.0 * Theoretical::from(volume.sqrt())),
        };

        let ingredients = mix
            .named_ingredients_iter()
            .map(|x| (x.0.to_string(), x.1))
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
