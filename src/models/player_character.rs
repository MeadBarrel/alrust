use diesel::{prelude::*, associations::HasTable};
use super::Conn;
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
    pub fn load(conn: &mut Conn) -> QueryResult<Vec<PlayerCharacter>> {
        PlayerCharacter::table().load(conn)
    }

    pub fn from_grimoire(src: &data::Character) -> Self {
        Self {
            name: src.name.clone(),
            advanced_potion_making: src.advanced_potion_making as i32,
            alvarin_clade: src.alvarin_clade,
        }
    }

    pub fn lores_from_grimoire(src: &data::Character) -> Vec<PlayerCharacterLore> {
        src.lore_values.iter().map(
            |(name, &value)| PlayerCharacterLore {
                character: src.name.clone(),
                lore: name.clone(),
                value: value.into()
            }
        ).collect()
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
