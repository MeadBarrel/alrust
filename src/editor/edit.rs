use std::marker::PhantomData;

use eframe::egui::{Ui, Window};
use crate::id::PrefixedId;
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
            let item = items.get_mut(&window.name);
            match item {
                Some(ok_item) => window.show(ui, ok_item),
                None => {
                    handle_error(
                        Report::new(Error::Generic(
                            "Item deleted, but an edition window was open, closing".to_string()
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
            type_: PhantomData
        }
    }

    pub fn show(&mut self, ui: &mut Ui, character: &mut T) -> bool {
        let mut open = true;

        let mut window = Window::new(&self.name)
            .id(self.id.derive_suffix("character_edit_window").id())
            .open(&mut open)
            .collapsible(true);

        let window_result = window
            .show(ui.ctx(), |ui| {
                self.editor.show(ui, character);
                true
            });

        match window_result {
            Some(x) => open,
            None => false,
        }
    }
}