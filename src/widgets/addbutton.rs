use egui::{Ui, TextEdit, Widget, Key};

use crate::id::PrefixedId;


#[derive(Debug)]
pub struct AddButton {
    value: Option<String>,
    width: f32,
    button_text: String,
    id: PrefixedId,
    trim: bool,
    allow_empty: bool,
}

impl AddButton {
    pub fn show(&mut self, ui: &mut Ui, func: impl FnOnce(&mut Ui, &str)) {
        match &mut self.value {
            Some(value) => {
                let id = self.id.id();
                let editor = TextEdit::singleline(value)
                    .desired_width(self.width)
                    .hint_text("Enter to confirm")
                    .id(id);
                ui.ctx().memory().request_focus(id);    
                if editor.ui(ui).lost_focus() {
                    if ui.ctx().input().key_down(Key::Enter) {
                        let mut final_result = value.clone();
                        if self.trim {
                            final_result = final_result.trim().into();
                        }
                        if !self.allow_empty && final_result.is_empty() {
                            return
                        }
                        func(ui, &final_result);
                    }
                    self.value = None;
                }
                if ui.ctx().input().key_down(Key::Escape) {
                    self.value = None;
                }                
            }
            None => {
                if ui.button(&self.button_text).clicked() {
                    self.value = Some("".into());
                }
            }
        }
    }

    pub fn width(mut self, width: f32) -> Self {        
        self.width = width;
        self
    }

    pub fn id(mut self, id: PrefixedId) -> Self {
        self.id = id;
        self
    }

    pub fn button_text(mut self, text: impl Into<String>) -> Self {
        self.button_text = text.into();
        self
    }
    
    pub fn allow_empty(mut self, allow: bool) -> Self {
        self.allow_empty = allow;
        self
    }

    pub fn trim(mut self, trim: bool) -> Self {
        self.trim = trim;
        self
    }
}

impl Default for AddButton {
    fn default() -> Self {
        Self {
            id: PrefixedId::default(),
            value: None,
            width: 100.,
            button_text: "+".to_string(),
            allow_empty: false,
            trim: true,
        }
    }
}