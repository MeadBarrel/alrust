use crate::id::PrefixedId;
use grimoire2::grimoire::{Grimoire, Character};
use egui::{Ui, Widget, CollapsingHeader};
use super::clades::CharacterCladesEditor;

#[derive(Debug, Default)]
pub struct CharacterEditor {
    id: PrefixedId,
    clades_editor: CharacterCladesEditor,
}


impl CharacterEditor {
    pub fn show(&mut self, ui: &mut Ui, grimoire: &mut Grimoire, mut character: Character) -> Character {
        let collapsing_header_clades = CollapsingHeader::new("Clades")
            .id_source(self.id.derive_suffix("clades_editor").id())
            .default_open(true);

        collapsing_header_clades.show(ui, |ui| {
            ui.group(|ui| {
                self.clades_editor.show(ui, &mut character)
            })
        });

        let collapsing_header_skills = CollapsingHeader::new("Skills")
            .id_source(self.id.derive_suffix("skills_editor").id())
            .default_open(true);        

        character
    }
}