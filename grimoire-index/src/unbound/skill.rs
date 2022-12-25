use super::{UnboundIndex, UnboundGrimoireBacklink, grimoire::UnboundGrimoire};

use grimoire2::grimoire::{Grimoire, Skill};

pub struct UnboundSkill(pub UnboundGrimoire, pub String);

impl UnboundIndex for UnboundSkill {
    type Item = Skill;

    fn get<'a>(&self, source: &'a Grimoire) -> Option<&'a Self::Item> {
        source.skills.get(&self.1)
    }

    fn get_mut<'a>(&self, source: &'a mut Grimoire) -> Option<&'a mut Self::Item> {
        source.skills.get_mut(&self.1)
    }
}

impl UnboundGrimoireBacklink for UnboundSkill {
    type Backlink = UnboundGrimoire;

    fn grimoire(&self) -> &Self::Backlink {
        &self.0
    }
}