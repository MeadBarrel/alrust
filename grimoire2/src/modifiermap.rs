use std::ops::{Index, IndexMut};

use strum::{EnumCount, IntoEnumIterator};

use crate::{effect::Effect, modifier::Modifier};

#[derive(Debug, Clone)]
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
