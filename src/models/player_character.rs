use diesel::{prelude::*, associations::HasTable};
use super::Connection;
use crate::schema::*;
use super::player_character_lore::PlayerCharacterLore;
use grimoire::data;


#[derive(Debug, Clone, Queryable, Insertable, AsChangeset, Identifiable)]
#[diesel(table_name=player_characters, primary_key(name))]
pub struct PlayerCharacter {
    pub name: String,
    pub advanced_potion_making: i32,
    pub alvarin_clade: bool,
    
}


impl PlayerCharacter {
    pub fn load(conn: &mut Connection) -> QueryResult<Vec<PlayerCharacter>> {
        PlayerCharacter::table().load(conn)
    }

    pub fn to_grimoire(&self, lores: &Vec<PlayerCharacterLore>) -> data::Character {
        data::Character {
            name: self.name.clone(),
            advanced_potion_making: self.advanced_potion_making as u8,
            alvarin_clade: self.alvarin_clade,
            lore_values: lores.into_iter().filter(|x| x.character == self.name)
                .map(|x| (x.lore.clone(), x.value as u8)).collect()
        }
    }
}
