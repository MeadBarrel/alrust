use diesel::{prelude::*, associations::HasTable};
use super::Connection;
use crate::schema::*;
use grimoire::data;


#[derive(Debug, Clone, Queryable, Insertable, AsChangeset, Identifiable)]
#[diesel(table_name=lores, primary_key(name))]
pub struct Lore {
    pub name: String,
    pub effectiveness: Option<f64>,
    pub parent: Option<String>,
}


impl Lore {
    pub fn load(conn: &mut Connection) -> QueryResult<Vec<Lore>> {
        Lore::table().load(conn)
    }

    pub fn from_grimoire(src: &data::Lore) -> Self {
        Self {
            name: src.name.clone(),
            effectiveness: src.effectiveness,
            parent: src.parent_name.clone(),
        }
    }    

    pub fn to_grimoire(&self) -> data::Lore {
        data::Lore {
            name: self.name.clone(),
            effectiveness: self.effectiveness,
            parent_name: self.parent.clone()
        }
    }
}