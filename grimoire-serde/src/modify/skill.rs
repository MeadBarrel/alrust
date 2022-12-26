use serde::{Serialize, Deserialize};

use grimoire2::modify::skill::SkillUpdate;

use crate::theoretical::TheoreticalWrapper;


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SkillUpdateSerializable {
    effectiveness: Option<TheoreticalWrapper>,
    parent: Option<String>,
    parent_2: Option<String>,
    remove_parent: bool,
    remove_parent_2: bool,
}


impl SkillUpdateSerializable {
    pub fn to_update(&self) -> SkillUpdate {
        let mut update = SkillUpdate::default();

        if let Some(x) = self.effectiveness {
            update.set_effectiveness(x.into());
        }

        if let Some(x) = &self.parent {
            update.set_parent(x);
        }

        if let Some(x) = &self.parent_2 {
            update.set_parent2(x);
        }

        if self.remove_parent {
            update.remove_parent();
        }

        if self.remove_parent_2 {
            update.remove_parent_2();
        }

        update
    }

}

impl From<SkillUpdateSerializable> for SkillUpdate {
    fn from(value: SkillUpdateSerializable) -> Self {
        value.to_update()
    }
}