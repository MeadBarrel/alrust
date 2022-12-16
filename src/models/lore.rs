use diesel::{prelude::*, associations::HasTable};
use super::Conn;
use crate::schema::*;
use grimoire::data;
use grimoire::theoretical::*;


#[derive(Debug, Clone, Queryable, Insertable, AsChangeset, Identifiable)]
#[diesel(table_name=lores, primary_key(name))]
pub struct Lore {
    pub name: String,
    pub effectiveness: Option<f64>,
    pub parent: Option<String>,
    pub parent2: Option<String>
}


impl Lore {
    pub fn load(conn: &mut Conn) -> QueryResult<Vec<Lore>> {
        Lore::table().load(conn)
    }

    pub fn from_grimoire(src: &data::Lore) -> Self {
        Self {
            name: src.name.clone(),
            effectiveness: src.effectiveness.into(),
            parent: src.parent_name.clone(),
            parent2: src.parent_2_name.clone(),
        }
    }    

    pub fn to_grimoire(&self) -> data::Lore {
        data::Lore {
            name: self.name.clone(),
            effectiveness: match self.effectiveness {
                Some(x) => Theoretical::Known(x),
                None => Theoretical::Unknown(0.66666),
            },
            parent_name: self.parent.clone(),
            parent_2_name: self.parent2.clone(),
        }
    }
}