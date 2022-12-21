use eframe::egui;
use egui::Ui;
use crate::id::PrefixedId;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OkCancel {
    Ok,
    Cancel,
    None,
}


#[derive(Debug)]
pub struct OkCancelWindow {
    id: PrefixedId,
    panel_id: egui::Id,
    title: String,
    ok_button: String,
    cancel_button: String,
    close_button: bool,
    resizable: bool,
}


impl OkCancelWindow {
    pub fn new(title: &str, mut id: PrefixedId) -> Self {
        Self { 
            panel_id: id.derive().id(),
            id, 
            title: title.to_string(),
            ok_button: "Ok".to_string(),
            cancel_button: "Cancel".to_string(),
            close_button: true,
            resizable: false,
        }
    }

    pub fn ok_button(mut self, name: &str) -> Self {
        self.ok_button = name.to_string();
        self
    }

    pub fn cancel_button(mut self, name: &str) -> Self {
        self.cancel_button = name.to_string();
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
            egui::TopBottomPanel::bottom(self.id.derive_suffix("bp").id()).show_inside(ui, |ui| {
                if ui.button(&self.ok_button).clicked() {
                    result = OkCancel::Ok;
                }
                if ui.button(&self.cancel_button).clicked() {
                    result = OkCancel::Cancel;
                }
            });            
            egui::CentralPanel::default().show_inside(ui, |ui| {
                result = func(ui);
            });
        });
        
        if !close_not_clicked { return OkCancel::Cancel };

        result
    }
}