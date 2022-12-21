use eframe::egui::Ui;
use grimoire2::prelude::{Characters, Character};
use crate::widget::{SetStringWindow, OkCancel};

#[derive(Debug, Default)]
pub struct CreateCharacterWindows {
    windows: Vec<SetStringWindow>
}


impl CreateCharacterWindows {
    pub fn show(&mut self, ui: &mut Ui, characters: &mut Characters) {
        self.windows.retain_mut(|window| {
            match window.show(ui).augment {
                OkCancel::Cancel => false,
                OkCancel::None => true,
                OkCancel::Ok => {
                    characters.insert(window.get_value().to_string(), Character::default());
                    false
                },
            }
        });        
    }

    pub fn add(&mut self) {
        self.windows.push(
            SetStringWindow::default().title("Create new Character")            
        )
    }
}