mod character;
mod characters;

use eframe::egui::Ui;
use grimoire2::prelude::Grimoire;

use crate::id::PrefixedId;


#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum Tab {
    #[default]
    Characters,
    Skills,
    Ingredients,
}


#[derive(Debug, Default)]
pub struct GrimoireEditor {
    pub grimoire: Grimoire,
    pub tab: Tab,
    pub id: PrefixedId,

    pub characters_editor: characters::CharactersEditor,
}


impl GrimoireEditor {
    pub fn new(grimoire: Grimoire) -> Self {
        Self {
            grimoire: grimoire,
            ..Default::default()
        }
    }

    pub fn show(&mut self, ui: &mut Ui) {        
        eframe::egui::TopBottomPanel::top(self.id.derive_suffix("top_panel")).show_inside(ui, |ui| {
                self.top_panel(ui);
            });
        eframe::egui::CentralPanel::default().show_inside(ui, |ui| {
            self.central_panel(ui);
        });
    }

    fn central_panel(&mut self, ui: &mut Ui) {
        match self.tab {
            Tab::Characters => self.characters_editor.show(ui, &mut self.grimoire.characters),
            Tab::Skills => { ui.heading("Work in Progress..."); },
            Tab::Ingredients => { ui.heading("Work in Progress..."); },
        }
        
    }

    fn top_panel(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            self.tabs_panel(ui);
        });
        
    }

    fn tabs_panel(&mut self, ui: &mut Ui) {
        self.tab_button(ui, "Characters", Tab::Characters);
        self.tab_button(ui, "Skills", Tab::Skills);
        self.tab_button(ui, "Ingredients", Tab::Ingredients);
    }

    fn tab_button(&mut self, ui: &mut Ui, text: &str, tab: Tab) {
        use egui::widgets::Button;
        use egui::widgets::Widget;
    
        let selected = self.tab == tab;
        
        let mut button = Button::new(text);
        
        if selected {
            button = button.fill(egui::Color32::from_rgb_additive(0, 0, 64));
        }
    
        if button.ui(ui).clicked() {
            self.tab = tab;
        };        
    }
}
