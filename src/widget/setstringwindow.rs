use eframe::egui::Ui;
use super::{OkCancelWindow, OkCancel};
use crate::types::*;


#[derive(Default, Debug)]
pub struct SetStringWindow {
    value: String,
    ok_cancel_window: OkCancelWindow,
}


impl SetStringWindow {
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.ok_cancel_window = self.ok_cancel_window.title(title);
        self
    }

    pub fn ok_button(mut self, name: impl Into<String>) -> Self {
        self.ok_cancel_window = self.ok_cancel_window.ok_button(name);
        self
    }

    pub fn cancel_button(mut self, name: impl Into<String>) -> Self {
        self.ok_cancel_window = self.ok_cancel_window.cancel_button(name);
        self
    }

    pub fn close_button(mut self, enabled: bool) -> Self {
        self.ok_cancel_window = self.ok_cancel_window.close_button(enabled);
        self
    }

    pub fn resizable(mut self, resizable: bool) -> Self {
        self.ok_cancel_window = self.ok_cancel_window.resizable(resizable);
        self
    }

    pub fn show(&mut self, ui: &mut Ui) -> AugmentedWindowResponse<OkCancel, ()> {
        self.ok_cancel_window.show(ui, |ui| {
            ui.text_edit_singleline(&mut self.value);
            OkCancel::None
        })
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }
}