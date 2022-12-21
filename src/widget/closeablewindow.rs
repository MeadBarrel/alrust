use eframe::egui;
use egui::InnerResponse;
use crate::types::*;
use crate::id::PrefixedId;

#[derive(Debug, Default)]
pub struct CloseableWindow {
    id: PrefixedId,
    title: String,
    resizable: bool,
    collapsible: bool,
    auto_sized: bool,
    default_pos: Option<egui::Pos2>,
}

impl CloseableWindow {
    pub fn id(mut self, id: PrefixedId) -> Self {
        self.id = id;
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    pub fn collabsible(mut self, collapsible: bool) -> Self {
        self.collapsible = collapsible;
        self
    }

    pub fn auto_sized(mut self) -> Self {
        self.auto_sized = true;
        self
    }

    pub fn default_pos(mut self, pos: impl Into<egui::Pos2>) -> Self {
        self.default_pos = Some(pos.into());
        self
    }


    pub fn show(&mut self, ui: &mut egui::Ui, func: impl FnOnce(&mut egui::Ui) -> bool) -> AugmentedWindowResponse<bool, ()> {
        let mut open = true;
        let mut func_open = true;

        let mut window = egui::Window::new(&self.title)
            .id(self.id.id())
            .open(&mut open)
            .resizable(self.resizable)
            .collapsible(self.collapsible);
        if let Some(x) = self.default_pos {
            window = window.default_pos(x);
        }
        if self.auto_sized {
            window = window.auto_sized()
        }
        let maybe_response = window.show(ui.ctx(), |ui| {
                func_open = func(ui);                
            });

        match maybe_response {
            None => AugmentedWindowResponse::new(false, None),
            Some(x) => AugmentedWindowResponse::new(
                open && func_open, Some(InnerResponse::new(Some(()), x.response))
            ),
        }
    }
}