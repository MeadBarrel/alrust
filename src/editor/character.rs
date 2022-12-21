use eframe::egui::Ui;
use grimoire2::grimoire::Character;
use crate::id::PrefixedId;


#[derive(Debug, Default)]
pub struct CharacterEditor {
    id: PrefixedId,
}


impl CharacterEditor {
    pub fn show(&mut self, ui: &mut Ui, character: &mut Character) {
        
    }
}