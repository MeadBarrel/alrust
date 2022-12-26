use grimoire2::grimoire::Grimoire;
use grimoire2::prelude::Character;
use crate::CharacterBacklink;

use super::{UnboundIndex, GrimoireBacklink};
use super::grimoire::GrimoireIndex;

#[derive(Clone)]
pub struct CharacterIndex(pub GrimoireIndex, pub String);

#[derive(Clone)]
pub struct CharacterSkillIndex(pub CharacterIndex, pub String);

impl UnboundIndex for CharacterIndex {
    type Item = Character;

    fn get<'a>(&self, source: &'a grimoire2::prelude::Grimoire) -> Option<&'a Self::Item> {
        source.characters.get(&self.1)
    }

    fn get_mut<'a>(&self, source: &'a mut grimoire2::prelude::Grimoire) -> Option<&'a mut Self::Item> {
        source.characters.get_mut(&self.1)
    }
}

impl GrimoireBacklink for CharacterIndex {
    type Backlink = GrimoireIndex;

    fn grimoire(&self) -> &Self::Backlink {
        &self.0
    }
}

impl CharacterIndex {
    pub fn skill(&self, name: impl Into<String>) -> CharacterSkillIndex {
        CharacterSkillIndex(self.clone(), name.into())
    }

    pub fn skills<'a>(&self, source: &'a Grimoire) -> Option<impl Iterator<Item =CharacterSkillIndex> + 'a> {
        let s = self.clone();
        Some(self.get(source)?.skills.keys().map(move |x| CharacterSkillIndex(s.clone(), x.clone())))
    }

    pub fn name(&self) -> &str {
        &self.1
    }
}

impl UnboundIndex for CharacterSkillIndex {
    type Item = u8;

    fn get<'a>(&self, source: &'a Grimoire) -> Option<&'a Self::Item> {
        self.0.get(source)?.skills.get(&self.1)
    }

    fn get_mut<'a>(&self, source: &'a mut Grimoire) -> Option<&'a mut Self::Item> {
        self.0.get_mut(source)?.skills.get_mut(&self.1)
    }
}

impl CharacterBacklink for CharacterSkillIndex {
    type Backlink = CharacterIndex;

    fn character(&self) -> &Self::Backlink {
        &self.0
    }
}