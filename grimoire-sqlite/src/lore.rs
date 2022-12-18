use super::Conn;
use crate::schema::*;
use diesel::{associations::HasTable, prelude::*};

use grimoire2::{prelude as g2, prelude::Theoretical};

#[derive(Debug, Clone, Queryable, Insertable, AsChangeset, Identifiable)]
#[diesel(table_name=lores, primary_key(name))]
pub struct Lore {
    pub name: String,
    pub effectiveness: Option<f64>,
    pub parent: Option<String>,
    pub parent2: Option<String>,
}

impl Lore {
    pub fn load(conn: &mut Conn) -> QueryResult<Vec<Lore>> {
        Lore::table().load(conn)
    }

    pub fn from_grimoire(name: &str, src: &g2::Skill) -> Self {
        Self {
            name: name.to_string(),
            effectiveness: src.effectiveness.into(),
            parent: src.parent.clone(),
            parent2: src.parent_2.clone(),
        }
    }

    pub fn to_grimoire(&self) -> (String, g2::Skill) {
        let skill = g2::Skill {
            effectiveness: match self.effectiveness {
                Some(x) => Theoretical::Known(x),
                None => Theoretical::Theory(0.66666),
            },
            parent: self.parent.clone(),
            parent_2: self.parent2.clone(),
        };

        (self.name.clone(), skill)
    }
}
