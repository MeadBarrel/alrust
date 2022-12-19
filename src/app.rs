use eframe::egui;


pub fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Alrust", native_options, Box::new(|cc| Box::new(AlrustApp::new(cc))));
}


#[derive(Default)]
struct AlrustApp {}


impl AlrustApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}


impl eframe::App for AlrustApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello WOrld")
        });
    }
}