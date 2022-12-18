use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::theoretical::TheoreticalWrapper;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrimoireUpdateSerializable {
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CharacterUpdateSerializable {
    remove_clades: Vec<String>,
    add_clades: Vec<String>,

    remove_skills: Vec<String>,    
    skills: HashMap<String, u8>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SkillUpdateSerializable {
    effectiveness: TheoreticalWrapper,
    parent: Option<String>,
    parent_2: Option<String>,
    remove_parent: bool,
    remove_parent_2: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct IngredientUpdateSerializable {
    skill: Option<String>,
    remove_skill: bool,
    weight: bool,

    dh: TheoreticalWrapper,
    dp: TheoreticalWrapper,
    mdh: TheoreticalWrapper,
    mdp: TheoreticalWrapper,
    hot: TheoreticalWrapper,
    pot: TheoreticalWrapper,
    hl: TheoreticalWrapper,
    pl: TheoreticalWrapper,
    a: TheoreticalWrapper,
}

