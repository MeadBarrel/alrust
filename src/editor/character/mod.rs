mod clades;
mod clade;
mod create_clade_button;
mod skills;
mod skill;

use std::borrow::BorrowMut;
use eframe::egui::Ui;
use indexmap::IndexMap;
use grimoire2::grimoire::Character;
use crate::id::PrefixedId;
use super::editor::ItemEditor;


use clades::CharacterCladesEditor;
use grimoire2::prelude::Grimoire;
use crate::editor::character::skills::CharacterSkillsEditor;


#[derive(Debug, Default)]
pub struct CharacterEditor {
    id: PrefixedId,
    clades_editor: CharacterCladesEditor,
    skills_editor: CharacterSkillsEditor,
}


impl CharacterEditor {
}


impl ItemEditor for CharacterEditor {
    type Item = Character;

    fn show(&mut self, ui: &mut Ui, item: &mut Character) {
        ui.vertical(|ui| {
            ui.heading("Clades");
            ui.group(|ui| {
                self.clades_editor.show(ui, &mut item.clades);
            });
            ui.separator();
            ui.heading("Skills");
            ui.group(|ui| {
                self.skills_editor.show(ui, &mut item.skills)
            })
        });
    }
}