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
        let label = egui::Label::new(clade).sense(egui::Sense::click());
        ui.add(label)
            .on_hover_cursor(egui::CursorIcon::ContextMenu)
            .context_menu(|ui| {
                if ui.button("Remove").clicked() {
                    clades.remove(clade);
                    ui.close_menu();
                }
            });
    }
}