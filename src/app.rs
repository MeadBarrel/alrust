use crate::editor;


#[derive(Default)]
pub struct AlrustApp {
    pub grimoire_editor: Option<editor::Editor>,
}


impl AlrustApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}


impl eframe::App for AlrustApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("ar_top_panel").show(ctx, |ui| {
            crate::toppanel::top_panel(ui, self)
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            match &mut self.grimoire_editor {
                Some(x) => x.show(ui),
                None => { ui.heading("Grimoire not Loaded"); }
            }
        });
    }
}


pub fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Alrust", native_options, Box::new(|cc| Box::new(AlrustApp::new(cc))));
}
