use egui::Ui;
use crate::publicstate::PublicState;


#[derive(Debug, Default)]
pub struct CharactersEditor {}


impl CharactersEditor {
    pub fn show(&mut self, ui: &mut Ui, state: &mut PublicState) {
        let layout = egui::Layout::top_down(egui::Align::Max);

        ui.with_layout(layout, |ui| {
            self.top_panel(ui, state);
        });
    }

    fn top_panel(&mut self, ui: &mut Ui, state: &mut PublicState) {
        let layout = egui::Layout::right_to_left(egui::Align::Center);
        if ui.button("Add Character").clicked() {

        }
    }
}

