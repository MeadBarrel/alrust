use eframe::egui::Ui;
use crate::wishes::Wishes;
use grimoire2::grimoire::Grimoire;


#[derive(Debug, Clone, Default)]
pub struct State {
    creating_characters: Vec<(egui::Id, String)>,
    editing_characters: Vec<(egui::Id, String)>,
}


pub fn editor(ui: &mut Ui, wishes: &mut Wishes, state: &mut State, grimoire: &mut Grimoire) {
    egui::TopBottomPanel::top("character_editor_tb").show(ui.ctx(), |ui| {
        top_panel(ui, wishes, state);
    });

    state.creating_characters.retain_mut(|(id, name)| {
        let mut window_close_not_clicked = true;
        let mut open = true;
        
        egui::Window::new("Type a name for new character")
            .id(*id)
            .open(&mut window_close_not_clicked)
            .show(ui.ctx(), |ui| {
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
                })
            });
            
            window_close_not_clicked && open
    })

    // for (id, name) in &mut state.creating_characters {
    //     let mut open = true;
    //     egui::Window::new("Create new Character")
    //         .id(*id)
    //         .open(&mut open)
    //         .show(ui.ctx(), |ui| {
    //         ui.text_edit_singleline(name);
    //     });
    // }
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