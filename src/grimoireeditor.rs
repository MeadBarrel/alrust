use egui::Ui;
use crate::wishes::Wishes;
use crate::grimoire_state::GrimoireState;


pub enum GrimoireEditorTab {
    Characters,
    Skills,
    Ingredients,
}

pub struct GrimoireEditor {
    grimoire: GrimoireState,
    tab: GrimoireEditorTab,
}


impl GrimoireEditor {
    pub fn show(&mut self, ui: &mut Ui, wishes: &mut Wishes) {
        ui.label("Hello Grimoire!");
    }
}