use eframe::egui::Ui;
use crate::wishes::Wishes;
use crate::grimoire_state::GrimoireState;


#[derive(Debug, Clone, Default)]
pub struct State {}


pub fn editor(ui: &mut Ui, wishes: &mut Wishes, state: &mut State, grimoire: &mut GrimoireState) {
    let layout = egui::Layout::top_down(egui::Align::Max);

    ui.with_layout(layout, |ui| {
        top_panel(ui, state, grimoire);
    });

    ui.label("This is characters editor");
}


fn top_panel(ui: &mut Ui, state: &mut State, grimoire: &mut GrimoireState) {

}