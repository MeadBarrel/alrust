use std::collections::HashSet;
use egui::{Key, TextEdit, Ui, Widget};
use crate::id::PrefixedId;


#[derive(Debug, Default)]
pub struct CreateCladeButton {
    id: PrefixedId,
    value: Option<String>,
}


impl CreateCladeButton {
    pub fn show(&mut self, ui: &mut Ui, clades: &mut HashSet<String>) {
        match &mut self.value {
            Some(value) => {
                let editor_id = self.id.derive_suffix("new_clade_name_edit").id();
                let mut editor = TextEdit::singleline(value).desired_width(100.).id(editor_id);
                ui.ctx().memory().request_focus(editor_id);
                if editor.ui(ui).lost_focus() {
                    if ui.ctx().input().key_down(Key::Enter) {
                        clades.insert(value.clone());
                    }
                    self.value = None
                };
            }
            None=> {
                if ui.button("+").clicked() {
                    self.value = Some("".to_string());
                }
            }
        }
    }
}