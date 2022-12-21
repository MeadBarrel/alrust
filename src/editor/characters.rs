use eframe::egui::Ui;
use crate::wishes::Wishes;
use grimoire2::grimoire::Grimoire;
use super::character;
use crate::widget::{OkCancelWindow, OkCancel};
use crate::id::PrefixedId;



#[derive(Debug, Default)]
pub struct State {
    //creating_characters: Vec<(egui::Id, String)>,
    id: PrefixedId,
    creating_characters: Vec<(String, OkCancelWindow)>,
}


pub fn editor(ui: &mut Ui, wishes: &mut Wishes, state: &mut State, grimoire: &mut Grimoire) {
    egui::TopBottomPanel::top(state.id.derive_suffix("tbp"))
        .show(ui.ctx(), |ui| {
            top_panel(ui, state)
        });
    
    state.creating_characters.retain_mut(|(name, window)| {
        window.show(ui, |ui| {
            let text_edit_has_focus = ui.text_edit_singleline(name).lost_focus();
            if text_edit_has_focus && ui.ctx().input().key_pressed(egui::Key::Escape) {
                return OkCancel::Cancel
            }        
            OkCancel::None
        }) == OkCancel::None
    });
}

fn top_panel(ui: &mut Ui, state: &mut State) {
    let layout = egui::Layout::right_to_left(egui::Align::Center);

    ui.with_layout(layout, |ui| {
        ui.ctx().
        if ui.button("Add Character").clicked() {
            state.creating_characters.push(
                ("".to_string(), 
                OkCancelWindow::new("Create a new Character", state.id.derive()).resizable(true)
            ));
        }
    });
}

//     if state.id.is_none() { state.id = Some(wishes.counter()) }

//     let top_panel_id = egui::Id::new(format!("character_editor_tb_{}", state.id.unwrap()));

//     egui::TopBottomPanel::top(top_panel_id).show(ui.ctx(), |ui| {
//         top_panel(ui, wishes, state);
//     });

//     state.creating_characters.retain_mut(|(id, name)| {
//         let mut window_close_not_clicked = true;
//         let mut window_state = OkCancelState::None;
        
//         egui::Window::new("Type a name for new character")
//             .id(*id)
//             .open(&mut window_close_not_clicked)
//             .show(ui.ctx(), |ui| {
//                 window_state = create_character_window(ui, name)
//             });

//             if window_state == OkCancelState::Ok {
//                 let info = CharacterEditInfo {
//                     id: egui::Id::new(wishes.counter()),
//                     name: name.clone(),
//                     type_: CharacterEditType::New,
//                     state: character::State {
//                         id: wishes.counter(),
//                         character: grimoire2::grimoire::Character::default(),
//                     }
//                 };
//                 state.editing_characters.push(info);
//             }

//             window_close_not_clicked && window_state != OkCancelState::None
//     });

//     state.editing_characters.retain_mut(|info| {
//         let mut window_close_not_clicked = true;
//         let mut open = true;

//         egui::Window::new("Character editor") 
//             .id(info.id)
//             .open(&mut window_close_not_clicked)
//             .show(ui.ctx(), |ui| {
//                 edit_character_window(ui, wishes, grimoire, info)
//             });

//         window_close_not_clicked && open
//     });
// }


// fn edit_character_window(ui: &mut Ui, wishes: &mut Wishes, grimoire: &mut Grimoire, info: &mut CharacterEditInfo) -> bool {
//     let mut open = true;
//     character::editor(ui, wishes, &mut info.state);
//     open
// }


// fn create_character_window(ui: &mut Ui, name: &mut String) -> OkCancelState {
//     ui.vertical(|ui| {
//         let text_edit_has_focus = ui.text_edit_singleline(name).lost_focus();
//         if text_edit_has_focus && ui.ctx().input().key_pressed(egui::Key::Escape) {
//             return OkCancelState::Cancel
//         }        

//         ui.horizontal(|ui| {
//             if ui.button("Add").clicked() {
//                 return OkCancelState::Ok;
//             }
//             if ui.button("Cancel").clicked() {
//                 return OkCancelState::Cancel;
//             }
//             OkCancelState::None
//         }).inner
//     }).inner
// }
