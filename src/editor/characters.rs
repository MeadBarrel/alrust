use eframe::egui::Ui;
use crate::wishes::Wishes;
use grimoire2::grimoire::Grimoire;


#[derive(Debug, Clone, Default)]
pub struct State {
    id: Option<usize>,
    creating_characters: Vec<(egui::Id, String)>,
    editing_characters: Vec<(egui::Id, String)>,
}


pub fn editor(ui: &mut Ui, wishes: &mut Wishes, state: &mut State, grimoire: &mut Grimoire) {
    if state.id.is_none() { state.id = Some(wishes.counter()) }

    let top_panel_id = egui::Id::new(format!("character_editor_tb_{}", state.id.unwrap()));

    egui::TopBottomPanel::top(top_panel_id).show(ui.ctx(), |ui| {
        top_panel(ui, wishes, state);
    });

    state.creating_characters.retain_mut(|(id, name)| {
        let mut window_close_not_clicked = true;
        let mut open = true;
        
        egui::Window::new("Type a name for new character")
            .id(*id)
            .open(&mut window_close_not_clicked)
            .show(ui.ctx(), |ui| {
                open = create_character_window(ui, name)
            });
            
            window_close_not_clicked && open
    })

}


fn create_character_window(ui: &mut Ui, name: &mut String) -> bool {
    let mut open = true;
    ui.vertical(|ui| {
        let text_edit_has_focus = ui.text_edit_singleline(name).lost_focus();
        if text_edit_has_focus && ui.ctx().input().key_pressed(egui::Key::Escape) {
            open = false;
        }        

        ui.horizontal(|ui| {
            if ui.button("Add").clicked() {};
            if ui.button("Cancel").clicked() {
                open = false;
            };    
        })
    });
    
    open
}


fn top_panel(ui: &mut Ui, wishes: &mut Wishes, state: &mut State) {
    let layout = egui::Layout::right_to_left(egui::Align::Center);

    ui.with_layout(layout, |ui| {
        if ui.button("Add Character").clicked() {
            let id = egui::Id::new(wishes.counter());
            state.creating_characters.push((id, "".to_string()));
        }
    });
}