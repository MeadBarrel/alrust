use std::marker::PhantomData;

use eframe::egui::Ui;
use crate::widget::{SetStringWindow, OkCancel};
use grimoire2::indexmap::IndexMap;


type Items<T> = IndexMap<String, T>;


#[derive(Debug, Default)]
pub struct ItemCreationWindows<T> {
    windows: Vec<SetStringWindow>,
    type_: PhantomData<T>
}


impl<T> ItemCreationWindows<T> 
    where T: Default
{
    pub fn show(&mut self, ui: &mut Ui, characters: &mut Items<T>) {
        self.windows.retain_mut(|window| {
            match window.show(ui).augment {
                OkCancel::Cancel => false,
                OkCancel::None => true,
                OkCancel::Ok => {
                    characters.insert(window.get_value().to_string(), T::default());
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