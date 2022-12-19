use crate::grimoire_state::GrimoireState;


#[derive(Default, PartialEq, Eq)]
pub enum GrimoireEditTab {
    #[default]
    Characters,
    Skills,
    Ingredients,
}


#[derive(Default)]
pub struct PublicState {
    pub grimoire_state: Option<GrimoireState>,
    pub grimoire_edit_tab: GrimoireEditTab,    
}