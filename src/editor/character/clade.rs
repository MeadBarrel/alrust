use std::collections::HashSet;
use egui::Ui;
use grimoire2::prelude::Character;
use crate::id::PrefixedId;


#[derive(Debug)]
pub struct CharacterClade {
    id: PrefixedId
}


impl CharacterClade {
    pub fn new(id: PrefixedId) -> Self {
        Self {
            id
        }
    }

    pub fn show(&mut self, ui: &mut Ui, clades: &mut HashSet<String>, clade: &String) {
        ui.label(clade);
    }
}