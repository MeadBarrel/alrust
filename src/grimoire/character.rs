use std::ops::Index;
use grimoire2::grimoire::Character;
use super::grimoire::IndexGrimoireStruct;
use super::characterskill::IndexCharacterSkill;
use super::*;


pub struct IndexCharacter<'a, 'b: 'a> {
    grimoire: &'a mut IndexGrimoireStruct<'b>,
    name: String,
}

impl<'a, 'b> IndexElement for IndexCharacter<'a, 'b> {
    type Item = Character;

    fn get_mut(&mut self) -> Option<&mut Character> {
        self.grimoire.characters_mut().get_mut(&self.name)
    }

    fn get(&self) -> Option<&Character> {
        self.grimoire.characters().get(&self.name)
    }
}

impl<'a, 'b> GrimoireBackLink for IndexCharacter<'a, 'b> {
    type Item = IndexGrimoireStruct<'b>;

    fn grimoire_mut(&mut self) -> &mut Self::Item {
        self.grimoire
    }

    fn grimoire(&self) -> &Self::Item {
        &*self.grimoire
    }
}

impl<'a, 'b> IndexCharacter<'a, 'b> {
    pub fn new(grimoire: &'a mut IndexGrimoireStruct<'b>, name: impl Into<String>) -> Self {
        Self {
            grimoire, name: name.into()
        }
    }

    pub fn skill(&mut self, name: impl Into<String>) -> IndexCharacterSkill<'_, 'a, 'b> {
        IndexCharacterSkill::new(self, name)
    } 
}
