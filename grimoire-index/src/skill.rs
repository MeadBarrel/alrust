use super::{UnboundIndex, GrimoireBacklink, grimoire::GrimoireIndex};

use grimoire2::grimoire::{Grimoire, Skill};

pub struct SkillIndex(pub GrimoireIndex, pub String);

impl UnboundIndex for SkillIndex {
    type Item = Skill;

    fn get<'a>(&self, source: &'a Grimoire) -> Option<&'a Self::Item> {
        source.skills.get(&self.1)
    }

    fn get_mut<'a>(&self, source: &'a mut Grimoire) -> Option<&'a mut Self::Item> {
        source.skills.get_mut(&self.1)
    }
}

impl GrimoireBacklink for SkillIndex {
    type Backlink = GrimoireIndex;

    fn grimoire(&self) -> &Self::Backlink {
        &self.0
    }
}