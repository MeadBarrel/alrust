use diesel::{sqlite::SqliteConnection, QueryResult, RunQueryDsl, sql_query};
use geneticalchemy::prelude::Compendium;

pub type Conn = SqliteConnection;

pub mod ingredient;
pub mod lore;
pub mod player_character;
pub mod player_character_lore;


pub fn write_compendium(connection: &mut Conn, grimoire: &Compendium) -> QueryResult<()> {
    use crate::schema::*;
    use diesel::{delete, insert_into};

    //connection.execute("PRAGMA foreign_keys = off")?;
    sql_query("PRAGMA foreign_keys = off;").execute(connection);

    delete(ingredients::table).execute(connection)?;
    delete(player_character_lores::table).execute(connection)?;
    delete(lores::table).execute(connection)?;
    delete(player_characters::table).execute(connection)?;

    let ingredients_to_insert: Vec<ingredient::Ingredient> = grimoire.ingredients.values().map(
        ingredient::Ingredient::from_grimoire
    ).collect();

    let lores_to_insert: Vec<lore::Lore> = grimoire.lores.values().map(
        lore::Lore::from_grimoire
    ).collect();

    let characters_to_insert: Vec<player_character::PlayerCharacter> = grimoire.characters.values()
        .map(
            player_character::PlayerCharacter::from_grimoire
        ).collect();

    let character_lores_to_insert: Vec<player_character_lore::PlayerCharacterLore> = 
        grimoire.characters.values().flat_map(
            player_character::PlayerCharacter::lores_from_grimoire
        ).collect();

    insert_into(ingredients::table).values(ingredients_to_insert).execute(connection)?;
    insert_into(lores::table).values(lores_to_insert).execute(connection)?;
    insert_into(player_characters::table).values(characters_to_insert).execute(connection)?;
    insert_into(player_character_lores::table)
        .values(character_lores_to_insert).execute(connection)?;

    sql_query("PRAGMA foreign_keys = on").execute(connection);
    Ok(())
}