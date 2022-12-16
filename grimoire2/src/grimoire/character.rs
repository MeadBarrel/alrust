use std::collections::HashMap;
use std::cmp::min;

use super::Skills;
use crate::theoretical::Theoretical;


#[derive(Default, Debug, Clone)]
pub struct Character {
    pub skills: HashMap<String, u8>
}


impl Character {
    pub fn new(skills: HashMap<String, u8>) -> Self {
        Self { skills }
    }

    pub fn lore_multiplier(&self, skills: &Skills, skill: &str) -> Theoretical<f64> {
        let effectiveness = skills.get(skill).cloned().unwrap_or_default().effectiveness;
        let value = self.skill(skills, skill);

        Theoretical::from(1.) + effectiveness * Theoretical::from(value as f64 /100.)
    }

    pub fn skill(&self, skills: &Skills, skill: &str) -> u8 {
        let this_skill_value = self.skills.get(skill).cloned().unwrap_or_default();
        let this_skill = skills.get(skill).cloned().unwrap_or_default();
        
        let parent_1_value = match this_skill.parent {
            Some(x) => self.skill(skills, &x),
            None => 100
        };

        let parent_2_value = match this_skill.parent_2 {
            Some(x) => self.skill(skills, &x),
            None => 100
        };

        let min_parent = min(parent_1_value, parent_2_value);

        min(this_skill_value, min_parent)
    }
}


#[cfg(test)]
pub mod tests {
    use crate::grimoire::Skill;
    use super::*;

    use float_cmp::approx_eq;

    #[test]
    fn test_skill_no_parents() {
        let expected = 80;

        let skills = vec![
            ("Skill".to_string(), Skill::new(Theoretical::Known(2.33333), None, None))
        ].into_iter().collect();

        let character = Character::new(
            vec![
                ("Skill".to_string(), expected), 
                ("Parent Skill".to_string(), 50)
            ].into_iter().collect()
        );

        let actual = character.skill(&skills, "Skill");

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_skill_has_parent() {
        let expected = 50;

        let skills = vec![
            (
                "Skill".to_string(), 
                Skill::new(Theoretical::Known(2.33333), Some("Parent Skill".to_string()), None)
            )
        ].into_iter().collect();

        let character = Character::new(
            vec![
                ("Skill".to_string(), 80), 
                ("Parent Skill".to_string(), expected)
            ].into_iter().collect()
        );

        let actual = character.skill(&skills, "Skill");

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_lore_multiplier() {
        let skills = vec![
            ("Parent Skill".to_string(), Skill::new(Theoretical::from(0.33333), None, None)),
            ("Skill".to_string(), 
                Skill::new(Theoretical::Known(2.33333), Some("Parent Skill".to_string()), None))
        ].into_iter().collect();

        let character = Character::new(
            vec![
                ("Skill".to_string(), 100), 
                ("Parent Skill".to_string(), 100)
            ].into_iter().collect()
        );
        

        let expected = 3.33333;
        let actual = character.lore_multiplier(&skills, "Skill");

        assert!(actual.is_known(), "{:?}", actual);
        assert!( approx_eq!(f64, actual.inner(), expected, epsilon=0.01), "{:?}", actual )
    }

    #[test]
    fn test_lore_multiplier_unknown() {
        let skills = vec![
            ("Parent Skill".to_string(), Skill::new(Theoretical::from(0.33333), None, None)),
            ("Skill".to_string(), 
                Skill::new(Theoretical::Unknown(2.33333), Some("Parent Skill".to_string()), None))
        ].into_iter().collect();

        let character = Character::new(
            vec![
                ("Skill".to_string(), 100), 
                ("Parent Skill".to_string(), 100)
            ].into_iter().collect()
        );
        

        let expected = 3.33333;
        let actual = character.lore_multiplier(&skills, "Skill");

        assert!(!actual.is_known(), "{:?}", actual);
        assert!( approx_eq!(f64, actual.inner(), expected, epsilon=0.01), "{:?}", actual )
    }
}    
