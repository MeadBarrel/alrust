use egui::{Ui, TextEdit, Widget};

use crate::id::PrefixedId;

#[derive(Debug)]
pub enum RenameResult {
    Done(String),
    Cancelled,
    None
}

#[derive(Debug)]
pub struct Rename {
    id: PrefixedId,
    pub original: String,
    value: String,
    width: f32,
    trim: bool,
    last_value_valid: bool,
}

impl Rename {
    pub fn new(original: impl Into<String>) -> Self {
        let original = original.into();
        let value = original.clone();
        Self {
            id: PrefixedId::default(),
            original,
            value,
            width: 100.,
            trim: true,
            last_value_valid: true,
        }
    }

    pub fn show(&mut self, ui: &mut Ui, validate: impl FnOnce(&str) -> bool) -> RenameResult {
        let id = self.id.id();        

        // Prepare the editor
        let mut editor = TextEdit::singleline(&mut self.value)
            .id(id)
            .desired_width(self.width);
        
        // Change text color to red is value is invalid
        if !self.last_value_valid {
            editor = editor.text_color(egui::Color32::from_rgb_additive(255, 0, 0));
        };

        // Force focus on the editor
        ui.ctx().memory().request_focus(id);


        let response = editor.ui(ui);

        // Process the final value
        let final_value = &self.modify_value(&self.value);

        self.last_value_valid = validate(final_value);

        if response.lost_focus() {
            // If enter is hit and value is valid, inform the caller that renaming is done
            // If value is invalid, do nothing
            if ui.ctx().input().key_down(egui::Key::Enter) {
                if self.last_value_valid {
                    return RenameResult::Done(final_value.clone());
                }
                return RenameResult::None;
            };
            // We lost focus so renaming cancels
            return RenameResult::Cancelled;
        };

        // Cancel renaming on ESC
        if ui.ctx().input().key_down(egui::Key::Escape) {
            return RenameResult::Cancelled;
        };

        RenameResult::None
    }

    fn modify_value(&self, value: &str) -> String {
        let mut result = value.to_string();
        if self.trim {
            result = result.trim().to_string();
        }
        result
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub fn trim(mut self, trim: bool) -> Self {
        self.trim = trim;
        self
    }
}