use crate::theoretical::Theoretical;

#[derive(Debug, Clone)]
pub struct Skill {
    pub effectiveness: Theoretical<f64>,
    pub parent: Option<String>,
    pub parent_2: Option<String>,
}

impl Skill {
    pub fn new(
        effectiveness: Theoretical<f64>,
        parent: Option<String>,
        parent_2: Option<String>,
    ) -> Self {
        Self {
            effectiveness,
            parent,
            parent_2,
        }
    }
}

impl Default for Skill {
    fn default() -> Self {
        Self {
            effectiveness: Theoretical::Theory(0.66666),
            parent: None,
            parent_2: None,
        }
    }
}


pub mod versioned {
    use serde::{Serialize, Deserialize};

    use crate::theoretical::versioned::TheoreticalVersioned;

    use super::Skill;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum SkillVersioned {
        #[serde(rename="0")]
        V0(SkillV0)
    }

    impl From<Skill> for SkillVersioned {
        fn from(value: Skill) -> Self {
            SkillVersioned::V0(value.into())
        }
    }

    impl From<SkillVersioned> for Skill {
        fn from(value: SkillVersioned) -> Self {
            match value {
                SkillVersioned::V0(x) => x.into()
            }
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SkillV0 {
        pub effectiveness: TheoreticalVersioned<f64>,
        pub parent: Option<String>,
        pub parent_2: Option<String>,       
    }

    impl From<Skill> for SkillV0 {
        fn from(value: Skill) -> Self {
            Self {
                effectiveness: value.effectiveness.into(),
                parent: value.parent,
                parent_2: value.parent_2,
            }
        }
    }

    impl From<SkillV0> for Skill {
        fn from(value: SkillV0) -> Self {
            Self {
                effectiveness: value.effectiveness.into(),
                parent: value.parent,
                parent_2: value.parent_2,
            }            
        }
    }
}