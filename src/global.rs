use egui::Ui;


pub fn show_info(ui: &mut Ui, title: &str, text: &str) {
    rfd::MessageDialog::new()
        .set_level(rfd::MessageLevel::Info)
        .set_description(text)
        .set_title(title)
        .show();
}