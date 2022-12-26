use eframe::egui::Ui;
use crate::app::AlrustApp;
use error_stack::*;
use grimoire2::grimoire::versioned::GrimoireVersioned;
use grimoire2::grimoire::Grimoire;
use crate::error::{Error, Result, handle_error};


pub fn top_panel(ui: &mut Ui, app: &mut AlrustApp) {
    ui.horizontal(move |ui| {
        menu(ui, app);
        ui.add_space(5.);
        ui.separator();
    });
}


pub fn menu(ui: &mut Ui, app: &mut AlrustApp) {
    ui.menu_button("File", |ui| {
        open_button(ui, &mut app.grimoire);
        if app.grimoire.is_some() {
            close_button(ui, &mut app.grimoire)
         }
    });
}



fn open_button(ui: &mut Ui, maybe_grimoire: &mut Option<Grimoire>) {
    if !ui.button("Open").clicked() { return }

    ui.close_menu();

    let dialog = rfd::FileDialog::new().add_filter("Grimoire as JSON", &["json"]);
    let path = if let Some(path) = dialog.pick_file() { path } else { return; };

    match load_grimoire(path) {
        Ok(grimoire) => { 
            *maybe_grimoire = Some(grimoire)
        },
        Err(err) => handle_error(err)
    }
}


fn close_button(ui: &mut Ui, maybe_editor: &mut Option<Grimoire>) {
    if !ui.button("Close").clicked() { return; }

    ui.close_menu();

    let ok = rfd::MessageDialog::default()
        .set_level(rfd::MessageLevel::Warning)
        .set_title("Are you sure?")
        .set_description("Are you sure you want to close the grimoire?")
        .set_buttons(rfd::MessageButtons::OkCancel)
        .show();
    
    if ok { *maybe_editor = None }
}


fn load_grimoire(path: std::path::PathBuf) -> Result<Grimoire> {
    let f = std::fs::File::open(path)
        .into_report()
        .change_context(Error::CouldNotOpenFile)?;
    let grimoire_ver: GrimoireVersioned = serde_json::from_reader(f)
        .into_report()
        .change_context(Error::DeserializationFailed)?;
    Ok(grimoire_ver.into())
}