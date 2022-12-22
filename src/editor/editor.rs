use eframe::egui::Ui;
use grimoire2::prelude::Grimoire;


pub trait ItemEditor {
    type Item;

    fn show(&mut self, ui: &mut Ui, item: &mut Self::Item);
}
