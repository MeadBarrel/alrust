use grimoire2::grimoire::Skill;
use crate::grimoire::{GrimoireBackLink, IndexElement};
use super::grimoire::IndexGrimoireStruct;

pub struct IndexSkill<'a, 'b: 'a> {
    grimoire: &'a mut IndexGrimoireStruct<'b>,
    name: String,
}

impl<'a, 'b> IndexSkill<'a, 'b> {
    pub fn new(grimoire: &'a mut IndexGrimoireStruct<'b>, name: impl Into<String>) -> Self {
        Self { grimoire, name: name.into() }
    }

    pub fn parent(&mut self) -> Option<IndexSkill<'_, 'b>> {
        Some(
            IndexSkill::new(
                self.grimoire, self.get()?.parent.clone()?
            )
        )
    }

    pub fn parent_2(&mut self) -> Option<IndexSkill<'_, 'b>> {
        Some(
            IndexSkill::new(
                self.grimoire, self.get()?.parent_2.clone()?
            )
        )
    }
}

impl<'a, 'b> IndexElement for IndexSkill<'a, 'b> {
    type Item = Skill;

    fn get_mut(&mut self) -> Option<&mut Self::Item> {
        self.grimoire.skills_mut().get_mut(&self.name)
    }

    fn get(&self) -> Option<&Self::Item> {
        self.grimoire.skills().get(&self.name)
    }
}

impl<'a, 'b> GrimoireBackLink for IndexSkill<'a, 'b> {
    type Item = IndexGrimoireStruct<'b>;

    fn grimoire_mut(&mut self) -> &mut Self::Item {
        self.grimoire
    }

    fn grimoire(&self) -> &Self::Item {
        &*self.grimoire
    }
}