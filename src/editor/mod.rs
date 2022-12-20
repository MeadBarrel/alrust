pub mod characters;

use eframe::egui::Ui;
use crate::wishes::Wishes;
use crate::grimoire_state::GrimoireState;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum Tab {
    #[default]
    Characters,
    Skills,
    Ingredients,
}

#[derive(Debug, Default)]
pub struct State {
    pub grimoire: crate::grimoire_state::GrimoireState,
    pub current_tab: Tab,

    pub characters_editor_state: characters::State,
}

pub fn editor(ui: &mut Ui, wishes: &mut Wishes, state: &mut State) {
    match state.current_tab {
        Tab::Characters => characters::editor(ui, wishes, &mut state.characters_editor_state, &mut state.grimoire),
        Tab::Skills => skills_editor(ui, wishes, &mut state.grimoire),
        Tab::Ingredients => ingredients_editor(ui, wishes, &mut state.grimoire),
    }
}

fn skills_editor(ui: &mut Ui, wishes: &mut Wishes, state: &mut GrimoireState) {
    ui.label("This is skills editor");
}

fn ingredients_editor(ui: &mut Ui, wishes: &mut Wishes, state: &mut GrimoireState) { 
    ui.label("This is ingredients editor");
}