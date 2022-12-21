use crate::theoretical::Theoretical;

#[derive(Debug, Clone, PartialEq)]
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


#[cfg(test)]
pub mod tests {
    use proptest::strategy::Strategy;
    use proptest::sample::select;
    use crate::theoretical::tests::theoretical_f64_strategy;
    use super::*;

    pub fn skill_strategy() -> impl Strategy<Value = Skill> {
        let effectiveness = theoretical_f64_strategy();
        let parent = select(vec![
            Some("a"),
            Some("b"),
            None
        ]);
        let parent_2 = select(vec![
            Some("a"),
            Some("b"),
            None
        ]);        

        (effectiveness, parent, parent_2).prop_map(|(e, p, p2)| {
            Skill { 
                effectiveness: e, 
                parent: p.map(|x| x.to_string()), 
                parent_2: p2.map(|x| x.to_string()), 
            }
        })
    }    
}