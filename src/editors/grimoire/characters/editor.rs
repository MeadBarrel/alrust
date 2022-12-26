use egui::{Ui, Label, Sense, WidgetText};
use grimoire2::grimoire::{Grimoire, Character};
use crate::widgets::{AddButton, Rename};
use crate::global::show_info;
use crate::editors::character::window::CharacterEditorWindows;
use crate::error::{handle_error, Report, Error};

#[derive(Debug, Default)]
pub struct CharactersEditor {
    /// A collection of character editor windows
    editor_windows: CharacterEditorWindows,
    /// A button for adding new characters
    add_button: AddButton,
    renaming: Option<Rename>,
}

impl CharactersEditor {
    /// Displays the character editor   
    pub fn show(&mut self, ui: &mut Ui, grimoire: &mut Grimoire) {
        let names: Vec<String> = grimoire.characters.keys().cloned().collect();
        for name in names.iter() {
            if !Rename::handle_option(ui, &mut self.renaming, &mut grimoire.characters, name) {
                self.character_label(ui, grimoire, name);
            }          
        };

        self.add_button.show(ui, |ui, value| {

            // Show info message if a character with that name already exists
            if grimoire.characters.contains_key(value) {
                show_info(ui, "Already exists", "Character already exists");
                return
            }

            // Otherwise add the character
            grimoire.characters.insert(value.into(), Character::default());
        });

        // Show our character editor windows
        self.editor_windows.show(ui, grimoire);

    }

    fn character_label(&mut self, ui: &mut Ui, grimoire: &mut Grimoire, name: &str) {        
        let text = egui::RichText::new(name).size(32.);
        // Adding sense is required for on_hover_cursor
        let label = Label::new(WidgetText::RichText(text)).sense(Sense::click());

        ui.add(label)
        // Make sure the label reacts visually on hover
        .on_hover_cursor(egui::CursorIcon::ContextMenu) 
        // Add context menu with Edit, Delete, and Rename buttons
        .context_menu(|ui| {

            ui.vertical(|ui| {

                if ui.button("Edit").clicked() {
                    // Add a new character editor window
                    self.editor_windows.add(name);
                    ui.close_menu();
                };

                if ui.button("Rename").clicked() {
                    self.renaming = Some(Rename::new(name));
                    ui.close_menu();
                }

                if ui.button("Delete").clicked() {
                    // Delete the character
                    grimoire.characters.remove(name);
                    ui.close_menu();
                };  

            });     

        });        
    }
}