use std::collections::{HashSet, HashMap};
use serde::{Serialize};
use grimoire2::grimoire::Character;

#[derive(Serialize)]
pub struct CharacterHumanReadable {
    skills: HashMap<String, u8>,
    clades: HashSet<String>
}

impl From<Character> for CharacterHumanReadable {
    fn from(value: Character) -> Self {
        Self {
            skills: value.skills,
            clades: value.clades,
        }
    }
}
