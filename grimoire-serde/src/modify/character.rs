use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use grimoire2::modify::character::CharacterUpdate;


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CharacterUpdateSerializable {
    remove_clades: Vec<String>,
    add_clades: Vec<String>,

    remove_skills: Vec<String>,    
    skills: HashMap<String, u8>,
}


impl CharacterUpdateSerializable {
    pub fn to_update(&self) -> CharacterUpdate {
        let mut update = CharacterUpdate::default();
        self.remove_clades.iter().for_each(|clade| { update.remove_clade(clade); });
        self.add_clades.iter().for_each(|clade| { update.add_clade(clade); } );
        
        self.remove_skills.iter().for_each(|skill| { update.remove_skill(skill); } );
        self.skills.iter().for_each(|(skill, value)| { update.set_skill(skill, *value); } );

        update
    }

    pub fn from_update(update: &CharacterUpdate) -> Self {
        Self {
            remove_clades: update.clades_remove.iter().cloned().collect(),
            add_clades: update.clades_add.to_vec(),
            remove_skills: update.skills_remove.iter().cloned().collect(),
            skills: update.skills.iter().cloned().collect(),
        }
    }
}


impl From<CharacterUpdate> for CharacterUpdateSerializable {
    fn from(value: CharacterUpdate) -> Self {
        CharacterUpdateSerializable::from_update(&value)
    }
}


impl From<CharacterUpdateSerializable> for CharacterUpdate {
    fn from(value: CharacterUpdateSerializable) -> Self {
        value.to_update()
    }
}



#[cfg(test)]
mod tests {
    use grimoire2::modify::character::CharacterUpdate;

    use super::CharacterUpdateSerializable;

    #[test]
    fn test_to_update() {
        let mut character = CharacterUpdate::default()
            .add_clade("a")
            .add_clade("b")
            .set_skill("a", 80)
            .set_skill("b", 100)
            .create();

        let ser_update = CharacterUpdateSerializable {
            add_clades: vec! [ "d".to_string(), "e".to_string() ].into_iter().collect(),
            remove_clades: vec! [ "a".to_string() ].into_iter().collect(),
            remove_skills: vec! [ "a".to_string() ].into_iter().collect(),
            skills: vec! [ ("c".to_string(), 50) ].into_iter().collect(),
        };

        let update = ser_update.to_update();

        update.update(&mut character);

        assert!(character.clades.contains("d"));
        assert!(character.clades.contains("e"));
        assert!(character.clades.contains("b"));
        assert!(!character.clades.contains("a"));
        assert!(!character.skills.contains_key("a"));
        assert!(character.skills.contains_key("b"));
        assert!(character.skills.contains_key("c"));
        assert_eq!(*character.skills.get("c").unwrap(), 50);
    }

    #[test]
    fn test_from_update() {
        let update = CharacterUpdate::default()
            .add_clade("a")
            .add_clade("b")
            .remove_clade("c")
            .remove_skill("a")
            .set_skill("b", 50)
            .clone();
        
        let ser_update = CharacterUpdateSerializable::from_update(&update);

        assert!(ser_update.add_clades.contains(&"a".to_string()));
        assert!(ser_update.add_clades.contains(&"b".to_string()));
        assert!(ser_update.remove_clades.contains(&"c".to_string()));
        assert!(ser_update.skills.contains_key("b"));
        assert_eq!(ser_update.skills["b"], 50);
    }
}