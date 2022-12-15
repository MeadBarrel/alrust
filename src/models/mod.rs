use diesel::sqlite::SqliteConnection;

pub type Connection = SqliteConnection;

pub mod ingredient;
pub mod lore;
pub mod player_character;
pub mod player_character_lore;