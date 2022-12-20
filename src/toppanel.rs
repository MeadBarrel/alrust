use eframe::egui::Ui;
use crate::app::AppState;
use crate::wishes::Wishes;
use error_stack::*;
use grimoire2::grimoire::versioned::GrimoireVersioned;
use grimoire2::grimoire::Grimoire;
use crate::grimoire_state::GrimoireState;
use crate::error::{Error, Result};
use crate::editor;


pub fn top_panel(ui: &mut Ui, wishes: &mut Wishes, state: &mut AppState) {
    ui.horizontal(move |ui| {
        menu(ui, wishes, state);
        ui.add_space(5.);
        ui.separator();
        if let Some(editor) = &mut state.grimoire_editor {
            tab_panel(ui, editor)
        };
    });
}


pub fn menu(ui: &mut Ui, wishes: &mut Wishes, state: &mut AppState) {
    ui.menu_button("File", |ui| {
        open_button(ui, wishes, &mut state.grimoire_editor);
        if state.grimoire_editor.is_some() { 
            close_button(ui, wishes, &mut state.grimoire_editor)
         }
    });
}


fn tab_panel(ui: &mut Ui, editor: &mut editor::State) {
    tab_button(ui, "Characters", editor, editor::Tab::Characters);
    tab_button(ui, "Skills", editor, editor::Tab::Skills);
    tab_button(ui, "Ingredients", editor, editor::Tab::Ingredients);
}


fn tab_button(ui: &mut Ui, text: &str, editor: &mut editor::State, this_tab: editor::Tab) {
    use egui::widgets::Button;
    use egui::widgets::Widget;

    let selected = editor.current_tab == this_tab;
    
    let mut button = Button::new(text);
    
    if selected {
        button = button.fill(egui::Color32::from_rgb_additive(0, 0, 64));
    }

    if button.ui(ui).clicked() {
        editor.current_tab = this_tab;
    };

}


fn open_button(ui: &mut Ui, wishes: &mut Wishes, maybe_editor: &mut Option<editor::State>) {
    if !ui.button("Open").clicked() { return }

    let dialog = rfd::FileDialog::new().add_filter("Grimoire as JSON", &["json"]);
    let path = if let Some(path) = dialog.pick_file() { path } else { return; };

    match load_grimoire(path) {
        Ok(grimoire) => { 
            *maybe_editor = Some(editor::State { 
                grimoire: GrimoireState::new(grimoire), ..Default::default()
            })
        },
        Err(err) => wishes.handle_error(err)
    }
}


fn close_button(ui: &mut Ui, _: &mut Wishes, maybe_editor: &mut Option<editor::State>) {
    if !ui.button("Close").clicked() { return; }

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