use eframe::egui::Ui;
use super::character::CharacterEditor;
use crate::id::PrefixedId;
use grimoire2::prelude::{Character, Characters};
use crate::widget::CloseableWindow;
use crate::types::AugmentedWindowResponse;
use crate::error::{Report, Error, handle_error};


#[derive(Debug, Default)]
pub struct EditCharacterWindows {
    windows: Vec<CharacterEditWindow>
}

impl EditCharacterWindows {
    pub fn show(&mut self, ui: &mut Ui, characters: &mut Characters) {
        self.windows.retain_mut(|window| {
            let character = characters.get_mut(&window.character_name);
            match character {
                Some(x) => window.show(ui, x).augment,
                None => {
                    handle_error(
                        Report::new(Error::Generic(
                            "Character deleted, but character edition window was open, closing".to_string()
                        ))
                    );
                    false
                }
            }
            
        })
    }

    pub fn add(&mut self, name: impl Into<String>) {
        self.windows.push(
            CharacterEditWindow::new(name, CharacterEditor::default())
        )
    }
}


#[derive(Debug)]
struct CharacterEditWindow {
    pub character_name: String,
    pub editor: CharacterEditor,
    pub id: PrefixedId,
    pub default_pos: Option<egui::Pos2>,
}



impl CharacterEditWindow {
    pub fn new(character_name: impl Into<String>, editor: CharacterEditor) -> Self {
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