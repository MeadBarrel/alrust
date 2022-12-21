use eframe::egui::Id;
use std::sync::atomic::AtomicUsize;


static COUNTER: AtomicUsize = AtomicUsize::new(0);


#[derive(Debug)]
pub struct PrefixedId {
    counter: AtomicUsize,
    id_string: String
}


impl Default for PrefixedId {
    fn default() -> Self {
        let counter = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Self::new("prefixed_", counter)
    }
}


impl PrefixedId {
    pub fn new(prefix: &str, counter: usize) -> Self {
        Self { counter: AtomicUsize::new(counter), id_string: format!("{}_{}", prefix, counter) }
    }

    pub fn id(&self) -> Id {
        Id::new(&self.id_string)
    }

    pub fn derive(&mut self) -> Self {
        Self::new(&self.id_string, self.counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed))
    }

    pub fn derive_suffix(&self, suffix: &str) -> Self {
        Self::new(&format!("{}_{}", &self.id_string, suffix), 0)
    }
}



impl From<PrefixedId> for Id {
    fn from(value: PrefixedId) -> Self {
        value.id()
    }
}