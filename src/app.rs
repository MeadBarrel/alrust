use crate::grimoireeditor;
use crate::wishes::Wishes;


#[derive(Default)]
pub struct AppState {
    pub grimoire_editor: Option<grimoireeditor::GrimoireEditorState>,
}


#[derive(Default)]
struct AlrustApp {
    state: AppState,
    wishes: Wishes,
}


impl AlrustApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}


impl eframe::App for AlrustApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let state = &mut self.state;
        let wishes = &mut self.wishes;
        
        egui::TopBottomPanel::top("ar_top_panel").show(ctx, |ui| {
            crate::toppanel::top_panel(ui, wishes, state)
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("hi!")
        });
    }
}


pub fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Alrust", native_options, Box::new(|cc| Box::new(AlrustApp::new(cc))));
}
