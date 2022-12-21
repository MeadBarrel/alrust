use eframe::egui::Ui;
use grimoire2::grimoire::Characters;
use crate::id::PrefixedId;
use eframe::egui;
use crate::widget::{SetStringWindow, OkCancel};
use super::character;

#[derive(Debug, Default)]
pub struct Editor {
    id: PrefixedId,
    create_windows: Vec<SetStringWindow>,
}


impl Editor {
    pub fn show(&mut self, ui: &mut Ui, characters: &mut Characters) {
        egui::TopBottomPanel::top(self.id.derive_suffix("top")).show_inside(ui, |ui| {
            self.top_panel(ui);
        });
        egui::CentralPanel::default().show_inside(ui, |ui| {
            self.central_panel(ui, characters);
        });

        self.handle_create_windows(ui);
    }

    fn top_panel(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            self.top_panel_right(ui);
        });
    }

    fn top_panel_right(&mut self, ui: &mut Ui) {
        let right_to_left = egui::Layout::right_to_left(egui::Align::Center);
        ui.with_layout(right_to_left, |ui| {
            self.add_character_button(ui);
        });
    }

    fn add_character_button(&mut self, ui: &mut Ui) {
        if ui.button("Add Character").clicked() {
            self.create_windows.push(
                SetStringWindow::default()
                    .title("Create new Character")
            )
        };
    }

    fn central_panel(&mut self, ui: &mut Ui, characters: &mut Characters) {

    }

    fn handle_create_windows(&mut self, ui: &mut Ui) {
        self.create_windows.retain_mut(|window| {
            match window.show(ui) {
                OkCancel::Cancel => false,
                OkCancel::None => true,
                OkCancel::Ok => true,
            }
        });
    }
}

