use grimoire::types::Property;
use grimoire::mix::*;

use crate::prelude::AlchemyFitnessElement;


pub struct AlchemyEffectFitness {
    effect: Property,
    reverse: bool,
}


impl AlchemyEffectFitness {
    pub fn new(effect: Property, reverse: bool) -> AlchemyEffectFitness {
        AlchemyEffectFitness {
            effect,
            reverse,
        }
    }
}


impl AlchemyFitnessElement for AlchemyEffectFitness {
    fn fitness(&self, mix: &grimoire::prelude::Mix) -> f64 {
        let result = mix_effect(mix, self.effect);
        if self.reverse { return -result }
        result
    }
}


pub struct DesiredVolumeConstraint {
    volume: f64,
}


impl DesiredVolumeConstraint {
    pub fn new(volume: f64) -> Self {
        Self {
            volume
        }
    }
}


impl AlchemyFitnessElement for DesiredVolumeConstraint {
    fn fitness(&self, mix: &Mix) -> f64 {
        let volume = mix_volume(mix);
        0. - (volume - self.volume).abs()
    }
}