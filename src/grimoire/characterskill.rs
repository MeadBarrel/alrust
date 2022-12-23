use crate::grimoire::{CharacterBackLink, IndexElement};
use super::character::IndexCharacter;

pub struct IndexCharacterSkill<'a, 'b: 'a, 'c: 'b> {
    character: &'a mut IndexCharacter<'b, 'c>,
    name: String,
}

impl<'a, 'b, 'c> IndexElement for IndexCharacterSkill<'a, 'b, 'c> {
    type Item = u8;

    fn get_mut(&mut self) -> Option<&mut u8> {
        self.character.get_mut().and_then(|x| x.skills.get_mut(&self.name))
    }

    fn get(&self) -> Option<&u8> {
        self.character.get().and_then(|x| x.skills.get(&self.name))
    }
}

impl<'a, 'b, 'c> CharacterBackLink for IndexCharacterSkill<'a, 'b, 'c> {
    type Item = IndexCharacter<'b, 'c>;

    fn character_mut(&mut self) -> &mut Self::Item {
        self.character
    }

    fn character(&self) -> &Self::Item {
        &*self.character
    }
}

impl<'a, 'b, 'c> IndexCharacterSkill<'a, 'b, 'c> {
    pub fn new(character: &'a mut IndexCharacter<'b, 'c>, name: impl Into<String>) -> Self {
        Self { character, name: name.into() }
    }
}