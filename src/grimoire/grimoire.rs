use indexmap::IndexMap;
use grimoire2::grimoire::{Grimoire, Ingredient, Skill};
use grimoire2::prelude::Character;
use crate::grimoire::IndexElement;
use super::character::IndexCharacter;

pub struct IndexGrimoireStruct<'a>
{
    grimoire: &'a mut Grimoire
}

impl<'a> IndexElement for IndexGrimoireStruct<'a> {
    type Item = Grimoire;

    fn get_mut(&mut self) -> Option<&mut Self::Item> {
        Some(self.grimoire)
    }

    fn get(&self) -> Option<&Self::Item> {
        Some(&*self.grimoire)
    }
}

impl<'a> IndexGrimoireStruct<'a> {
    pub fn new(grimoire: &'a mut Grimoire) -> Self {
        Self { grimoire }
    }

    pub fn characters_mut(&mut self) -> &mut IndexMap<String, Character> {
        &mut self.grimoire.characters
    }

    pub fn characters(&self) -> &IndexMap<String, Character> {
        &self.grimoire.characters
    }

    pub fn skills_mut(&mut self) -> &mut IndexMap<String, Skill> {
        &mut self.grimoire.skills
    }

    pub fn skills(&self) -> &IndexMap<String, Skill> {
        &self.grimoire.skills
    }

    pub fn ingredients_mut(&mut self) -> &mut IndexMap<String, Ingredient> {
        &mut self.grimoire.ingredients
    }

    pub fn ingredients(&self) -> &IndexMap<String, Ingredient> {
        &self.grimoire.ingredients
    }

    pub fn character(&mut self, name: impl Into<String>) -> IndexCharacter<'_, 'a> {
        IndexCharacter::new(self, name)
    }
}

