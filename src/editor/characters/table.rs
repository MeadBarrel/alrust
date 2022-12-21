use eframe::egui::Ui;
use egui_extras::{TableBuilder, TableRow, TableBody, Column};
use grimoire2::prelude::{Characters, Character};
use crate::editor::Tab;

use super::edit::EditCharacterWindows;


#[derive(Debug)]
pub struct CharactersTable {
    edit_windows: EditCharacterWindows,
    header_height: f32,
    row_height: f32,
}


impl Default for CharactersTable {
    fn default() -> Self {
        Self {
            edit_windows: EditCharacterWindows::default(),
            header_height: 25.,
            row_height: 25.,
        }
    }
}


impl CharactersTable {
    pub fn show(&mut self, ui: &mut Ui, characters: &mut Characters) {
        self.edit_windows.show(ui, characters);

        let table = TableBuilder::new(ui)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .striped(true)
            .column(Column::initial(100.).range(40.0..=300.)).resizable(true)
            .column(Column::remainder());
        table
            .header(self.header_height, |mut header| {
                header.col(|ui| {
                    ui.strong("Name"); 
                });
                header.col(|ui| {
                    ui.strong("Skills total");                    
                });
            })
            .body(|mut body| {
                for (name, character) in characters.clone().iter() {
                    body.row(self.row_height, |ui| self.row(ui, characters, name, character))
                }
            });

    }

    fn row(&mut self, mut ui: TableRow, characters: &mut Characters, name: &str, character: &Character) {
        ui.col(|ui| {
            self.character_label(ui, characters, name)
        });
        ui.col(|ui| {
            ui.label(
                character.skills.values()
                    .map(|x| *x as u64)
                    .sum::<u64>()
                    .to_string()
            );
        });    
    }

    fn character_label(&mut self, ui: &mut Ui, characters: &mut Characters, name: &str) {
        let label = egui::Label::new(name)
        .sense(egui::Sense::click());
        ui.add(label)
            .on_hover_cursor(egui::CursorIcon::ContextMenu)
            .context_menu(|ui| {
                self.context_menu(ui, name, characters)
            });
    }

    fn context_menu(&mut self, ui: &mut Ui, name: &str, characters: &mut Characters) {
        if ui.button("Edit").clicked() {
            self.edit_windows.add(name);
            ui.close_menu();
        };
        if ui.button("Delete").clicked() {
            characters.remove(name);
            ui.close_menu();
        };
    }
}
