use eframe::egui::Ui;
use crate::wishes::Wishes;
use crate::grimoire_state::GrimoireState;


pub fn editor(ui: &mut Ui, wishes: &mut Wishes, state: &mut GrimoireState) {
    let layout = egui::Layout::top_down(egui::Align::Max);

    ui.with_layout(layout, |ui| {
        top_panel(ui, wishes, state);
    });

    ui.label("This is characters editor");
}