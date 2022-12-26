mod table;
mod create;
mod edit;

use std::cell::Cell;
use eframe::egui::Ui;
use grimoire2::grimoire::Characters;
use crate::id::PrefixedId;
use eframe::egui;
use indexmap::IndexMap;
use grimoire2::prelude::Character;
use crate::grimoire::CellGrimoire;
use super::character;


#[derive(Debug, Default)]
pub struct CharactersEditor {
    id: PrefixedId,
    create_windows: create::CreateCharacterWindows,
    edit_windows: edit::EditCharacterWindows,
    table: table::CharactersTable,
}


impl CharactersEditor {
    pub fn show(&mut self, ui: &mut Ui, grimoire: &CellGrimoire, characters: IndexMap<String, Cell<Character>>) {
        egui::TopBottomPanel::top(self.id.derive_suffix("top")).show_inside(ui, |ui| {
            self.top_panel(ui);
        });
        egui::CentralPanel::default().show_inside(ui, |ui| {
            self.central_panel(ui, grimoire.characters_mut());
        });

        self.create_windows.show(ui, grimoire.characters_mut());
        self.edit_windows.show(ui, grimoire.characters_mut());
    }

    pub fn tf(&mut self, character: &Character) {}

    fn top_panel(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            self.top_panel_right(ui);
        });
    }

    fn top_panel_right(&mut self, ui: &mut Ui) {
        let right_to_left = egui::Layout::right_to_left(egui::Align::Center);
        ui.with_layout(right_to_left, |ui| {
            self.add_character_button(ui);
        });
    }

    fn central_panel(&mut self, ui: &mut Ui, characters: &mut Characters) {
        //self.characters_table(ui, characters);
        self.table.show(ui, characters)
    }

    fn add_character_button(&mut self, ui: &mut Ui) {
        if ui.button("Add Character").clicked() {
            self.create_windows.add()
        };
    }


}

