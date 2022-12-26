use crate::id::PrefixedId;
use grimoire2::grimoire::Character;
use egui::Ui;

#[derive(Debug)]
pub struct CharacterClade {
    clade: String,
}

impl CharacterClade {
    pub fn new(clade: impl Into<String>) -> Self {
        Self {
            clade: clade.into(),
        }
    }

    pub fn show(&mut self, ui: &mut Ui, character: &mut Character) {
        let label = egui::Label::new(&self.clade).sense(egui::Sense::click());

        ui.add(label)
            .on_hover_cursor(egui::CursorIcon::ContextMenu)
            .context_menu(|ui| {
                if ui.button("Remove").clicked() {
                    character.clades.remove(&self.clade);
                    ui.close_menu();
                }
            });        
    }
}