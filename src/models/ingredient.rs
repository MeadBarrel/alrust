use diesel::{prelude::*, associations::HasTable};
use super::Connection;
use crate::schema::*;
use grimoire::data;


#[derive(Debug, Clone, Queryable, Insertable, AsChangeset, Identifiable)]
#[diesel(table_name=ingredients, primary_key(name))]
pub struct Ingredient {
    pub name: String,
    pub lore: String,
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
    pub notes: Option<String>
}



impl Ingredient {
    pub fn load(conn: &mut Connection) -> QueryResult<Vec<Ingredient>> {
        Ingredient::table().load(conn)
    }

    pub fn from_grimoire(src: &data::Ingredient) -> Self {
        use grimoire::types::Property;

        Self {
            name: src.name.clone(),
            lore: src.name.clone(),
            al_weight: src.alchemical_weight as i32,

            dh: src.modifiers.get(&Property::DirectHealing).cloned().unwrap_or_default().modifier,
            mdh: src.modifiers.get(&Property::DirectHealing).cloned().unwrap_or_default().multiplier,

            dp: src.modifiers.get(&Property::DirectPoison).cloned().unwrap_or_default().modifier,
            mdp: src.modifiers.get(&Property::DirectPoison).cloned().unwrap_or_default().multiplier,

            hot: src.modifiers.get(&Property::HealingOverTime).cloned().unwrap_or_default().modifier,
            mhot: src.modifiers.get(&Property::HealingOverTime).cloned().unwrap_or_default().multiplier,

            pot: src.modifiers.get(&Property::PoisonOverTime).cloned().unwrap_or_default().modifier,
            mpot: src.modifiers.get(&Property::PoisonOverTime).cloned().unwrap_or_default().multiplier,

            hl: src.modifiers.get(&Property::HealingLength).cloned().unwrap_or_default().modifier,
            mhl: src.modifiers.get(&Property::HealingLength).cloned().unwrap_or_default().multiplier,

            pl: src.modifiers.get(&Property::PoisonLength).cloned().unwrap_or_default().modifier,
            mpl: src.modifiers.get(&Property::PoisonLength).cloned().unwrap_or_default().multiplier,

            a: src.modifiers.get(&Property::Alcohol).cloned().unwrap_or_default().modifier,
            ma: src.modifiers.get(&Property::Alcohol).cloned().unwrap_or_default().multiplier,

            notes: None,
        }
    }

    pub fn to_grimoire(&self) -> data::Ingredient {
        use grimoire::types::{Modifier, Property};

        data::Ingredient {
            name: self.name.clone(),
            alchemical_weight: self.al_weight as u8,
            lore_name: self.lore.clone(),
            modifiers: vec! [
                (
                    Property::DirectHealing, 
                    Modifier::new(self.dh, self.mdh)
                ),
                (
                    Property::DirectPoison, 
                    Modifier::new(self.dp, self.mdp)
                ),
                (
                    Property::HealingOverTime, 
                    Modifier::new(self.hot, self.mhot)
                ),
                (
                    Property::PoisonOverTime, 
                    Modifier::new(self.pot, self.mpot)
                ),
                (
                    Property::HealingLength, 
                    Modifier::new(self.hl, self.mhl)
                ),
                (
                    Property::PoisonLength, 
                    Modifier::new(self.pl, self.mpl)
                ),
                (
                    Property::Alcohol, 
                    Modifier::new(self.a, self.ma)
                ),
            ].into_iter().collect()
        }
    }
}