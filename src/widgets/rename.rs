use egui::{Ui, TextEdit, Widget};
use indexmap::IndexMap;
use crate::error::{Error, Report, handle_error};

use crate::id::PrefixedId;

#[derive(Debug)]
pub struct Rename {
    id: PrefixedId,
    pub original: String,
    value: String,
    width: f32,
    trim: bool,
    allow_empty: bool,
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
            allow_empty: false,
            last_value_valid: true,
        }
    }

    pub fn handle_option<T>(
        ui: &mut Ui, 
        rename_opt: &mut Option<Rename>, 
        map: &mut IndexMap<String, T>, 
        current: &str
    ) -> bool {
        let taken = std::mem::take(rename_opt);
        match taken {
            Some(rename) if rename.original == current => {
                *rename_opt = rename.show(ui, map);
                true
            }
            _ => {
                *rename_opt = taken;
                false
            }
        }
    }

    pub fn show<T>(mut self, ui: &mut Ui, map: &mut IndexMap<String, T>) -> Option<Self> {
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

        self.last_value_valid = 
            (self.allow_empty || !final_value.is_empty()) && !map.contains_key(final_value.as_str());

        if response.lost_focus() {
            // If enter is hit and value is valid, inform the caller that renaming is done
            // If value is invalid, do nothing
            if ui.ctx().input().key_down(egui::Key::Enter) {
                if self.last_value_valid {
                    let old = map.remove(&self.original);
                    match old {
                        Some(item) => {
                            map.insert(final_value.clone(), item);
                            return None
                        }
                        None => {
                            handle_error(
                                Report::new(
                                    Error::Generic(
                                        "Could not rename item, the item that existed \
                                        under the old name no longer exists".to_string()
                                    )
                                )
                            )
                        }
                    }
                }
                return Some(self);
            };
            // We lost focus so renaming cancels
            return None;
        };

        // Cancel renaming on ESC
        if ui.ctx().input().key_down(egui::Key::Escape) {
            return None;
        };

        Some(self)
    }


    fn modify_value(&self, value: &str) -> String {
        let mut result = value.to_string();
        if self.trim {
            result = result.trim().to_string();
        }
        result
    }
}