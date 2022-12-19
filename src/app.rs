use eframe::egui;
use egui::Ui;

use crate::publicstate::PublicState;
use crate::top_panel::top_panel;
use crate::characters::CharactersEditor;
use crate::publicstate::GrimoireEditTab;



pub fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Alrust", native_options, Box::new(|cc| Box::new(AlrustApp::new(cc))));
}


#[derive(Default)]
struct AlrustApp {
    state: PublicState,
    characters_editor: CharactersEditor,
}


impl AlrustApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    fn grimoire_edit_panel(&mut self, ui: &mut Ui) {
        if self.state.grimoire_state.is_some() {            
            match self.state.grimoire_edit_tab {
                GrimoireEditTab::Characters => self.characters_editor.show(ui, &mut self.state),
                GrimoireEditTab::Skills => {},
                GrimoireEditTab::Ingredients => {},
            }
        } else {
            ui.heading("Load a grimoire first");
        }
    }
}


impl eframe::App for AlrustApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("ar_top_panel").show(ctx, |ui| {
            top_panel(ui, &mut self.state);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.grimoire_edit_panel(ui)
        });
    }

}
