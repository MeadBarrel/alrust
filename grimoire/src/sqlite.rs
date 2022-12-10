use std::collections::HashMap;
use rusqlite::{Connection, Result};
use crate::{data::*, types::{Property, Modifier}};


pub fn load_from_db(path: &str) -> Result<Compendium> {
    let connection = Connection::open(path)?;
    let characters = load_characters(&connection)?;
    let lores = load_lores(&connection)?;
    let ingredients = load_ingredients(&connection)?;

    Ok(Compendium::create_from_vecs(characters, lores, ingredients))
}


fn load_characters(connection: &Connection) -> Result<Vec<Character>> {
    let mut characters_stmt = connection.prepare(
        "SELECT name, advanced_potion_making, alvarin_clade from PlayerCharacters"
    )?;

    let mut lores_stmt = connection.prepare("
        SELECT lore, value FROM PlayerCharacterLores WHERE PlayerCharacterLores.character = ?1
    ")?;

    let result: Result<Vec<Character>> = characters_stmt.query_map([], |row| {
        let character_name: String = row.get("name")?;
        Ok(
            Character {
                name: character_name.clone(),
                advanced_potion_making: row.get("advanced_potion_making")?,
                alvarin_clade: row.get("alvarin_clade")?,
                lore_values: lores_stmt.query_map([character_name.clone()], |r| {
                    Ok(
                        (
                            r.get::<&str, String>("lore")?, 
                            r.get::<&str, u8>("value")?
                        )
                    )
                })?.collect::<Result<HashMap<String, u8>>>()?,
            }
        )
    })?.collect();

    result
}


fn load_lores(connection: &Connection) -> Result<Vec<Lore>> {
    let mut stmt = connection.prepare("SELECT name, effectiveness, parent FROM Lore")?;
    
    let result: Result<Vec<Lore>> = stmt.query_map([], |row| {
        Ok(
            Lore {
                name: row.get("name")?,
                effectiveness: row.get("effectiveness")?,
                parent_name: row.get("parent")?
            }
        )
    })?.collect();

    result
}


fn load_ingredients(connection: &Connection) -> Result<Vec<Ingredient>> {
    let mut stmt = connection.prepare("
        SELECT
        Ingredient.name as name,
        Ingredient.lore as lore,
        Ingredient.al_weight as al_weight,
        Coalesce(Ingredient.dh, 0) as dh,
        Coalesce(Ingredient.dp, 0) as dp,
        Coalesce(Ingredient.mdh, 0) as mdh,
        Coalesce(Ingredient.mdp, 0) as mdp,
        Coalesce(Ingredient.hot, 0) as hot,
        Coalesce(Ingredient.pot, 0) as pot,
        Coalesce(Ingredient.mhot, 0) as mhot,
        Coalesce(Ingredient.mpot, 0) as mpot,
        Coalesce(Ingredient.hl, 0) as hl,
        Coalesce(Ingredient.pl, 0) as pl,
        Coalesce(Ingredient.mhl, 0) as mhl,
        Coalesce(Ingredient.mpl, 0) as mpl,
        Coalesce(Ingredient.a, 0) as a,
        Coalesce(Ingredient.ma, 0) as ma
        FROM Ingredient        
    ")?;

    let result: Result<Vec<Ingredient>> = stmt.query_map([], |row|
        {
            Ok(
                Ingredient {
                    name: row.get("name")?,
                    alchemical_weight: row.get("al_weight")?,
                    lore_name: row.get("lore")?,
                    modifiers: vec![
                        (Property::DirectHealing, Modifier {modifier: row.get("dh")?, multiplier: row.get("mdh")?} ),
                        (Property::DirectPoison, Modifier {modifier: row.get("dp")? , multiplier: row.get("mdp")?} ),
                        (Property::HealingOverTime, Modifier {modifier: row.get("hot")?, multiplier: row.get("mhot")? } ),
                        (Property::PoisonOverTime, Modifier {modifier: row.get("pot")?, multiplier: row.get("mpot")?} ),
                        (Property::HealingLength, Modifier {modifier: row.get("hl")?, multiplier: row.get("mhl")?} ),
                        (Property::PoisonLength, Modifier {modifier: row.get("pl")?, multiplier: row.get("mpl")? } ),
                        (Property::Alcohol, Modifier {modifier: row.get("a")?, multiplier: row.get("ma")? } )
                    ]
                }
            )
        }
    )?.collect();

    result
}