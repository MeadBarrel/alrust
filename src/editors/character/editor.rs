use crate::id::PrefixedId;
use grimoire2::grimoire::{Grimoire, Character};
use egui::{Ui};

#[derive(Debug, Default)]
pub struct CharacterEditor {
    id: PrefixedId,
}


impl CharacterEditor {
    pub fn show(&mut self, ui: &mut Ui, grimoire: &mut Grimoire, character: Character) -> Character {
        character
    }
}