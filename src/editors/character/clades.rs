use egui::Ui;
use grimoire2::grimoire::Character;

use crate::id::PrefixedId;
use crate::widgets::AddButton;
use super::clade::CharacterClade;

#[derive(Debug, Default)]
pub struct CharacterCladesEditor {
    id: PrefixedId,
    add_button: AddButton,
}

impl CharacterCladesEditor {
    pub fn show(&mut self, ui: &mut Ui, character: &mut Character) {
        let clades_cloned = character.clades.clone();
        ui.horizontal_wrapped(|ui| {
            for clade in clades_cloned.iter() {
                CharacterClade::new(clade).show(ui, character);
            }
            self.add_button.show(ui, |_, name| {
                character.clades.insert(name.to_string());
            });
        });
    }
}