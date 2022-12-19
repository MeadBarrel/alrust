use serde::{Serialize, Deserialize};
use crate::{theoretical::Theoretical, prelude::Skill};


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

    pub fn from_skill(skill: &Skill) -> Self {
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

    pub fn update(&self, skill: &mut Skill) {
        for command in &self.commands {
            match command {
                SkillUpdateCommand::SetEffectiveness(x) => skill.effectiveness = *x,
                SkillUpdateCommand::SetParent(x) => skill.parent = x.clone(),
                SkillUpdateCommand::SetParent2(x) => skill.parent_2 = x.clone(),
            }
        };
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


impl From<Skill> for SkillUpdate {
    fn from(value: Skill) -> Self {
        Self::from_skill(&value)
    }
}


impl From<SkillUpdate> for Skill {
    fn from(value: SkillUpdate) -> Self {
        value.create()
    }
}


#[cfg(test)]
mod tests {
    use crate::{grimoire::Skill, prelude::{Known, Theory}};

    use super::SkillUpdate;

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
}