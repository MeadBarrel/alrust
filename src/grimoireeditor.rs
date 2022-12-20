use eframe::egui::Ui;
use crate::wishes::Wishes;
use crate::grimoire_state::GrimoireState;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum GrimoireEditTab {
    #[default]
    Characters,
    Skills,
    Ingredients,
}

#[derive(Debug, Default)]
pub struct GrimoireEditorState {
    pub grimoire: crate::grimoire_state::GrimoireState,
    pub current_tab: GrimoireEditTab,
}

pub fn grimoire_editor(ui: &mut Ui, wishes: &mut Wishes, state: &mut GrimoireEditorState) {

}

fn characters_editor(ui: &mut Ui, wishes: &mut Wishes, state: &mut GrimoireState) {
}

fn skills_editor(ui: &mut Ui, wishes: &mut Wishes, state: &mut GrimoireState) {
}

fn ingredients_editor(ui: &mut Ui, wishes: &mut Wishes, state: &mut GrimoireState) { 
}