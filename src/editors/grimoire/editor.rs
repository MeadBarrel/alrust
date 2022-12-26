use egui::Ui;
use crate::id::PrefixedId;
use grimoire2::grimoire::Grimoire;
use super::characters::editor::CharactersEditor;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum Tab {
    #[default]
    Characters,
    Skills,
    Ingredients,
}

#[derive(Debug, Default)]
pub struct GrimoireEditor {
    id: PrefixedId,
    tab: Tab,
    characters_editor: CharactersEditor,
}

impl GrimoireEditor {
    pub fn show(&mut self, ui: &mut Ui, mut grimoire: Grimoire) -> Grimoire {
        eframe::egui::TopBottomPanel::top(self.id.derive_suffix("top_panel")).show_inside(ui, |ui| {
                self.top_panel(ui);
            });
        eframe::egui::CentralPanel::default().show_inside(ui, |ui| {
            self.central_panel(ui, &mut grimoire)
        });

        grimoire
    }

    pub fn central_panel(&mut self, ui: &mut Ui, grimoire: &mut Grimoire) {
        self.characters_editor.show(ui, grimoire);
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

