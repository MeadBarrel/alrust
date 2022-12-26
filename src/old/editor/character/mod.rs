mod clades;
mod clade;
mod create_clade_button;
mod skills;
mod skill;
pub mod windows;


use eframe::egui::Ui;
use grimoire2::grimoire::Character;
use crate::id::PrefixedId;
use super::editor::ItemEditor;
use clades::CharacterCladesEditor;
use grimoire2::prelude::Grimoire;
use crate::editor::character::skills::CharacterSkillsEditor;

#[derive(Debug)]
pub struct CharacterEditor {
    id: PrefixedId,
    pub index: CharacterIndex,
    clades_editor: CharacterCladesEditor,
    skills_editor: CharacterSkillsEditor,
}


impl CharacterEditor {
    pub fn new(index: CharacterIndex) -> Self {
        Self {
            id: PrefixedId::default(),
            index,
            clades_editor: CharacterCladesEditor::new(index),
            skills_editor: CharacterSkillsEditor::new(index),
        }
    }

    pub fn name(&self) -> &str {
        self.index.name()
    }

    fn show(&mut self, ui: &mut Ui, grimoire: &mut Grimoire) {
        ui.vertical(|ui| {
            ui.heading("Clades");
            ui.group(|ui| {
                self.clades_editor.show(ui, grimoire);
            });
            ui.separator();
            ui.heading("Skills");
            ui.group(|ui| {
                self.skills_editor.show(ui, grimoire)
            })
        });
    }
}