use std::ops::{Index, IndexMut};

use serde::{Serialize, Deserialize};
use strum::{EnumCount, IntoEnumIterator};

use crate::{effect::Effect, modifier::Modifier};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ModifierMap(Vec<Modifier>);

impl ModifierMap {
    pub fn size(&self) -> usize {
        Effect::COUNT
    }

    pub fn iter(&self) -> impl Iterator<Item = (Effect, &Modifier)> {
        Effect::iter().zip(self.0.iter())
    }
}


impl From<Vec<(Effect, Modifier)>> for ModifierMap {
    fn from(src: Vec<(Effect, Modifier)>) -> Self {
        let mut result = Self::default();

        for (effect, modifier) in src.into_iter() {
            result[effect] = modifier;
        }

        result
    }
}

impl From<Vec<(Effect, f64, f64)>> for ModifierMap {
    fn from(src: Vec<(Effect, f64, f64)>) -> Self {
        let mut result = Self::default();

        for (effect, term, multiplier) in src.into_iter() {
            result[effect] = Modifier::new_known(term, multiplier);
        }

        result
    }
}

impl Default for ModifierMap {
    fn default() -> Self {
        Self(Effect::iter().map(|_| Modifier::default()).collect())
    }
}

impl Index<Effect> for ModifierMap {
    type Output = Modifier;

    fn index(&self, index: Effect) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl IndexMut<Effect> for ModifierMap {
    fn index_mut(&mut self, index: Effect) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}


pub mod versioned {
    use serde::{Serialize, Deserialize};

    use super::ModifierMap;
    use crate::modifier::versioned::ModifierVersioned;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ModifierMapV0(Vec<ModifierVersioned>);

    impl From<ModifierMap> for ModifierMapV0 {
        fn from(value: ModifierMap) -> Self {
            Self(value.0.into_iter().map(|x| x.into()).collect())
        }
    }

    impl From<ModifierMapV0> for ModifierMap {
        fn from(value: ModifierMapV0) -> Self {
            Self(value.0.into_iter().map(|x| x.into()).collect())
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum ModifierMapVersioned {
        #[serde(rename="0")]
        V0(ModifierMapV0)
    }

    impl From<ModifierMap> for ModifierMapVersioned {
        fn from(value: ModifierMap) -> Self {
            Self::V0(value.into())
        }
    }

    impl From<ModifierMapVersioned> for ModifierMap {
        fn from(value: ModifierMapVersioned) -> Self {
            match value {
                ModifierMapVersioned::V0(x) => x.into()
            }
        }
    }
}


#[cfg(test)]
pub mod tests {
    use proptest::strategy::Strategy;
    use crate::modifier::tests::modifier_strategy;
    use super::*;
    
    pub fn modifier_map_strategy() -> impl Strategy<Value=ModifierMap> {
        (
            modifier_strategy(),
            modifier_strategy(),
            modifier_strategy(),
            modifier_strategy(),
            modifier_strategy(),
            modifier_strategy(),
            modifier_strategy(),
        ).prop_map(|mods| 
            vec![
                (Effect::DirectHealing, mods.0),
                (Effect::DirectPoison, mods.1),
                (Effect::HealingOverTime, mods.2),
                (Effect::PoisonOverTime, mods.3),
                (Effect::HealingLength, mods.4),
                (Effect::PoisonLength, mods.5),
                (Effect::Alcohol, mods.6),
            ].into()
        )
    }
}