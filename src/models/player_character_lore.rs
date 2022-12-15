use diesel::{prelude::*, associations::HasTable};
use super::Connection;
use super::player_character::PlayerCharacter;
use crate::schema::*;


#[derive(Debug, Clone, Queryable, Insertable, AsChangeset, Identifiable, Associations)]
#[diesel(table_name=player_character_lores, primary_key(character,lore), belongs_to(PlayerCharacter, foreign_key=character))]
pub struct PlayerCharacterLore {
    pub character: String,
    pub lore: String,
    pub value: i32,
}


impl PlayerCharacterLore {
    pub fn load(conn: &mut Connection) -> QueryResult<Vec<PlayerCharacterLore>> {
        PlayerCharacterLore::table().load(conn)
    }

    pub fn load_for_character(conn: &mut Connection, character_name: &str) -> QueryResult<Vec<PlayerCharacterLore>> {
        use crate::schema::player_character_lores::dsl::*;

        player_character_lores.filter(character.eq(character_name)).load(conn)
    }

}