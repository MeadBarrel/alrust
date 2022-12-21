pub mod characters;
pub mod character;

use eframe::egui::Ui;
use crate::wishes::Wishes;
use grimoire2::grimoire::Grimoire;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum Tab {
    #[default]
    Characters,
    Skills,
    Ingredients,
}

#[derive(Debug, Default)]
pub struct State {
    pub grimoire: Grimoire,
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

fn skills_editor(ui: &mut Ui, wishes: &mut Wishes, state: &mut Grimoire) {
    ui.label("This is skills editor");
}

fn ingredients_editor(ui: &mut Ui, wishes: &mut Wishes, state: &mut Grimoire) { 
    ui.label("This is ingredients editor");
}