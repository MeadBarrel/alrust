use eframe::egui::Ui;
use grimoire2::grimoire::{Characters, Character};
use tracing::debug;
use crate::{id::PrefixedId, error::handle_error};
use eframe::egui;
use crate::widget::{SetStringWindow, OkCancel};
use super::character;
use crate::error::{Report, Error};
use crate::types::*;


#[derive(Debug, Default)]
pub struct CharactersEditor {
    id: PrefixedId,
    create_windows: Vec<SetStringWindow>,
    edit_windows: Vec<CharacterEditWindow>,
}


impl CharactersEditor {
    pub fn show(&mut self, ui: &mut Ui, characters: &mut Characters) {
        egui::TopBottomPanel::top(self.id.derive_suffix("top")).show_inside(ui, |ui| {
            self.top_panel(ui);
        });
        egui::CentralPanel::default().show_inside(ui, |ui| {
            self.central_panel(ui, characters);
        });

        self.handle_create_windows(ui, characters);
        self.handle_edit_windows(ui, characters);
    }

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
        self.characters_table(ui, characters);
    }

    fn add_character_button(&mut self, ui: &mut Ui) {
        if ui.button("Add Character").clicked() {
            self.create_windows.push(
                SetStringWindow::default()
                    .title("Create new Character")
            )
        };
    }

    fn characters_table(&mut self, ui: &mut Ui, characters: &mut Characters) {
        use egui_extras::{Column, TableBuilder};

        let table = TableBuilder::new(ui)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .striped(true)
            .column(Column::initial(100.).range(40.0..=300.)).resizable(true)
            .column(Column::remainder());
        
        table
            .header(20.0, |mut header| {
                header.col(|ui| { 
                    ui.strong("Name"); 
                });
                header.col(|ui| {
                    ui.strong("Skills total");
                });
            })
            .body(|mut body| {
                for (name, character_) in characters.clone() {
                    body.row(32., |mut row| {
                        row.col(|ui| {
                            self.character_label(ui, characters, &name)
                        });
                        row.col(|ui| {
                            ui.label(
                                character_.skills.values()
                                    .map(|x| *x as u64)
                                    .sum::<u64>()
                                    .to_string()
                            );
                        });
                    });
                }
            })
            
            
    }

    fn character_label(&mut self, ui: &mut Ui, characters: &mut Characters, name: &String) {
        let label = egui::Label::new(name)
        .sense(egui::Sense::click());
        ui.add(label)
            .on_hover_cursor(egui::CursorIcon::ContextMenu)
            .context_menu(|ui| {
                if ui.button("Edit").clicked() {
                    self.open_character_editor(ui, name);
                    ui.close_menu();
                };
                if ui.button("Delete").clicked() {
                    characters.remove(name);
                    ui.close_menu();
                };
            });
    }

    fn open_character_editor(&mut self, ui: &mut Ui, name: &String) {
        self.edit_windows.push(
            CharacterEditWindow::new(name, character::CharacterEditor::default())
        )
    }

    fn handle_create_windows(&mut self, ui: &mut Ui, characters: &mut Characters) {
        self.create_windows.retain_mut(|window| {
            match window.show(ui).augment {
                OkCancel::Cancel => false,
                OkCancel::None => true,
                OkCancel::Ok => {
                    characters.insert(window.get_value().to_string(), Character::default());
                    false
                },
            }
        });
    }

    fn handle_edit_windows(&mut self, ui: &mut Ui, characters: &mut Characters) {
        self.edit_windows.retain_mut(|window| {
            let editing_character = characters.get_mut(&window.character_name);
            match editing_character {
                Some(x) => window.show(ui, x).augment,
                None => {
                    handle_error(
                        Report::new(Error::GenericError(
                            "Character deleted, but character edition window was open, closing".to_string()
                        ))
                    );
                    false
                }
            }
            
        })
    }
}



#[derive(Debug)]
struct CharacterEditWindow {
    pub character_name: String,
    pub editor: character::CharacterEditor,
    pub id: PrefixedId,
    pub default_pos: Option<egui::Pos2>,
}


impl CharacterEditWindow {
    pub fn new(character_name: impl Into<String>, editor: character::CharacterEditor) -> Self {
        Self {
            character_name: character_name.into(),
            editor,
            id: PrefixedId::default(),
            default_pos: None,
        }
    }

    pub fn default_pos(mut self, pos: impl Into<egui::Pos2>) -> Self {
        self.default_pos = Some(pos.into());
        self
    }

    pub fn show(&mut self, ui: &mut Ui, character: &mut Character) -> AugmentedWindowResponse<bool, ()> {
        use crate::widget::closeablewindow::CloseableWindow;

        let mut window = CloseableWindow::default()
            .id(self.id.derive_suffix("character_edit_window"))
            .auto_sized()
            .collabsible(true)
            .title(&self.character_name);
        
        if let Some(x) = self.default_pos {
            window = window.default_pos(x)
        }
        window
            .show(ui, |ui| {
                self.editor.show(ui, character);
                true
            })
    }
}