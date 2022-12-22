mod clades;
mod clade;
mod create_clade_button;

use eframe::egui::Ui;
use grimoire2::grimoire::Character;
use crate::id::PrefixedId;
use super::editor::ItemEditor;


use clades::CharacterCladesEditor;


#[derive(Debug, Default)]
pub struct CharacterEditor {
    id: PrefixedId,
    clades_editor: CharacterCladesEditor,
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
        });
    }
}