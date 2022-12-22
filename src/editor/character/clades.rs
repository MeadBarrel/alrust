use std::collections::HashSet;
use egui::{Key, TextEdit, Ui, Widget};
use grimoire2::grimoire::Character;
use crate::id::PrefixedId;
use super::clade::CharacterClade;
use super::create_clade_button::CreateCladeButton;


#[derive(Debug, Default)]
pub struct CharacterCladesEditor {
    id: PrefixedId,
    create_clade_button: CreateCladeButton,
}


impl CharacterCladesEditor {
    pub fn new(id: PrefixedId) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }

    pub fn show(&mut self, ui: &mut Ui, clades: &mut HashSet<String>) {
        ui.horizontal_wrapped(|ui| {
            clades.clone().iter().for_each(|clade| {
                CharacterClade::new(self.id.derive_suffix(clade.as_str()))
                    .show(ui, clades, clade);
            });
            self.create_clade_button.show(ui, clades);
        });

    }
}