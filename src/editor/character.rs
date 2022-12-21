use eframe::egui::Ui;
use grimoire2::grimoire::Character;
use crate::wishes::Wishes;


#[derive(Debug, Clone)]
pub struct State {
    pub id: usize,    
    pub character: Character,
}


pub fn editor(ui: &mut Ui, wishes: &mut Wishes, state: &mut State) {

}

