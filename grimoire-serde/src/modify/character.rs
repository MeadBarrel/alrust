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

}

impl From<CharacterUpdateSerializable> for CharacterUpdate {
    fn from(value: CharacterUpdateSerializable) -> Self {
        value.to_update()
    }
}
