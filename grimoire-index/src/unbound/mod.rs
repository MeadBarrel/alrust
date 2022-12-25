mod grimoire;
mod character;
mod skill;
mod ingredient;

use grimoire2::{grimoire::Grimoire, prelude::{Character, Skill, Ingredient}};

pub trait UnboundIndex {
    type Item;

    fn get_mut<'a>(&self, source: &'a mut Grimoire) -> Option<&'a mut Self::Item>;
    fn get<'a>(&self, source: &'a Grimoire) -> Option<&'a Self::Item>;
}

pub trait UnboundGrimoireBacklink {
    type Backlink: UnboundIndex<Item = Grimoire>;

    fn grimoire(&self) -> &Self::Backlink;
}

pub trait UnboundCharacterBacklink {
    type Backlink: UnboundIndex<Item = Character>;

    fn character(&self) -> &Self::Backlink;
}

pub trait UnboundSkillBacklink {
    type Backlink: UnboundIndex<Item = Skill>;

    fn skill(&self) -> &Self::Backlink;
}

pub trait UnboundIngredientBacklink {
    type Backlink: UnboundIndex<Item = Ingredient>;

    fn Ingredient(&self) -> &Self::Backlink;
}


