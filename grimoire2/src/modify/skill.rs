use std::ops::Index;
use serde::{Serialize, Deserialize};
use crate::{theoretical::Theoretical, prelude::Skill};


use super::Commands;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SkillUpdateCommand {
    SetEffectiveness(Theoretical<f64>),
    SetParent(Option<String>),
    SetParent2(Option<String>)
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct SkillUpdate {
    commands: Vec<SkillUpdateCommand>
}


impl SkillUpdate {
    pub fn create(&self) -> Skill {
        let mut skill = Skill::default();
        self.update(&mut skill);
        skill
    }

    pub fn set_effectiveness(&mut self, value: Theoretical<f64>) -> &mut Self {
        self.commands.push(SkillUpdateCommand::SetEffectiveness(value));
        self
    }

    pub fn set_parent(&mut self, value: &str) -> &mut Self {
        self.commands.push(SkillUpdateCommand::SetParent(Some(value.to_string())));
        self
    }

    pub fn set_parent2(&mut self, value: &str) -> &mut Self {
        self.commands.push(SkillUpdateCommand::SetParent2(Some(value.to_string())));
        self
    }    

    pub fn remove_parent(&mut self) -> &mut Self {
        self.commands.push(SkillUpdateCommand::SetParent(None));
        self
    }

    pub fn remove_parent_2(&mut self) -> &mut Self {
        self.commands.push(SkillUpdateCommand::SetParent2(None));
        self
    }

}


impl Index<usize> for SkillUpdate {
    type Output = SkillUpdateCommand;

    fn index(&self, index: usize) -> &Self::Output {
        &self.commands[index]
    }
}


impl Commands<Skill, SkillUpdateCommand> for SkillUpdate {
    fn create_from(skill: &Skill) -> Self {
        let mut update = SkillUpdate::default();
        update.set_effectiveness(skill.effectiveness);

        match &skill.parent {
            Some(x) => update.set_parent(x),
            None => update.remove_parent(),
        };

        match &skill.parent_2 {
            Some(x) => update.set_parent2(x),
            None => update.remove_parent_2()
        };

        update
    }

    fn diff(c1: &Skill, c2: &Skill) -> Self {
        let mut result = Self::default();
        if c2.parent_2 != c1.parent_2 {
            result.commands.push(SkillUpdateCommand::SetParent2(c2.parent_2.clone()))
        };
        if c1.effectiveness != c2.effectiveness {
            result.set_effectiveness(c2.effectiveness);
        };
        if c2.parent != c1.parent {
            result.commands.push(SkillUpdateCommand::SetParent(c2.parent.clone()))
        }
        result
    }

    fn update(&self, skill: &mut Skill) {
        for command in &self.commands {
            match command {
                SkillUpdateCommand::SetEffectiveness(x) => skill.effectiveness = *x,
                SkillUpdateCommand::SetParent(x) => skill.parent = x.clone(),
                SkillUpdateCommand::SetParent2(x) => skill.parent_2 = x.clone(),
            }
        };
    }

    fn add(&mut self, command: SkillUpdateCommand) -> &mut Self {
        self.commands.push(command);
        self
    }

    fn len(&self) -> usize {
        self.commands.len()
    }

    fn combine_last(&mut self) -> &mut Self {
        use SkillUpdateCommand::*;

        if self.len() < 2 { return  self; }

        let prev = &self.commands[self.len()-2];
        let last = &self.commands[self.len()-1];

        match (prev, last) {
            (SetEffectiveness(_), SetEffectiveness(_)) =>
                self._replace_last_two_with(last.clone()),
            (SetParent(_), SetParent(_)) => self._replace_last_two_with(last.clone()),
            (SetParent2(_), SetParent2(_)) => self._replace_last_two_with(last.clone()),
            (_, _) => {}
        }

        self
    }

    fn truncate(&mut self, index: usize) -> &mut Self {
        self.commands.truncate(index);
        self
    }
    
    fn extend(&mut self, other: &Self) {
        self.commands.extend(other.commands.iter().cloned())
    }    
}


impl From<Skill> for SkillUpdate {
    fn from(value: Skill) -> Self {
        Self::create_from(&value)
    }
}


impl From<SkillUpdate> for Skill {
    fn from(value: SkillUpdate) -> Self {
        value.create()
    }
}


impl From<&Skill> for SkillUpdate {
    fn from(value: &Skill) -> Self {
        Self::create_from(value)
    }
}


pub mod versioned {
    use serde::{Serialize, Deserialize};

    use super::{SkillUpdate, SkillUpdateCommand};

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub enum SkillUpdateVersioned {
        #[serde(rename="0")]
        V0(v0::SkillUpdateV0)
    }

    impl From<SkillUpdate> for SkillUpdateVersioned {
        fn from(value: SkillUpdate) -> Self {
            Self::V0(value.into())
        }
    }

    impl From<SkillUpdateVersioned> for SkillUpdate {
        fn from(value: SkillUpdateVersioned) -> Self {
            match value {
                SkillUpdateVersioned::V0(x) => x.into()
            }
        }
    }

    pub mod v0 {
        use super::*;
        use crate::theoretical::versioned::TheoreticalVersioned;
        
        #[derive(Clone, Debug, Serialize, Deserialize)]
        pub struct SkillUpdateV0 {
            commands: Vec<SkillUpdateCommandV0>
        }

        impl From<SkillUpdate> for SkillUpdateV0 {
            fn from(value: SkillUpdate) -> Self {
                Self {
                    commands: value.commands.into_iter().map(|x| x.into()).collect(),
                }
            }
        }

        impl From<SkillUpdateV0> for SkillUpdate {
            fn from(value: SkillUpdateV0) -> Self {
                Self {
                    commands: value.commands.into_iter().map(|x| x.into()).collect(),
                }                
            }
        }

        #[derive(Clone, Debug, Serialize, Deserialize)]
        pub enum SkillUpdateCommandV0 {
            SetEffectiveness(TheoreticalVersioned<f64>),
            SetParent(Option<String>),
            SetParent2(Option<String>)
        }

        impl From<SkillUpdateCommand> for SkillUpdateCommandV0 {
            fn from(value: SkillUpdateCommand) -> Self {
                match value {
                    SkillUpdateCommand::SetEffectiveness(x) => 
                        SkillUpdateCommandV0::SetEffectiveness(x.into()),
                    SkillUpdateCommand::SetParent(x) => SkillUpdateCommandV0::SetParent(x),
                    SkillUpdateCommand::SetParent2(x) => SkillUpdateCommandV0::SetParent2(x),
                }
            }
        }

        impl From<SkillUpdateCommandV0> for SkillUpdateCommand {
            fn from(value: SkillUpdateCommandV0) -> Self {
                match value {
                    SkillUpdateCommandV0::SetEffectiveness(x) => 
                        SkillUpdateCommand::SetEffectiveness(x.into()),
                    SkillUpdateCommandV0::SetParent(x) => SkillUpdateCommand::SetParent(x),
                    SkillUpdateCommandV0::SetParent2(x) => SkillUpdateCommand::SetParent2(x),
                }                
            }
        }
    }
}


#[cfg(test)]
pub mod tests {
    use crate::{grimoire::Skill, prelude::{Known, Theory}};

    use super::SkillUpdate;
    use super::Commands;
    use proptest::prelude::*;
    use crate::grimoire::skill::tests::skill_strategy;

    proptest! {
        #[test]
        fn test_diff(s1 in skill_strategy(), s2 in skill_strategy()) {            
            let mut s1_ = s1.clone();
            let diff = SkillUpdate::diff(&s1, &s2);
            diff.update(&mut s1_);
            prop_assert_eq!(s1_, s2);
        }
    }    

    #[test]
    fn test_skill_update_set_effectiveness() {
        let mut skill = Skill::new(Known(0.9), None, None);
        let update = SkillUpdate::default().set_effectiveness(Theory(0.5)).clone();

        update.update(&mut skill);

        assert!( !skill.effectiveness.is_known() );
        assert_eq!( skill.effectiveness.inner(), 0.5 );
    }

    #[test]
    fn test_skill_update_set_parents() {
        let mut skill = Skill::new(Known(1.), None, None);
        let update = SkillUpdate::default()
            .set_parent("parent")
            .set_parent2("other parent")
            .clone();
        update.update(&mut skill);

        assert_eq!( skill.parent, Some("parent".to_string()) );
        assert_eq!( skill.parent_2, Some("other parent".to_string()) );
    }

    #[test]
    fn test_skill_remove_parents() {
        let mut skill = Skill::new(
            Known(1.0), Some("parent".to_string()), Some("parent_2".to_string()));
        let update = SkillUpdate::default()
            .remove_parent()
            .remove_parent_2()
            .clone();

        update.update(&mut skill);

        assert!( skill.parent.is_none() );
        assert!( skill.parent_2.is_none() );
    }

    #[test]
    fn test_skill_keep_intact() {
        let mut skill = Skill::new(
            Known(1.0),
            Some("parent".to_string()),
            None
        );
        let update = SkillUpdate::default();

        update.update(&mut skill);

        assert!( skill.effectiveness.is_known() );
        assert_eq!( skill.parent, Some("parent".to_string()) );
        assert!( skill.parent_2.is_none() );
    }

    #[test]
    fn test_update_last_set_effectiveness() {
        let update = SkillUpdate::default()
            .set_effectiveness(Known(1.0))
            .set_effectiveness(Known(2.0))
            .combine_last()
            .clone();
        let skill = &mut SkillUpdate::default().create();
        update.update(skill);
        assert_eq!(update.len(), 1);
        assert_eq!(skill.effectiveness, Known(2.0));
    }

    #[test]
    fn test_update_last_set_parent() {
        let update = SkillUpdate::default()
            .set_parent("a")
            .remove_parent()
            .combine_last()
            .clone();
        let skill = &mut SkillUpdate::default().set_parent("b").create();
        update.update(skill);
        assert_eq!(update.len(), 1);
        assert_eq!(skill.parent, None);
    }

    #[test]
    fn test_update_last_set_parent_2() {
        let update = SkillUpdate::default()
            .set_parent2("b")
            .remove_parent_2()
            .combine_last()
            .clone();
        let skill = &mut SkillUpdate::default().set_parent2("b").create();
        update.update(skill);
        assert_eq!(update.len(), 1);
        assert_eq!(skill.parent_2, None);
    }
}