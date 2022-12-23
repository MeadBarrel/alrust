use grimoire2::prelude::Ingredient;
use crate::grimoire::{GrimoireBackLink, IndexElement};
use super::grimoire::IndexGrimoireStruct;
use super::skill::IndexSkill;

pub struct IndexIngredient<'a, 'b: 'a> {
    grimoire: &'a mut IndexGrimoireStruct<'b>,
    name: String
}

impl<'a, 'b> IndexIngredient<'a, 'b> {
    pub fn new(grimoire: &'a mut IndexGrimoireStruct<'b>, name: impl Into<String>) -> Self {
        Self { grimoire, name: name.into() }
    }

    pub fn skill(&mut self) -> Option<IndexSkill<'_, 'b>> {
        Some(
            IndexSkill::new(self.grimoire, self.get()?.skill.clone()?)
        )
    }
}

impl<'a, 'b> IndexElement for IndexIngredient<'a, 'b> {
    type Item = Ingredient;

    fn get_mut(&mut self) -> Option<&mut Self::Item> {
        self.grimoire.ingredients_mut().get_mut(&self.name)
    }

    fn get(&self) -> Option<&Self::Item> {
        self.grimoire.ingredients().get(&self.name)
    }
}

impl<'a, 'b> GrimoireBackLink for IndexIngredient<'a, 'b> {
    type Item = IndexGrimoireStruct<'b>;

    fn grimoire_mut(&mut self) -> &mut Self::Item {
        self.grimoire
    }

    fn grimoire(&self) -> &Self::Item {
        &*self.grimoire
    }
}
