use std::marker::PhantomData;
use eframe::egui::Ui;
use egui::Window;
use grimoire2::indexmap::IndexMap;
use crate::id::PrefixedId;


type Items<T> = IndexMap<String, T>;

#[derive(Debug, Default)]
pub struct ItemCreationWindows<T> {
    title: String,
    ids: Vec<(PrefixedId, String)>,
    type_: PhantomData<T>
}

impl<T> ItemCreationWindows<T> 
    where T: Default
{
    pub fn show(&mut self, ui: &mut Ui, items: &mut Items<T>) {
        self.ids.retain_mut(|(id, name)| {
            let mut open = true;
            let window_result = Window::new(&self.title).id(id.id())
                .open(&mut open)
                .show(ui.ctx(), |ui| {
                    set_string(ui, name)
                });

            match window_result {
                Some(x) => match x.inner {
                    Some(x) => match x {
                        SetStringResult::Ok => {
                            items.insert(name.clone(), T::default());
                            false
                        },
                        SetStringResult::Cancel => false,
                        SetStringResult::None => true,
                    },
                    None => open,
                },
                None => false,
            }
        });
    }

    pub fn add(&mut self) {
        self.ids.push(
            (PrefixedId::default(), "".to_string())
        )
    }
}


pub enum SetStringResult {
    Ok,
    Cancel,
    None
}


fn set_string(ui: &mut Ui, value: &mut String) -> SetStringResult {
    ui.vertical(|ui| {
        ui.text_edit_singleline(value);
        ui.horizontal(|ui| {
            if ui.button("Ok").clicked() {
                return SetStringResult::Ok;
            }
            if ui.button("Cancel").clicked() {
                return SetStringResult::Cancel;
            }
            SetStringResult::None
        }).inner
    }).inner
}