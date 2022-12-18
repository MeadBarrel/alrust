use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use grimoire2::standalone::Mix;
use grimoire2::effect::Effect;

use crate::theoretical::TheoreticalWrapper;
use crate::mix::MixIngredients;


#[derive(Debug, Clone, Serialize)]
pub struct PotionSerializable {
    #[serde(skip_serializing_if = "Option::is_none")]
    volume: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    effects: Option<PotionEffectsSerializable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ingredients: Option<MixIngredients>,
    #[serde(skip_serializing_if = "Option::is_none")]
    total_healing_raw: Option<TotalEffect>,
    #[serde(skip_serializing_if = "Option::is_none")]
    total_poison_raw: Option<TotalEffect>,
    #[serde(skip_serializing_if = "Option::is_none")]
    total_healing: Option<TotalEffect>,
    #[serde(skip_serializing_if = "Option::is_none")]
    total_poison: Option<TotalEffect>
}


#[derive(Debug, Clone, Serialize)]
pub struct PotionEffectsSerializable {
    dh: TheoreticalWrapper,
    dp: TheoreticalWrapper,
    hot: TheoreticalWrapper,
    pot: TheoreticalWrapper,
    hl: TheoreticalWrapper,
    pl: TheoreticalWrapper,
    a: TheoreticalWrapper
}


#[derive(Debug, Clone, Serialize)]
pub struct TotalEffect {
    over_time_total: TheoreticalWrapper,
    per_second_total: TheoreticalWrapper,
    length_total: TheoreticalWrapper,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct PotionSerializableConfig {
    volume: bool,
    effects: bool,
    ingredients: bool,
    total_healing_raw: bool,
    total_poison_raw: bool,
    total_healing: bool,
    total_poison: bool,
}


impl Default for PotionSerializableConfig {
    fn default() -> Self {
        Self {
            volume: true,
            effects: true,
            ingredients: true,
            total_healing_raw: false,
            total_poison_raw: false,
            total_healing: false,
            total_poison: false,
        }
    }
}


impl PotionSerializableConfig {
    pub fn serialize_mix(&self, mix: &Mix) -> PotionSerializable {
        let volume = self.volume.then_some(self.serialize_volume(mix));
        let effects = self.effects.then_some(self.serialize_effects(mix));
        let ingredients = self.ingredients.then_some(self.serialize_ingredients(mix));
        let total_healing_raw = 
            self.total_healing_raw.then_some(self.serialize_total_healing_raw(mix));
        let total_poison_raw = 
            self.total_poison_raw.then_some(self.serialize_total_poison_raw(mix));
        let total_healing = self.total_healing.then_some(self.serialize_total_healing(mix));
        let total_poison = self.total_poison.then_some(self.serialize_total_poison(mix));

        PotionSerializable { 
            volume, 
            effects, 
            ingredients, 
            total_healing_raw, 
            total_poison_raw, 
            total_healing, 
            total_poison 
        }
    }

    pub fn serialize_volume(&self, mix: &Mix) -> f64 {
        mix.volume()
    }

    pub fn serialize_effects(&self, mix: &Mix) -> PotionEffectsSerializable {
        let dh = mix.effect(Effect::DirectHealing);
        let dp = mix.effect(Effect::DirectPoison);
        let hot = mix.effect(Effect::HealingOverTime);
        let pot = mix.effect(Effect::PoisonOverTime);
        let hl = mix.effect(Effect::HealingLength);
        let pl = mix.effect(Effect::PoisonLength);
        let a = mix.effect(Effect::Alcohol);

        PotionEffectsSerializable { 
            dh: dh.into(), 
            dp: dp.into(), 
            hot: hot.into(), 
            pot: pot.into(), 
            hl: hl.into(), 
            pl: pl.into(), 
            a: a.into() 
        }
    }

    pub fn serialize_ingredients(&self, mix: &Mix) -> HashMap<String, u64> {
        mix.named_ingredients_iter().map(|(n, a)| (n.to_string(), a)).collect()
    }

    pub fn serialize_total_healing_raw(&self, mix: &Mix) -> TotalEffect {
        let hot = mix.effect(Effect::HealingOverTime);
        let hl = mix.effect(Effect::HealingLength);
        let volume = mix.volume();

        TotalEffect { 
            over_time_total: ((hot * hl) * volume.into()).into(), 
            per_second_total: (hot * volume.sqrt().into()).into(), 
            length_total: (hl * volume.sqrt().into()).into(),
        }
    } 

    pub fn serialize_total_poison_raw(&self, mix: &Mix) -> TotalEffect {
        let pot = mix.effect(Effect::PoisonOverTime);
        let pl = mix.effect(Effect::PoisonLength);
        let volume = mix.volume();

        TotalEffect { 
            over_time_total: ((pot * pl) * volume.into()).into(), 
            per_second_total: (pot * volume.sqrt().into()).into(), 
            length_total: (pl * volume.sqrt().into()).into(),
        }
    }    
    
    pub fn serialize_total_healing(&self, mix: &Mix) -> TotalEffect {
        let hot = mix.effect(Effect::HealingOverTime);
        let hl = mix.effect(Effect::HealingLength);
        let pot = mix.effect(Effect::PoisonOverTime);
        let pl = mix.effect(Effect::PoisonLength);
        let volume = mix.volume();

        TotalEffect { 
            over_time_total: ((hot * hl) * volume.into() - (pot * pl) * volume.into()).into(),
            per_second_total: (hot * volume.sqrt().into() - pot * volume.sqrt().into()).into(),
            length_total: (hl * volume.sqrt().into()).into(),
        }
    }

    pub fn serialize_total_poison(&self, mix: &Mix) -> TotalEffect {
        let hot = mix.effect(Effect::HealingOverTime);
        let hl = mix.effect(Effect::HealingLength);
        let pot = mix.effect(Effect::PoisonOverTime);
        let pl = mix.effect(Effect::PoisonLength);
        let volume = mix.volume();

        TotalEffect { 
            over_time_total: ((pot * pl) * volume.into() - (hot * hl) * volume.into()).into(),
            per_second_total: (pot * volume.sqrt().into() - hot * volume.sqrt().into()).into(),
            length_total: (pl * volume.sqrt().into()).into(),
        }
    }    
}


