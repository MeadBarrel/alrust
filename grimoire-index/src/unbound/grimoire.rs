use grimoire2::prelude::{Grimoire, Characters, Skills, Ingredients};
use crate::unbound::ingredient::UnboundIngredient;
use crate::unbound::skill::UnboundSkill;

use super::UnboundIndex;
use super::character::UnboundCharacter;

#[derive(Clone, Copy)]
pub struct UnboundGrimoire();

impl UnboundIndex for UnboundGrimoire {
    type Item = Grimoire;

    fn get<'a>(&self, source: &'a Grimoire) -> Option<&'a Self::Item> {
        Some(source)
    }

    fn get_mut<'a>(&self, source: &'a mut Grimoire) -> Option<&'a mut Self::Item> {
        Some(source)
    }
}

impl UnboundGrimoire {
    pub fn character(&self, name: impl Into<String>) -> UnboundCharacter {
        UnboundCharacter(*self, name.into())
    }

    pub fn characters<'a>(&self, source: &'a Grimoire) -> impl Iterator<Item = UnboundCharacter> + 'a {
        let s = *self;
        source.characters.keys().map(move |x| UnboundCharacter(s, x.clone()))
    }

    pub fn skill(&self, name: impl Into<String>) -> UnboundSkill {
        UnboundSkill(*self, name.into())
    }

    pub fn skills<'a>(&self, source: &'a Grimoire) -> impl Iterator<Item = UnboundSkill> + 'a {
        let s = *self;
        source.skills.keys().map(move |x| UnboundSkill(s, x.clone()))
    }

    pub fn ingredient(&self, name: impl Into<String>) -> UnboundIngredient {
        UnboundIngredient(*self, name.into())
    }

    pub fn ingredients<'a>(&self, source: &'a Grimoire) -> impl Iterator<Item = UnboundIngredient> + 'a {
        let s = *self;
        source.ingredients.keys().map(move |x| UnboundIngredient(s, x.clone()))
    }
}
