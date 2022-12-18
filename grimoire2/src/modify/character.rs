use std::collections::{HashSet, HashMap};

use crate::{grimoire::Character};


#[derive(Clone, Debug)]
pub enum ModifyClade {
    Add(String),
    Remove(String),
}


#[derive(Default, Clone, Debug)]
pub struct CharacterUpdate {
    pub clades_add: Vec<String>,
    pub clades_remove: HashSet<String>,
    pub skills: Vec<(String, u8)>,
    pub skills_remove: HashSet<String>,
}

impl CharacterUpdate {
    pub fn create(&self) -> Character {
        let mut character = Character::default();
        self.update(&mut character);
        character
    }

    pub fn from_character(character: &Character) -> CharacterUpdate {
        let mut update = Self::default();

        character.clades.iter().for_each(|clade| { update.add_clade(clade); });
        character.skills.iter().for_each(|(skill, value)| { update.set_skill(skill, *value); } );

        update
    }

    pub fn update(&self, character: &mut Character) {
        character.clades.retain(|x| !self.clades_remove.contains(x));
        character.skills.retain(|x, _| !self.skills_remove.contains(x));
        character.clades.extend(self.clades_add.iter().cloned());
        character.skills.extend(self.skills.iter().cloned())
    }

    pub fn add_clade(&mut self, clade: &str) -> &mut Self {
        self.clades_add.push(clade.to_string());
        self
    }

    pub fn remove_clade(&mut self, clade: &str) -> &mut Self {
        self.clades_remove.insert(clade.to_string());
        self
    }

    pub fn set_skill(&mut self, skill: &str, value: u8) -> &mut Self {
        self.skills.push((skill.to_string(), value));
        self
    }

    pub fn remove_skill(&mut self, skill: &str) -> &mut Self {
        self.skills_remove.insert(skill.to_string());
        self
    }
}


impl From<Character> for CharacterUpdate {
    fn from(value: Character) -> Self {
        Self::from_character(&value)
    }
}


impl From<CharacterUpdate> for Character {
    fn from(value: CharacterUpdate) -> Self {
        value.create()
    }
}


#[cfg(test)]
mod tests {
    use crate::grimoire::Character;
    use super::*;
    use maplit::{hashmap, hashset};

    #[test]
    fn test_from_character() {
        let character = CharacterUpdate::default()
            .add_clade("a")
            .add_clade("b")
            .set_skill("a", 100)
            .set_skill("b", 5)
            .create();
        
        let update = CharacterUpdate::from_character(&character);

        let new_character = update.create();

        assert!(new_character.has_clade("a"));
        assert!(new_character.has_clade("b"));
        assert_eq!(new_character.raw_skill("a"), 100);
        assert_eq!(new_character.raw_skill("b"), 5);
    }

    #[test]
    fn test_modify_character() {
        let mut character = character();
        
        let update = CharacterUpdate::default()
            .add_clade("a")
            .add_clade("d")
            .remove_clade("a")
            .remove_clade("b")
            .remove_clade("d")
            .set_skill("a", 10)
            .set_skill("c", 80)
            .clone();
        
        update.update(&mut character);

        assert!( character.has_clade("a") );
        assert!( character.has_clade("d"));
        assert!( !character.has_clade("b") );

        assert_eq!( character.raw_skill("a"), 10 );
        assert_eq!( character.raw_skill("c"), 80 )
    }

    fn character() -> Character {
        let clades = hashset! ["a".to_string(), "b".to_string(), "c".to_string()];
        let skills = hashmap! {
            "a".to_string() => 100,
            "b".to_string() => 50,
            "c".to_string() => 0,
        };

        Character::new(clades, skills)
    }
}