use std::collections::HashMap;
use egui::Ui;
use grimoire2::grimoire::Grimoire;
use grimoire_index::character::CharacterIndex;
use crate::id::PrefixedId;

#[derive(Debug, Default)]
pub struct CharacterSkillsEditor {
    id: PrefixedId,
    index: CharacterIndex,
}

struct Branch(String, Vec<Branch>);


impl CharacterSkillsEditor {
    pub fn new(index: CharacterIndex) -> Self {
        Self {
            id: PrefixedId::default(),
            index,
        }
    }

    pub fn show(&mut self, ui: &mut Ui, grimoire: &mut Grimoire) {
        ui.label("This will be the skills window");
    }

    // pub fn show_branch(&mut self, ui: &mut Ui, skills: &mut HashMap<String, u8>, branch: &Branch) {
    //     ui.label(&branch.0);
    //     ui.group(|ui| {
    //         branch.1.iter().for_each(|child| {
    //             self.show_branch(ui, skills, child);
    //         })
    //     });
    // }
}