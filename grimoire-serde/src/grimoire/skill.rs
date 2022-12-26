use serde::Serialize;
use grimoire2::grimoire::Skill;

use crate::theoretical::TheoreticalWrapper;

#[derive(Serialize)]
pub struct SkillHumanReadable {
    #[serde(skip_serializing_if = "Option::is_none")]
    parent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_2: Option<String>,
    #[serde(skip_serializing_if = "TheoreticalWrapper::is_unknown")]
    effectiveness: TheoreticalWrapper,
}

impl From<Skill> for SkillHumanReadable {
    fn from(value: Skill) -> Self {
        Self {
            parent: value.parent,
            parent_2: value.parent_2,
            effectiveness: value.effectiveness.into()
        }
    }
}
