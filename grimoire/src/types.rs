use strum::EnumCount;
use strum_macros::{EnumIter, EnumCount as EnumCountMacro};


#[derive(Debug, Clone, Copy, EnumIter, EnumCountMacro)]
pub enum Property {
    DirectHealing = 0,
    DirectPoison = 1,
    HealingOverTime = 2,
    PoisonOverTime = 3,
    HealingLength = 4,
    PoisonLength = 5,
    Alcohol = 6,
}


#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Modifier {
    pub modifier: f64,
    pub multiplier: f64,
}


pub type ModifierMap = [Modifier; Property::COUNT];
pub type EffectsMap = [f64; Property::COUNT];


pub fn create_modifier_map(modifiers: &Vec<(Property, Modifier)>) -> ModifierMap {
    let mut modifier_map = ModifierMap::default();
    for (property, modifiers) in modifiers.iter() {
        modifier_map[property.to_owned() as usize] = modifiers.to_owned();
    }

    modifier_map
}


#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;
    use super::*;

    #[test]
    fn test_create_modifier_map() {
        let mut expected = ModifierMap::default();

        expected[0] = Modifier {modifier: 3.5, multiplier: 1.0};
        expected[3] = Modifier {modifier: 1.5, multiplier: 0.2};
        expected[4] = Modifier {modifier: 0.9, multiplier: 0.1};
        

        let props: Vec<Property> = Property::iter().collect();
        let source_vec = vec![
            (props[0], expected[0].clone()),
            (props[3], expected[3].clone()),
            (props[4], expected[4].clone())
        ];
        let actual = create_modifier_map(&source_vec);
        assert!(actual.iter().zip(expected.iter()).all(|(a, b)| a==b));
    }
}