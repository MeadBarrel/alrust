pub mod grimoire;
pub mod character;
pub mod skill;
pub mod ingredient;

use grimoire2::{grimoire::Grimoire, prelude::{Character, Skill, Ingredient}};

pub trait UnboundIndex {
    type Item;

    fn get_mut<'a>(&self, source: &'a mut Grimoire) -> Option<&'a mut Self::Item>;
    fn get<'a>(&self, source: &'a Grimoire) -> Option<&'a Self::Item>;
}

pub trait GrimoireBacklink {
    type Backlink: UnboundIndex<Item = Grimoire>;

    fn grimoire(&self) -> &Self::Backlink;
}

pub trait CharacterBacklink {
    type Backlink: UnboundIndex<Item = Character>;

    fn character(&self) -> &Self::Backlink;
}

pub trait SkillBacklink {
    type Backlink: UnboundIndex<Item = Skill>;

    fn skill(&self) -> &Self::Backlink;
}

pub trait IngredientBacklink {
    type Backlink: UnboundIndex<Item = Ingredient>;

    fn ingredient(&self) -> &Self::Backlink;
}

pub mod prelude {
    pub use super::grimoire::*;
    pub use super::character::*;
    pub use super::skill::*;
    pub use super::ingredient::*;
}