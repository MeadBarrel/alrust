use eframe::egui::InnerResponse;


pub type WindowResponse<T> = Option<InnerResponse<Option<T>>>;
pub struct AugmentedWindowResponse<A, T> {
    pub augment: A,
    pub response: WindowResponse<T>
}

impl<A, T> AugmentedWindowResponse<A, T> {
    pub fn new(augment: A, response: WindowResponse<T>) -> Self{
        Self { augment, response }
    }
}