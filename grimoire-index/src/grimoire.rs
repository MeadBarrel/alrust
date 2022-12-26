use grimoire2::prelude::{Grimoire, Characters, Skills, Ingredients};
use crate::ingredient::IngredientIndex;
use crate::skill::SkillIndex;

use super::UnboundIndex;
use super::character::CharacterIndex;

#[derive(Clone, Copy)]
pub struct GrimoireIndex();

impl UnboundIndex for GrimoireIndex {
    type Item = Grimoire;

    fn get<'a>(&self, source: &'a Grimoire) -> Option<&'a Self::Item> {
        Some(source)
    }

    fn get_mut<'a>(&self, source: &'a mut Grimoire) -> Option<&'a mut Self::Item> {
        Some(source)
    }
}

impl GrimoireIndex {
    pub fn character(&self, name: impl Into<String>) -> CharacterIndex {
        CharacterIndex(*self, name.into())
    }

    pub fn characters<'a>(&self, source: &'a Grimoire) -> impl Iterator<Item =CharacterIndex> + 'a {
        let s = *self;
        source.characters.keys().map(move |x| CharacterIndex(s, x.clone()))
    }

    pub fn skill(&self, name: impl Into<String>) -> SkillIndex {
        SkillIndex(*self, name.into())
    }

    pub fn skills<'a>(&self, source: &'a Grimoire) -> impl Iterator<Item =SkillIndex> + 'a {
        let s = *self;
        source.skills.keys().map(move |x| SkillIndex(s, x.clone()))
    }

    pub fn ingredient(&self, name: impl Into<String>) -> IngredientIndex {
        IngredientIndex(*self, name.into())
    }

    pub fn ingredients<'a>(&self, source: &'a Grimoire) -> impl Iterator<Item =IngredientIndex> + 'a {
        let s = *self;
        source.ingredients.keys().map(move |x| IngredientIndex(s, x.clone()))
    }
}
