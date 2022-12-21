use eframe::egui;
use egui::Ui;
use crate::id::PrefixedId;
use tracing::*;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OkCancel {
    Ok,
    Cancel,
    None,
}


#[derive(Debug)]
pub struct OkCancelWindow {
    id: PrefixedId,
    title: String,
    ok_button: String,
    cancel_button: String,
    close_button: bool,
    resizable: bool,
}


impl Default for OkCancelWindow {
    fn default() -> Self {
        Self {
            id: PrefixedId::default(),
            title: "Yes or nah?".into(),
            ok_button: "Ok".into(),
            cancel_button: "Cancel".into(),
            close_button: true,
            resizable: false,
        }
    }
}


impl OkCancelWindow {
    pub fn id(mut self, id: PrefixedId) -> Self {
        self.id = id;
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn ok_button(mut self, name: impl Into<String>) -> Self {
        self.ok_button = name.into();
        self
    }

    pub fn cancel_button(mut self, name: impl Into<String>) -> Self {
        self.cancel_button = name.into();
        self
    }

    pub fn close_button(mut self, enabled: bool) -> Self {
        self.close_button = enabled;
        self
    }

    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    pub fn show(&mut self, ui: &mut egui::Ui, func: impl FnOnce(&mut egui::Ui) -> OkCancel ) -> OkCancel {
        let mut close_not_clicked = true;
        let mut result = OkCancel::None;

        let mut window = egui::Window::new(&self.title).id(self.id.id())
            .resizable(self.resizable);

        if self.close_button {
            window = window.open(&mut close_not_clicked);
        }

        window.show(ui.ctx(), |ui| {            
            ui.vertical(|ui| {
                let func_result = func(ui);
                ui.horizontal(|ui| {
                    if ui.button(&self.ok_button).clicked() {
                        result = OkCancel::Ok;
                    }
                    if ui.button(&self.cancel_button).clicked() {
                        result = OkCancel::Cancel;
                    }                    
                });

                if result == OkCancel::None {
                    result = func_result;
                }
            })
        });
        
        if !close_not_clicked { return OkCancel::Cancel };

        result
    }
}