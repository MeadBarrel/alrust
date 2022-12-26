use crate::id::PrefixedId;
use super::editor::CharacterEditor;
use egui::{Ui, Window};
use grimoire2::prelude::{Grimoire, Character};

#[derive(Debug, Default)]
pub struct CharacterEditorWindows(Vec<CharacterEditorWindow>);

#[derive(Debug, Default)]
pub struct CharacterEditorWindow {
    name: String,
    editor: CharacterEditor,
    id: PrefixedId,
}

impl CharacterEditorWindows {
    /// Show the windows
    pub fn show(&mut self, ui: &mut Ui, grimoire: &mut Grimoire) {
        // Close all window that return `false` on calling `show`
        self.0.retain_mut(|window| {
            window.show(ui, grimoire)
        });
    }

    /// Add a new window
    pub fn add(&mut self, name: &str) {
        self.0.push(
            CharacterEditorWindow::new(name)
        )
    }
}

impl CharacterEditorWindow {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }

    pub fn show(&mut self, ui: &mut Ui, grimoire: &mut Grimoire) -> bool {
        // Cloning the character in advance to please the borrow checker
        let maybe_character = grimoire.characters.get(&self.name).cloned();
        match maybe_character {
            Some(character) => self.show_window(ui, grimoire, character),            
            None => false  // Just close the window if character is not found at that name
        }
    }

    pub fn show_window(&mut self, ui: &mut Ui, grimoire: &mut Grimoire, mut character: Character) -> bool {
        let mut open = true;

        // Create new window
        let window = Window::new(self.name.as_str())
            .id(self.id.id())
            .collapsible(true)
            .open(&mut open);

        // ...and show it
        window.show(ui.ctx(), |ui| {
            // Replace the character at that name with the one returned by the editor
            character = self.editor.show(ui, grimoire, character);
            grimoire.characters.insert(self.name.clone(), character);
        });

        open

    }
}
