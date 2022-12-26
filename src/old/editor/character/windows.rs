use egui::Window;
use egui::Ui;
use grimoire2::prelude::Grimoire;
use grimoire_index::prelude::CharacterIndex;
use crate::id::PrefixedId;
use super::CharacterEditor;

pub struct CharacterEditWindow(CharacterEditor, PrefixedId);

pub struct CharacterEditWindows {
    windows: Vec<CharacterEditWindow>
}

impl CharacterEditWindows {
    pub fn show(&mut self, ui: &mut Ui, grimoire: &mut Grimoire) {
        let ctx = ui.ctx();
        self.windows.retain_mut(|window| {
            let mut open = true;
            Window::new(window.0.name())
                .id(window.1.id())
                .open(&mut open)
                .collapsible(true)
                .show(ctx, |ui| { window.0.show(ui, grimoire) });
            open
        })
    }

    pub fn add(&mut self, index: CharacterIndex) {
        self.windows.push(
            CharacterEditWindow(
                CharacterEditor::new(index),
                PrefixedId::default(),
            )
        )
    }
}