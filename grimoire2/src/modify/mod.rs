pub mod character;
pub mod skill;
pub mod ingredient;


use crate::effect::Effect;


pub enum ModifyEffect {
    Term(Effect),
    Multiplier(Effect),
}


pub use ModifyEffect::*;


pub struct GrimoireUpdates {

}