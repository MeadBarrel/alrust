use grimoire2::grimoire::Grimoire;
use grimoire2::modify::{
    GrimoireUpdate,
    command::Commands,
    skill::SkillUpdate,
    character::CharacterUpdate,
    ingredient::IngredientUpdate
};


#[derive(Debug)]
pub struct GrimoireState {
    grimoire: Grimoire,
    current: Grimoire,
    updates: GrimoireUpdate,
}


impl GrimoireState {
    pub fn new(grimoire: Grimoire) -> Self {
        Self {
            current: grimoire.clone(),
            grimoire,
            updates: GrimoireUpdate::default(),
        }
    }

    pub fn original(&self) -> &Grimoire {
        &self.grimoire
    }

    pub fn updates(&self) -> &GrimoireUpdate {
        &self.updates
    }

    pub fn current(&self) -> &Grimoire {
        &self.current
    }

    pub fn character(&mut self, name: &str, update: CharacterUpdate) {
        self.updates.character(name, update);        
        self.update();
    }

    pub fn skill(&mut self, name: &str, update: SkillUpdate) {
        self.updates.skill(name, update);
        self.update();
    }

    pub fn ingredient(&mut self, name: &str, update: IngredientUpdate) {
        self.updates.ingredient(name, update);
        self.update();
    }

    pub fn remove_character(&mut self, name: &str) {
        self.updates.remove_character(name);
        self.update();
    }

    pub fn remove_skill(&mut self, name: &str) {
        self.updates.remove_skill(name);
        self.update();
    }

    pub fn remove_ingredient(&mut self, name: &str) {
        self.updates.remove_ingredient(name);
        self.update();
    }

    pub fn apply_changes(&mut self) {
        self.updates.update(&mut self.grimoire);
        self.current = self.grimoire.clone();
        self.updates = GrimoireUpdate::default();

    }

    fn update(&mut self) {
        self.updates.combine_last();
        self.current = self.grimoire.clone();
        self.updates.update(&mut self.current);
    }
}