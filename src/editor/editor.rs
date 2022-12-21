use eframe::egui::Ui;


pub trait ItemEditor {
    type Item;

    fn show(&mut self, ui: &mut Ui, item: &mut Self::Item);
}
