pub mod error;

mod schema;
mod ingredient;
mod lore;
mod player_character;
mod player_character_lore;

use std::collections::HashMap;
use diesel::{sql_query, sqlite::SqliteConnection, QueryResult, RunQueryDsl, Connection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use grimoire2::prelude::Grimoire;
use grimoire2::prelude as g2;

use crate::{
    ingredient::Ingredient,
    lore::Lore,
    player_character::PlayerCharacter,
    player_character_lore::PlayerCharacterLore
};

use error::*;

pub type Conn = SqliteConnection;


pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../migrations");


pub struct GrimoireSqlite {
    connection: Conn,
}


impl GrimoireSqlite {
    pub fn connect(url: &str) -> Result<Self> {
        let connection = SqliteConnection::establish(url)?;
        Ok(Self { connection })
    }

    pub fn migrate(&mut self) -> Result<()> {
        match self.connection.run_pending_migrations(MIGRATIONS) {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::MigrationFailed)
        }
    }

    pub fn load(&mut self) -> Result<Grimoire> {
        self.migrate()?;
        load(&mut self.connection)
    }

    pub fn write(&mut self, grimoire: &Grimoire) -> Result<()> {
        self.migrate()?;
        match write(&mut self.connection, grimoire) {
            Ok(()) => Ok(()),
            Err(err) => Err(Error::QueryFailed { source: err })
        }
    }
}


fn load(conn: &mut Conn) -> Result<Grimoire> {
    let ingredients_db = Ingredient::load(conn)?;
    let lores_db = Lore::load(conn)?;
    let player_characters_db = PlayerCharacter::load(conn)?;

    let player_character_lores_db = PlayerCharacterLore::load(conn)?;

    let ingredients: HashMap<String, g2::Ingredient> =
        ingredients_db.iter().map(|x| x.to_grimoire()).collect();

    let skills: HashMap<String, g2::Skill> = lores_db.iter().map(|x| x.to_grimoire()).collect();

    let characters: HashMap<String, g2::Character> = player_characters_db
        .iter()
        .map(|x| x.to_grimoire(&player_character_lores_db))
        .collect();

    let grimoire = Grimoire::new(skills, ingredients, characters);

    Ok(grimoire)
}


fn write(connection: &mut Conn, grimoire: &Grimoire) -> QueryResult<()> {
    use crate::schema::*;
    use diesel::{delete, insert_into};

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
