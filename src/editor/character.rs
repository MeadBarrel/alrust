use eframe::egui::Ui;
use grimoire2::grimoire::Character;
use crate::id::PrefixedId;
use super::editor::ItemEditor;


#[derive(Debug, Default)]
pub struct CharacterEditor {
    id: PrefixedId,
}


impl CharacterEditor {
    pub fn show(&mut self, ui: &mut Ui, character: &mut Character) {
        
    }
}


impl ItemEditor for CharacterEditor {
    type Item = Character;

    fn show(&mut self, ui: &mut Ui, item: &mut Character) {
        
    }
}