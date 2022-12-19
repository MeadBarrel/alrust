use eframe::egui::Ui;
use egui::{Layout, Widget};
use error_stack::{IntoReport, ResultExt};
use crate::error::{Result, Error, handle_result};
use crate::grimoire_state::GrimoireState;
use crate::publicstate::{PublicState, GrimoireEditTab};

use grimoire2::grimoire::versioned::GrimoireVersioned;
use grimoire2::grimoire::Grimoire;


pub fn top_panel(ui: &mut Ui, state: &mut PublicState) {
    let layout = Layout::left_to_right(egui::Align::Center);
        
    ui.with_layout(layout, |ui| {
        grimoire_panel_frame(ui, state);
        ui.separator();

        if state.grimoire_state.is_some() {
            tabs_panel_frame(ui, state);
        }
    });

}


fn tabs_panel_frame(ui: &mut Ui, state: &mut PublicState) {
    let margin = egui::style::Margin { left: 10., ..Default::default() };
    let frame = egui::Frame::none()
        .inner_margin(margin); 

    frame.show(ui, |ui| {
        tabs_panel(ui, state);
    });
}


fn tabs_panel(ui: &mut Ui, state: &mut PublicState) {
    tab_button(ui, state, "Characters", GrimoireEditTab::Characters);
    tab_button(ui, state, "Skills", GrimoireEditTab::Skills);
    tab_button(ui, state, "Ingredients", GrimoireEditTab::Ingredients);
}


fn tab_button(ui: &mut Ui, state: &mut PublicState, text: &str, tab: GrimoireEditTab) {
    let selected = state.grimoire_edit_tab == tab;

    let mut button = egui::widgets::Button::new(text);

    if selected {
        button = button.fill(egui::Color32::from_rgb_additive(0, 0, 68));
    }

    if button.ui(ui).clicked() {
        state.grimoire_edit_tab = tab;
    }
}


fn grimoire_panel_frame(ui: &mut Ui, state: &mut PublicState) {
    let frame = egui::Frame::none()
        .inner_margin(2.5);
    frame.show(ui, |ui| grimoire_panel(ui, state));
}


fn grimoire_panel(ui: &mut Ui, state: &mut PublicState) {
    let grimoire_loaded_result = grimoire_select_button(ui, state);
    handle_result(state, grimoire_loaded_result);

    if state.grimoire_state.is_some() {
        grimoire_close_button(ui, state);
    }    
}


fn grimoire_select_button(ui: &mut Ui, state: &mut PublicState) -> Result<()> {
    if ui.button("Load Grimoire").clicked() {
        let dialog = rfd::FileDialog::new().add_filter("Grimoire as JSON", &["json"]);

        if let Some(path) = dialog.pick_file() {
            let grimoire = load_grimoire(path)?;
            state.grimoire_state = Some(GrimoireState::new(grimoire));
        }        
    }

    Ok(())
}

fn grimoire_close_button(ui: &mut Ui, state: &mut PublicState) {
    if ui.button("Close").clicked() {
        let dialog = rfd::MessageDialog::new()
            .set_buttons(rfd::MessageButtons::YesNo)
            .set_title("Are you sure?")
            .set_description("You sure you want to stop working with this grimoire?")
            .set_level(rfd::MessageLevel::Warning);
        if dialog.show() {
            state.grimoire_state = None;
        }
    }
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