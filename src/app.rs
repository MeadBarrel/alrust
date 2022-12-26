use crate::editors::grimoire::editor::GrimoireEditor;
use grimoire2::grimoire::Grimoire;


#[derive(Default)]
pub struct AlrustApp {
    pub grimoire_editor: GrimoireEditor,
    pub grimoire: Option<Grimoire>,
}


impl AlrustApp {
    fn new(_: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}


impl eframe::App for AlrustApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::TopBottomPanel::top("ar_top_panel").show(ctx, |ui| {
            crate::toppanel::top_panel(ui, self)
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.grimoire = match std::mem::take(&mut self.grimoire) {
                Some(x) => {
                    Some(self.grimoire_editor.show(ui, x))
                },
                None => { ui.heading("Load grimoire first"); None }
            }
        });
    }
}


pub fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Alrust", native_options, Box::new(|cc| Box::new(AlrustApp::new(cc))));
}
