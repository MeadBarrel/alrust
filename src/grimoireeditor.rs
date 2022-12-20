use egui::Ui;
use crate::events::{Event, Events};
use crate::grimoire_state::GrimoireState;


pub enum GrimoireEditorTab {
    Characters,
    Skills,
    Ingredients,
}

pub struct GrimoireEditor {
    grimoire: GrimoireState,
}


impl GrimoireEditor {
    pub fn show(&mut self, ui: &mut Ui, tab: GrimoireEditorTab) -> Events {
        Events::default()
    }

}