use std::marker::PhantomData;

use eframe::egui::Ui;
use crate::id::PrefixedId;
use crate::widget::CloseableWindow;
use crate::types::AugmentedWindowResponse;
use crate::error::{Report, Error, handle_error};
use grimoire2::indexmap::IndexMap;
use super::editor::ItemEditor;


type Items<T> = IndexMap<String, T>;


#[derive(Debug, Default)]
pub struct ItemEditionWindows<T, E> 
{
    windows: Vec<EditWindow<T, E>>,
    type_: PhantomData<T>
}

impl<T, E> ItemEditionWindows<T, E> 
    where 
        T: Default,
        E: ItemEditor<Item=T> + Default,
{
    pub fn show(&mut self, ui: &mut Ui, items: &mut Items<T>) {
        self.windows.retain_mut(|window| {
            let character = items.get_mut(&window.name);
            match character {
                Some(x) => window.show(ui, x).augment,
                None => {
                    handle_error(
                        Report::new(Error::Generic(
                            "Character deleted, but character edition window was open, closing".to_string()
                        ))
                    );
                    false
                }
            }
            
        })
    }

    pub fn add(&mut self, name: impl Into<String>) 
    {
        self.windows.push(
            EditWindow::new(name, E::default())
        )
    }
}


#[derive(Debug)]
struct EditWindow<T, E> 
{
    pub name: String,
    pub editor: E,
    pub id: PrefixedId,
    pub default_pos: Option<egui::Pos2>,
    type_: PhantomData<T>
}



impl<T, E> EditWindow<T, E> 
    where
        E: ItemEditor<Item=T>
{
    pub fn new(name: impl Into<String>, editor: E) -> Self {
        Self {
            name: name.into(),
            editor,
            id: PrefixedId::default(),
            default_pos: None,
            type_: PhantomData
        }
    }

    pub fn default_pos(mut self, pos: impl Into<egui::Pos2>) -> Self {
        self.default_pos = Some(pos.into());
        self
    }

    pub fn show(&mut self, ui: &mut Ui, character: &mut T) -> AugmentedWindowResponse<bool, ()> {
        let mut window = CloseableWindow::default()
            .id(self.id.derive_suffix("character_edit_window"))
            .auto_sized()
            .collabsible(true)
            .title(&self.name);
        
        if let Some(x) = self.default_pos {
            window = window.default_pos(x)
        }

        window
            .show(ui, |ui| {
                self.editor.show(ui, character);
                true
            })
    }
}