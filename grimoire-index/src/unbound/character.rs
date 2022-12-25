use grimoire2::grimoire::Grimoire;
use grimoire2::prelude::Character;
use crate::unbound::UnboundCharacterBacklink;

use super::{UnboundIndex, UnboundGrimoireBacklink};
use super::grimoire::UnboundGrimoire;

#[derive(Clone)]
pub struct UnboundCharacter(pub UnboundGrimoire, pub String);

#[derive(Clone)]
pub struct UnboundCharacterSkill(pub UnboundCharacter, pub String);

impl UnboundIndex for UnboundCharacter {
    type Item = Character;

    fn get<'a>(&self, source: &'a grimoire2::prelude::Grimoire) -> Option<&'a Self::Item> {
        source.characters.get(&self.1)
    }

    fn get_mut<'a>(&self, source: &'a mut grimoire2::prelude::Grimoire) -> Option<&'a mut Self::Item> {
        source.characters.get_mut(&self.1)
    }
}

impl UnboundGrimoireBacklink for UnboundCharacter {
    type Backlink = UnboundGrimoire;

    fn grimoire(&self) -> &Self::Backlink {
        &self.0
    }
}

impl UnboundCharacter {
    pub fn skill(&self, name: impl Into<String>) -> UnboundCharacterSkill {
        UnboundCharacterSkill(self.clone(), name.into())
    }

    pub fn skills<'a>(&self, source: &'a Grimoire) -> Option<impl Iterator<Item = UnboundCharacterSkill> + 'a> {
        let s = self.clone();
        Some(self.get(source)?.skills.keys().map(move |x| UnboundCharacterSkill(s.clone(), x.clone())))
    }
}

impl UnboundIndex for UnboundCharacterSkill {
    type Item = u8;

    fn get<'a>(&self, source: &'a Grimoire) -> Option<&'a Self::Item> {
        self.0.get(source)?.skills.get(&self.1)
    }

    fn get_mut<'a>(&self, source: &'a mut Grimoire) -> Option<&'a mut Self::Item> {
        self.0.get_mut(source)?.skills.get_mut(&self.1)
    }
}

impl UnboundCharacterBacklink for UnboundCharacterSkill {
    type Backlink = UnboundCharacter;

    fn character(&self) -> &Self::Backlink {
        &self.0
    }
}