use diesel::{sql_query, sqlite::SqliteConnection, QueryResult, RunQueryDsl};
use grimoire2::prelude::Grimoire;

pub type Conn = SqliteConnection;

pub mod ingredient;
pub mod lore;
pub mod player_character;
pub mod player_character_lore;

pub fn write_compendium(connection: &mut Conn, grimoire: &Grimoire) -> QueryResult<()> {
    use crate::schema::*;
    use diesel::{delete, insert_into};

    //connection.execute("PRAGMA foreign_keys = off")?;
    sql_query("PRAGMA foreign_keys = off;").execute(connection)?;

    delete(ingredients::table).execute(connection)?;
    delete(player_character_lores::table).execute(connection)?;
    delete(lores::table).execute(connection)?;
    delete(player_characters::table).execute(connection)?;

    let ingredients_to_insert: Vec<ingredient::Ingredient> = grimoire
        .ingredients
        .iter()
        .map(|(name, src)| ingredient::Ingredient::from_grimoire(name, src))
        .collect();

    let lores_to_insert: Vec<lore::Lore> = grimoire
        .skills
        .iter()
        .map(|(name, src)| lore::Lore::from_grimoire(name, src))
        .collect();

    let characters_to_insert: Vec<player_character::PlayerCharacter> = grimoire
        .characters
        .iter()
        .map(|(name, src)| player_character::PlayerCharacter::from_grimoire(name, src))
        .collect();

    let character_lores_to_insert: Vec<player_character_lore::PlayerCharacterLore> = grimoire
        .characters
        .iter()
        .flat_map(|(name, src)| player_character::PlayerCharacter::lores_from_grimoire(name, src))
        .collect();

    insert_into(ingredients::table)
        .values(ingredients_to_insert)
        .execute(connection)?;
    insert_into(lores::table)
        .values(lores_to_insert)
        .execute(connection)?;
    insert_into(player_characters::table)
        .values(characters_to_insert)
        .execute(connection)?;
    insert_into(player_character_lores::table)
        .values(character_lores_to_insert)
        .execute(connection)?;

    sql_query("PRAGMA foreign_keys = on").execute(connection)?;
    Ok(())
}
