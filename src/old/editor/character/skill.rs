use std::collections::HashMap;
use egui::Ui;
use grimoire2::prelude::Skills;
use crate::id::PrefixedId;

pub struct CharacterSkill {
    id: PrefixedId,
}


impl CharacterSkill {
    pub fn new(id: PrefixedId) -> Self {
        Self {
            id
        }
    }

    pub fn show(&mut self, ui: &mut Ui, skills: &Skills, skill: &String) {
        ui.label(skill);
    }
}