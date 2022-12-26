use super::{UnboundIndex, GrimoireBacklink};
use grimoire2::grimoire::{Grimoire, Ingredient};
use crate::grimoire::GrimoireIndex;

pub struct IngredientIndex(pub GrimoireIndex, pub String);

impl UnboundIndex for IngredientIndex {
    type Item = Ingredient;

    fn get<'a>(&self, source: &'a Grimoire) -> Option<&'a Self::Item> {
        source.ingredients.get(&self.1)
    }

    fn get_mut<'a>(&self, source: &'a mut Grimoire) -> Option<&'a mut Self::Item> {
        source.ingredients.get_mut(&self.1)
    }
}

impl GrimoireBacklink for IngredientIndex {
    type Backlink = GrimoireIndex;

    fn grimoire(&self) -> &Self::Backlink {
        &self.0
    }
}
