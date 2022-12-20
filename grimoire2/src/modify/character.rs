use std::ops::Index;

use serde::{Serialize, Deserialize};

use crate::{grimoire::Character};

use super::command::Commands;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ModifyClade {
    Add(String),
    Remove(String),
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CharacterUpdateCommand {
    AddClade(String),
    RemoveClade(String),
    SetSkill(String, u8)
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CharacterUpdate {
    commands: Vec<CharacterUpdateCommand>,
}

impl CharacterUpdate {
    pub fn create(&self) -> Character {
        let mut character = Character::default();
        self.update(&mut character);
        character
    }

    pub fn add_clade(&mut self, clade: &str) -> &mut Self {
        self.commands.push(CharacterUpdateCommand::AddClade(clade.to_string()));
        self
    }

    pub fn remove_clade(&mut self, clade: &str) -> &mut Self {
        self.commands.push(CharacterUpdateCommand::RemoveClade(clade.to_string()));
        self
    }

    pub fn set_skill(&mut self, skill: &str, value: u8) -> &mut Self {
        self.commands.push(CharacterUpdateCommand::SetSkill(skill.to_string(), value));
        self
    }

    pub fn remove_skill(&mut self, skill: &str) -> &mut Self {
        self.commands.push(CharacterUpdateCommand::SetSkill(skill.to_string(), 0));
        self
    }
}


impl Index<usize> for CharacterUpdate {
    type Output = CharacterUpdateCommand;

    fn index(&self, index: usize) -> &Self::Output {
        &self.commands[index]
    }
}


impl Commands<Character, CharacterUpdateCommand> for CharacterUpdate {
    fn create_from(character: &Character) -> CharacterUpdate {
        let mut update = Self::default();

        character.clades.iter().for_each(|clade| { update.add_clade(clade); });
        character.skills.iter().for_each(|(skill, value)| { update.set_skill(skill, *value); } );

        update
    }

    fn update(&self, character: &mut Character) {
        self.commands.iter().for_each(|command|
            match command {
                CharacterUpdateCommand::AddClade(clade) => { character.clades.insert(clade.clone()); },
                CharacterUpdateCommand::RemoveClade(clade) => { character.clades.remove(clade.as_str()); },
                CharacterUpdateCommand::SetSkill(skill, value) => match value {
                    0 => { character.skills.remove(skill.as_str()); },
                    x => { 
                        let v = character.skills.entry(skill.clone()).or_default();
                        *v = *x;
                    }
                }
            }
        );
    }

    fn add(&mut self, command: CharacterUpdateCommand) -> &mut Self {
        self.commands.push(command);
        self
    }

    fn len(&self) -> usize {
        self.commands.len()
    }

    fn combine_last(&mut self) -> &mut Self {
        use CharacterUpdateCommand::*;

        if self.len() < 2 { return  self; }

        let prev = &self.commands[self.len()-2];
        let last = &self.commands[self.len()-1];

        match (prev, last) {
            (AddClade(a), RemoveClade(b)) => if a == b { self._replace_last_two_with(last.clone()); },
            (RemoveClade(a), AddClade(b)) => if a == b { self._replace_last_two_with(last.clone()); },
            (SetSkill(a, _), SetSkill(b, _)) => if a == b { self._replace_last_two_with(last.clone()); },
            (_, _) => {},
        }

        self
    }

    fn truncate(&mut self, index: usize) -> &mut Self {
        self.commands.truncate(index);
        self
    }
}


impl From<Character> for CharacterUpdate {
    fn from(value: Character) -> Self {
        Self::create_from(&value)
    }
}


impl From<CharacterUpdate> for Character {
    fn from(value: CharacterUpdate) -> Self {
        value.create()
    }
}


pub mod versioned {
    use serde::{Serialize, Deserialize};

    use super::CharacterUpdate;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum CharacterUpdateVersioned {
        #[serde(rename="0")]
        V0(CharacterUpdate)
    }

    impl From<CharacterUpdate> for CharacterUpdateVersioned {
        fn from(value: CharacterUpdate) -> Self {
            Self::V0(value)
        }
    }

    impl From<CharacterUpdateVersioned> for CharacterUpdate {
        fn from(value: CharacterUpdateVersioned) -> Self {
            match value {
                CharacterUpdateVersioned::V0(x) => x
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::{grimoire::Character, prelude::character};
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
        
        let update = CharacterUpdate::create_from(&character);

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
            .set_skill("a", 10)
            .set_skill("c", 80)
            .clone();
        
        update.update(&mut character);

        assert!( !character.has_clade("a") );
        assert!( !character.has_clade("b") );
        assert!( character.has_clade("d"));
        

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

    #[test]
    fn test_combine_last_add_remove_clade() {
        let update = CharacterUpdate::default()
            .add_clade("a")
            .remove_clade("a")
            .combine_last()
            .clone();
        let character = &mut CharacterUpdate::default().add_clade("a").create();
        update.update(character);
        assert_eq!(update.len(), 1);
        assert!(!character.has_clade("a"));
    }

    #[test]
    fn test_combine_last_remove_add_clade() {
        let update = CharacterUpdate::default()
            .remove_clade("a")
            .add_clade("a")
            .combine_last()
            .clone();
        let character = &mut Character::default();
        update.update(character);
        assert_eq!(update.len(), 1);
        assert!(character.has_clade("a"));
    }    

    #[test]
    fn test_combine_last_add_remove_clade_different() {
        let update = CharacterUpdate::default()
            .add_clade("b")
            .remove_clade("a")
            .combine_last()
            .clone();
        let character = &mut CharacterUpdate::default().add_clade("a").create();
        update.update(character);
        assert_eq!(update.len(), 2);
        assert!(!character.has_clade("a"));
        assert!(character.has_clade("b"));
    }

    #[test]
    fn test_combine_last_remove_add_clade_different() {
        let update = CharacterUpdate::default()
            .remove_clade("a")
            .add_clade("b")
            .combine_last()
            .clone();
        let character = &mut CharacterUpdate::default().add_clade("a").create();
        update.update(character);
        assert_eq!(update.len(), 2);
        assert!(!character.has_clade("a"));
        assert!(character.has_clade("b"));
    }    

    #[test]
    fn test_combine_last_set_skill() {
        let update = CharacterUpdate::default()
            .set_skill("a", 50)
            .remove_skill("a")
            .combine_last()
            .clone();
        let character = &mut CharacterUpdate::default().set_skill("a", 100).create();
        update.update(character);
        assert_eq!(update.len(), 1);
        assert_eq!(character.raw_skill("a"), 0);
    }

    #[test]
    fn test_combine_last_set_skill_diff() {
        let update = CharacterUpdate::default()
            .set_skill("b", 50)
            .remove_skill("a")
            .combine_last()
            .clone();
        let character = &mut CharacterUpdate::default().set_skill("a", 100).create();
        update.update(character);
        assert_eq!(update.len(), 2);
        assert_eq!(character.raw_skill("a"), 0);
        assert_eq!(character.raw_skill("b"), 50);
    }    
}