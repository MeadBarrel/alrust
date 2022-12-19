use super::Conn;
use crate::schema::*;
use diesel::{associations::HasTable, prelude::*};
use grimoire2::prelude as g2;

#[derive(Debug, Clone, Queryable, Insertable, AsChangeset, Identifiable)]
#[diesel(table_name=ingredients, primary_key(name))]
pub struct Ingredient {
    pub name: String,
    pub lore: Option<String>,
    pub al_weight: i32,
    pub dh: Option<f64>,
    pub dp: Option<f64>,
    pub mdh: Option<f64>,
    pub mdp: Option<f64>,
    pub hot: Option<f64>,
    pub pot: Option<f64>,
    pub mhot: Option<f64>,
    pub mpot: Option<f64>,
    pub hl: Option<f64>,
    pub pl: Option<f64>,
    pub mhl: Option<f64>,
    pub mpl: Option<f64>,
    pub a: Option<f64>,
    pub ma: Option<f64>,
    pub notes: Option<String>,
}

impl Ingredient {
    pub fn load(conn: &mut Conn) -> QueryResult<Vec<Ingredient>> {
        Ingredient::table().load(conn)
    }

    pub fn from_grimoire(name: &str, src: &g2::Ingredient) -> Self {
        use grimoire2::effect::Effect;

        Self {
            name: name.to_string(),
            lore: src.skill.clone(),
            al_weight: src.weight as i32,

            dh: src.modifiers[Effect::DirectHealing].term.into(),
            mdh: src.modifiers[Effect::DirectHealing].multiplier.into(),

            dp: src.modifiers[Effect::DirectPoison].term.into(),
            mdp: src.modifiers[Effect::DirectPoison].multiplier.into(),

            hot: src.modifiers[Effect::HealingOverTime].term.into(),
            mhot: src.modifiers[Effect::HealingOverTime].multiplier.into(),

            pot: src.modifiers[Effect::PoisonOverTime].term.into(),
            mpot: src.modifiers[Effect::PoisonOverTime].multiplier.into(),

            hl: src.modifiers[Effect::HealingLength].term.into(),
            mhl: src.modifiers[Effect::HealingLength].multiplier.into(),

            pl: src.modifiers[Effect::PoisonLength].term.into(),
            mpl: src.modifiers[Effect::PoisonLength].multiplier.into(),

            a: src.modifiers[Effect::Alcohol].term.into(),
            ma: src.modifiers[Effect::Alcohol].multiplier.into(),

            notes: None,
        }
    }

    pub fn to_grimoire(&self) -> (String, g2::Ingredient) {
        use grimoire2::prelude::Effect;

        let ingredient = g2::Ingredient {
            weight: self.al_weight > 0,
            skill: self.lore.clone(),
            modifiers: vec![
                (Effect::DirectHealing, (self.dh, self.mdh).into()),
                (Effect::DirectPoison, (self.dp, self.mdp).into()),
                (Effect::HealingOverTime, (self.hot, self.mhot).into()),
                (Effect::PoisonOverTime, (self.pot, self.mpot).into()),
                (Effect::HealingLength, (self.hl, self.mhl).into()),
                (Effect::PoisonLength, (self.pl, self.mpl).into()),
                (Effect::Alcohol, (self.a, self.ma).into()),
            ]
            .into(),
        };

        (self.name.clone(), ingredient)
    }
}
