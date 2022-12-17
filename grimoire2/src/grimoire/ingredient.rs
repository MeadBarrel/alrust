use crate::modifiermap::ModifierMap;

#[derive(Default, Debug, Clone)]
pub struct Ingredient {
    pub skill: Option<String>,
    pub weight: bool,
    pub modifiers: ModifierMap,
}

impl Ingredient {
    pub fn new(skill: &str, weight: bool, modifiers: ModifierMap) -> Self {
        Self {
            skill: Some(skill.to_string()),
            weight,
            modifiers,
        }
    }
}
