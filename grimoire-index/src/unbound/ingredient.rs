use super::{UnboundIndex, UnboundGrimoireBacklink};
use grimoire2::grimoire::{Grimoire, Ingredient};
use crate::unbound::grimoire::UnboundGrimoire;

pub struct UnboundIngredient(pub UnboundGrimoire, pub String);

impl UnboundIndex for UnboundIngredient {
    type Item = Ingredient;

    fn get<'a>(&self, source: &'a Grimoire) -> Option<&'a Self::Item> {
        source.ingredients.get(&self.1)
    }

    fn get_mut<'a>(&self, source: &'a mut Grimoire) -> Option<&'a mut Self::Item> {
        source.ingredients.get_mut(&self.1)
    }
}

impl UnboundGrimoireBacklink for UnboundIngredient {
    type Backlink = UnboundGrimoire;

    fn grimoire(&self) -> &Self::Backlink {
        &self.0
    }
}
