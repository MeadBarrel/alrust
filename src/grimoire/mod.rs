mod grimoire;
mod character;
mod characterskill;
mod skill;
mod ingredient;

pub use grimoire::*;
pub use character::*;
pub use characterskill::*;
use grimoire2::grimoire::{Grimoire, Ingredient};
use grimoire2::prelude::Character;
pub use skill::*;
pub use ingredient::*;

pub trait IndexElement {
    type Item;

    fn get_mut(&mut self) -> Option<&mut Self::Item>;
    fn get(&self) -> Option<&Self::Item>;
}

pub trait CharactersIndex

pub trait GrimoireBackLink {
    type Item: IndexElement<Item=Grimoire>;

    fn grimoire_mut(&mut self) -> &mut Self::Item;
    fn grimoire(&self) -> &Self::Item;
}

pub trait CharacterBackLink {
    type Item: IndexElement<Item=Character>;

    fn character_mut(&mut self) -> &mut Self::Item;
    fn character(&self) -> &Self::Item;
}

pub trait IngredientBackLink {
    type Item: IndexElement<Item=Ingredient>;

    fn ingredient_mut(&mut self) -> &mut Self::Item;
    fn ingredient(&mut self) -> &Self::Item;
}
